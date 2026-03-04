use super::parsers::{parse_class_name, parse_weapon};
use crate::domain::{
    ArmorDistribution, ArmorTrait, AttributeChoice, BonusData, BuildConfig, ClassName, Food,
    JewelryTrait, MundusStone, Potion, Race, SetData, SkillData, WeaponEnchant,
    WeaponTrait, WeaponType, BUILD_CONSTRAINTS,
};
use crate::infrastructure::logger;
use crate::services::{OptimizePipeline, OptimizePipelineOptions};
use clap::Args;
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;

/// Optimize build command arguments
#[derive(Args, Debug)]
pub struct OptimizeArgs {
    /// Show optimization progress
    #[arg(short = 'v', long)]
    pub verbose: bool,

    /// Pin character race (dark-elf, khajiit, orc, etc.)
    #[arg(long, value_parser = Race::parse)]
    pub race: Option<Race>,

    /// Pin at least 1 skill line from these class (comma-separated)
    #[arg(long, value_delimiter = ',', value_parser = parse_class_name)]
    pub class: Option<Vec<ClassName>>,

    /// Restrict build to only the classes specified by --class
    #[arg(long, requires = "class")]
    pub pure: bool,

    /// Pin attribute points to magicka (optimized if omitted)
    #[arg(long, conflicts_with = "stamina")]
    pub magicka: bool,

    /// Pin attribute points to stamina (optimized if omitted)
    #[arg(long, conflicts_with = "magicka")]
    pub stamina: bool,

    /// Pin gear sets (comma-separated). Auto-grouped by type: max 2 normal/arena, 2 monster, 1 mythic.
    #[arg(long, value_delimiter = ',', value_parser = SetData::parse)]
    pub set: Option<Vec<&'static SetData>>,

    /// Pin weapon (comma-separated). Accepts skill lines (bow, destruction-staff, dual-wield,
    /// two-handed) or specific types (inferno-staff, lightning-staff, dual-wield-dagger, etc.).
    /// First value = bar1, second = bar2. One value = bar1 only (bar2 optimized).
    #[arg(long, value_delimiter = ',', value_parser = parse_weapon)]
    pub weapon: Option<Vec<WeaponType>>,

    /// Require these skills in every build (comma-separated skill names)
    #[arg(long, value_delimiter = ',', value_parser = SkillData::parse)]
    pub skill: Option<Vec<&'static SkillData>>,

    /// Require these champion points (comma-separated)
    #[arg(long = "cp", value_delimiter = ',', value_parser = BonusData::parse_champion_point)]
    pub champion_point: Option<Vec<BonusData>>,

    /// Pin mundus stone (thief, shadow, warrior, etc.) - optimized if omitted
    #[arg(long, value_parser = MundusStone::parse)]
    pub mundus: Option<MundusStone>,

    /// Pin food buff (lava-foot, ghastly-eye, sugar-skulls) - optimized if omitted
    #[arg(long, value_parser = Food::parse)]
    pub food: Option<Food>,

    /// Pin armor traits per slot (comma-separated, max 7). Pins first N slots, rest optimized.
    #[arg(long, value_delimiter = ',', value_parser = ArmorTrait::parse)]
    pub armor_trait: Option<Vec<ArmorTrait>>,

    /// Pin jewelry traits per slot (comma-separated, max 3). Pins first N slots, rest optimized.
    #[arg(long, value_delimiter = ',', value_parser = JewelryTrait::parse)]
    pub jewelry_trait: Option<Vec<JewelryTrait>>,

    /// Pin weapon traits per slot (comma-separated, max 2). Bar1, then bar2. Rest optimized.
    #[arg(long, value_delimiter = ',', value_parser = WeaponTrait::parse)]
    pub weapon_trait: Option<Vec<WeaponTrait>>,

    /// Armor piece counts as light,medium,heavy (e.g. 1,5,1). Free slots optimized.
    #[arg(long, value_parser = ArmorDistribution::parse, default_value = "1,5,1")]
    pub armor: ArmorDistribution,

    /// Potion buff (weapon-power, spell-power; defaults to weapon-power)
    #[arg(long, value_parser = Potion::parse)]
    pub potion: Option<Potion>,

    /// Weapon enchants per bar (comma-separated: bar1,bar2). Unpinned bars optimized.
    #[arg(long, value_delimiter = ',', value_parser = WeaponEnchant::parse)]
    pub enchant: Option<Vec<WeaponEnchant>>,

    /// Average resource percentage for resource-scaling sets like Bahsei's (0-100, default 50)
    #[arg(long, default_value = "50")]
    pub avg_resource_pct: f64,

    /// Disable trial dummy buffs/debuffs (enabled by default)
    #[arg(long = "no-trial")]
    pub no_trial: bool,

    /// Export build to this file without prompting
    #[arg(short = 'o', long)]
    pub output: Option<PathBuf>,

    /// Number of parallel threads to use (default: half of available CPUs)
    #[arg(short = 'p', long)]
    pub parallelism: Option<u8>,

    /// Cap non-spammable skill pool per skill-line combo (prune lowest-damage skills)
    #[arg(long)]
    pub max_pool_size: Option<usize>,
}

impl OptimizeArgs {
    pub fn run(&self) {
        self.validate();
        let options = self.build_pipeline_options();
        let result = OptimizePipeline::run(options);

        let path = self.output.clone().or_else(Self::prompt_export);
        if let Some(path) = &path {
            Self::export_to_file(&result.build_config, path);
        }
    }

    fn validate(&self) {
        if let Some(classes) = &self.class {
            if classes.len() > BUILD_CONSTRAINTS.class_skill_line_count {
                logger::error(&format!(
                    "Maximum {} classes allowed",
                    BUILD_CONSTRAINTS.class_skill_line_count
                ));
                std::process::exit(1);
            }
        }

        if let Some(weapons) = &self.weapon {
            if weapons.len() > BUILD_CONSTRAINTS.weapon_skill_line_count {
                logger::error(&format!(
                    "Maximum {} weapons allowed",
                    BUILD_CONSTRAINTS.weapon_skill_line_count
                ));
                std::process::exit(1);
            }
        }

        if let Some(skills) = &self.skill {
            if skills.len() > BUILD_CONSTRAINTS.skill_count {
                logger::error(&format!(
                    "Maximum {} required skills allowed",
                    BUILD_CONSTRAINTS.skill_count
                ));
                std::process::exit(1);
            }
        }

        if let Some(cp) = &self.champion_point {
            if cp.len() > BUILD_CONSTRAINTS.champion_point_count {
                logger::error(&format!(
                    "Maximum {} champion points allowed",
                    BUILD_CONSTRAINTS.champion_point_count
                ));
                std::process::exit(1);
            }
        }

        if let Some(traits) = &self.armor_trait {
            if traits.len() > 7 {
                logger::error("Maximum 7 armor trait values allowed (one per piece)");
                std::process::exit(1);
            }
        }
        if let Some(traits) = &self.jewelry_trait {
            if traits.len() > 3 {
                logger::error("Maximum 3 jewelry trait values allowed (one per piece)");
                std::process::exit(1);
            }
        }
        if let Some(traits) = &self.weapon_trait {
            if traits.len() > 2 {
                logger::error("Maximum 2 weapon trait values allowed (bar1, bar2)");
                std::process::exit(1);
            }
        }

        if let Some(sets) = &self.set {
            let (normals, monsters, mythics) = SetData::split_by_type(sets);
            if normals.len() > 2 {
                logger::error("Maximum 2 normal/arena sets allowed");
                std::process::exit(1);
            }
            if monsters.len() > 2 {
                logger::error("Maximum 2 monster sets allowed");
                std::process::exit(1);
            }
            if mythics.len() > 1 {
                logger::error("Maximum 1 mythic set allowed");
                std::process::exit(1);
            }
        }
    }

    fn build_pipeline_options(&self) -> OptimizePipelineOptions {
        let parallelism = self
            .parallelism
            .unwrap_or_else(|| (num_cpus::get() / 2).max(1) as u8);

        let pinned_attributes = if self.magicka {
            Some(AttributeChoice::Magicka)
        } else if self.stamina {
            Some(AttributeChoice::Stamina)
        } else {
            None
        };

        // Build baseline BuildConfig
        let baseline_armor_traits = {
            let mut arr = [ArmorTrait::Divines; 7];
            if let Some(pinned) = &self.armor_trait {
                for (i, t) in pinned.iter().enumerate() {
                    arr[i] = *t;
                }
            }
            arr
        };
        let baseline_jewelry_traits = {
            let mut arr = [JewelryTrait::Bloodthirsty; 3];
            if let Some(pinned) = &self.jewelry_trait {
                for (i, t) in pinned.iter().enumerate() {
                    arr[i] = *t;
                }
            }
            arr
        };
        let baseline_weapon_traits = {
            let mut arr = [WeaponTrait::Nirnhoned; 2];
            if let Some(pinned) = &self.weapon_trait {
                for (i, t) in pinned.iter().enumerate() {
                    arr[i] = *t;
                }
            }
            arr
        };

        // Derive bar weapons from positional --weapon values
        let (bar1_weapon, bar2_weapon) = match self.weapon.as_deref() {
            Some([w1, w2, ..]) => (Some(*w1), Some(*w2)),
            Some([w1]) => (Some(*w1), None),
            _ => (None, None),
        };

        // Derive bar enchants from positional --enchant values
        let (pinned_bar1_enchant, pinned_bar2_enchant) = match self.enchant.as_deref() {
            Some([e1, e2, ..]) => (Some(*e1), Some(*e2)),
            Some([e1]) => (Some(*e1), None),
            _ => (None, None),
        };

        let baseline = BuildConfig {
            race: self.race,
            mundus: self.mundus,
            food: self.food,
            armor_traits: baseline_armor_traits,
            jewelry_traits: baseline_jewelry_traits,
            weapon_traits: baseline_weapon_traits,
            attributes: pinned_attributes.unwrap_or(AttributeChoice::Stamina),
            armor: self.armor,
            bar1_weapon,
            bar2_weapon,
            potion: self.potion,
            avg_resource_pct: self.avg_resource_pct,
            trial: !self.no_trial,
            ..BuildConfig::default()
        };

        OptimizePipelineOptions {
            verbose: self.verbose,
            pure: self.pure,
            required_class_names: self.class.clone().unwrap_or_default(),
            required_weapon_skill_lines: self
                .weapon
                .as_ref()
                .map(|ws| ws.iter().map(|w| w.skill_line()).collect())
                .unwrap_or_default(),
            required_champion_points: self.champion_point.clone().unwrap_or_default(),
            required_skills: self.skill.clone().unwrap_or_default(),
            parallelism,
            max_pool_size: self.max_pool_size,
            pinned_sets: self.set.clone().unwrap_or_default(),
            baseline,
            pinned_bar1_enchant,
            pinned_bar2_enchant,
            potion: self.potion.unwrap_or(Potion::WeaponPower),
            pinned_armor_traits: self.armor_trait.clone().unwrap_or_default(),
            pinned_jewelry_traits: self.jewelry_trait.clone().unwrap_or_default(),
            pinned_weapon_traits: self.weapon_trait.clone().unwrap_or_default(),
            pinned_race: self.race,
            pinned_mundus: self.mundus,
            pinned_food: self.food,
            pinned_attributes,
        }
    }

    fn prompt_export() -> Option<PathBuf> {
        print!("\nExport build to file? [path/no]: \x1b[90mn\x1b[0m");
        print!("\x1b[1D");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            return None;
        }

        let input = input.trim();
        if input.is_empty() || input.eq_ignore_ascii_case("no") || input.eq_ignore_ascii_case("n") {
            return None;
        }

        Some(PathBuf::from(input))
    }

    fn export_to_file(config: &crate::domain::BuildConfig, path: &PathBuf) {
        match serde_json::to_string_pretty(config) {
            Ok(json) => match fs::write(path, json) {
                Ok(_) => logger::info(&format!("Build exported to {}", path.display())),
                Err(e) => logger::error(&format!("Failed to write file: {}", e)),
            },
            Err(e) => logger::error(&format!("Failed to serialize build: {}", e)),
        }
    }
}

