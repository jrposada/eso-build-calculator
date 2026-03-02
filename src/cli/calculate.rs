use super::build_config::BuildConfig;
use super::parsers::{parse_champion_point, parse_set, parse_skill};
use crate::data::bonuses::{TRIAL_BUFF_NAMES, TRIAL_DUMMY_BUFFS};
use crate::data::passives::armor::armor_passives;
use crate::data::passives::undaunted_mettle_bonuses;
use crate::domain::{
    ArmorTrait, ArmorWeight, AttributeChoice, BonusData, Build, CharacterStats, Food, GearConfig,
    JewelryTrait, MundusStone, Potion, Race, SetData, SetProcEffect, SkillData, SkillLineName,
    WeaponEnchant, WeaponTrait, WeaponType, BUILD_CONSTRAINTS,
};
use super::simulation_display::display_simulation_result;
use crate::infrastructure::{format, logger, table, table::ColumnDefinition};
use crate::services::{
    generate_distributions, infer_weapons, FightSimulator, PassivesService, PassivesServiceOptions,
};
use clap::Args;
use std::collections::HashSet;
use std::fs;
use std::path::PathBuf;

/// Calculate total damage for a specific build configuration
#[derive(Args, Debug)]
pub struct CalculateArgs {
    /// 10 skills (comma-separated skill names)
    #[arg(short = 's', long, value_delimiter = ',', value_parser = parse_skill, conflicts_with = "file")]
    pub skills: Option<Vec<&'static SkillData>>,

    /// 4 champion points (comma-separated)
    #[arg(long = "cp", value_delimiter = ',', value_parser = parse_champion_point, conflicts_with = "file")]
    pub champion_points: Option<Vec<BonusData>>,

    /// Path to build configuration file (exported from optimize)
    #[arg(short = 'f', long, conflicts_with_all = ["skills", "champion_points", "sets", "monster_sets", "mythic", "magicka", "stamina", "bar1_weapon", "bar2_weapon"])]
    pub file: Option<PathBuf>,

    /// Allocate 64 attribute points to magicka
    #[arg(long, conflicts_with = "stamina")]
    pub magicka: bool,

    /// Allocate 64 attribute points to stamina
    #[arg(long, conflicts_with = "magicka")]
    pub stamina: bool,

    /// Bar 1 weapon type (e.g., inferno-staff, bow)
    #[arg(long, value_parser = WeaponType::parse)]
    pub bar1_weapon: Option<WeaponType>,

    /// Bar 2 weapon type (e.g., inferno-staff, bow)
    #[arg(long, value_parser = WeaponType::parse)]
    pub bar2_weapon: Option<WeaponType>,

    /// Normal 5pc sets (comma-separated, max 2)
    #[arg(long, value_delimiter = ',', value_parser = parse_set, conflicts_with = "file")]
    pub sets: Option<Vec<&'static SetData>>,

    /// Monster sets (comma-separated, max 2)
    #[arg(long, value_delimiter = ',', value_parser = parse_set, conflicts_with = "file")]
    pub monster_sets: Option<Vec<&'static SetData>>,

    /// Mythic item (max 1)
    #[arg(long, value_parser = parse_set, conflicts_with = "file")]
    pub mythic: Option<&'static SetData>,

    /// Show extra details (buffed character stats)
    #[arg(short = 'v', long)]
    pub verbose: bool,

    /// Character race (dark-elf, khajiit, orc, etc.)
    #[arg(long, value_parser = Race::parse, conflicts_with = "file")]
    pub race: Option<Race>,

    /// Mundus stone (thief, shadow, warrior, etc.)
    #[arg(long, value_parser = MundusStone::parse, conflicts_with = "file")]
    pub mundus: Option<MundusStone>,

    /// Food buff (lava-foot, ghastly-eye, sugar-skulls)
    #[arg(long, value_parser = Food::parse, conflicts_with = "file")]
    pub food: Option<Food>,

    /// Armor trait for all 7 pieces (defaults to divines if omitted)
    #[arg(long, value_parser = ArmorTrait::parse)]
    pub armor_trait: Option<ArmorTrait>,

    /// Jewelry trait for all 3 pieces (defaults to bloodthirsty if omitted)
    #[arg(long, value_parser = JewelryTrait::parse)]
    pub jewelry_trait: Option<JewelryTrait>,

    /// Weapon trait (defaults to nirnhoned if omitted)
    #[arg(long, value_parser = WeaponTrait::parse)]
    pub weapon_trait: Option<WeaponTrait>,

    /// Armor weight for armor passives (medium, light, heavy; defaults to medium)
    #[arg(long, value_parser = ArmorWeight::parse)]
    pub armor_weight: Option<ArmorWeight>,

    /// Potion buff (weapon-power, spell-power; defaults to weapon-power)
    #[arg(long, value_parser = Potion::parse)]
    pub potion: Option<Potion>,

    /// Bar 1 weapon enchant (flame, poison, shock, berserker; defaults to flame)
    #[arg(long, value_parser = WeaponEnchant::parse)]
    pub bar1_enchant: Option<WeaponEnchant>,

    /// Bar 2 weapon enchant (flame, poison, shock, berserker; defaults to flame)
    #[arg(long, value_parser = WeaponEnchant::parse)]
    pub bar2_enchant: Option<WeaponEnchant>,

    /// Number of distinct armor weights worn (1-3, defaults to 3 for 5/1/1 builds)
    #[arg(long, default_value = "3")]
    pub armor_types: u8,

    /// Override computed max stamina
    #[arg(long, conflicts_with = "file")]
    pub max_stamina: Option<f64>,

    /// Override computed max magicka
    #[arg(long, conflicts_with = "file")]
    pub max_magicka: Option<f64>,

    /// Override computed weapon damage
    #[arg(long, conflicts_with = "file")]
    pub weapon_damage: Option<f64>,

    /// Override computed spell damage
    #[arg(long, conflicts_with = "file")]
    pub spell_damage: Option<f64>,

    /// Override computed critical rating
    #[arg(long, conflicts_with = "file")]
    pub critical_rating: Option<f64>,

    /// Override computed penetration
    #[arg(long, conflicts_with = "file")]
    pub penetration: Option<f64>,

    /// Disable trial dummy buffs/debuffs (enabled by default)
    #[arg(long = "no-trial")]
    pub no_trial: bool,
}

impl CalculateArgs {
    pub fn run(&self) {
        let (skills, champion_points, file_weapons, file_sets, stats, file_trial) =
            if let Some(path) = &self.file {
                let (s, cp, w, sets, character_stats, trial) = self.load_from_file(path);
                (s, cp, w, sets, character_stats, Some(trial))
            } else {
                let (s, cp) = self.get_from_args();

                let attributes = if self.magicka {
                    AttributeChoice::Magicka
                } else if self.stamina {
                    AttributeChoice::Stamina
                } else {
                    AttributeChoice::None
                };

                let gear = GearConfig {
                    race: self.race,
                    mundus: self.mundus,
                    food: self.food,
                    armor_trait: self.armor_trait.unwrap_or(ArmorTrait::Divines),
                    jewelry_trait: self.jewelry_trait.unwrap_or(JewelryTrait::Bloodthirsty),
                    weapon_trait: self.weapon_trait.unwrap_or(WeaponTrait::Nirnhoned),
                    attributes,
                    armor_weight: self.armor_weight.unwrap_or(ArmorWeight::Medium),
                };
                let mut stats = gear.compute_stats(self.bar1_weapon);

                // Apply stat overrides
                if let Some(v) = self.max_stamina { stats.max_stamina = v; }
                if let Some(v) = self.max_magicka { stats.max_magicka = v; }
                if let Some(v) = self.weapon_damage { stats.weapon_damage = v; }
                if let Some(v) = self.spell_damage { stats.spell_damage = v; }
                if let Some(v) = self.critical_rating { stats.critical_rating = v; }
                if let Some(v) = self.penetration { stats.penetration = v; }

                (s, cp, (None, None), Vec::new(), stats, None)
            };

        // Trial buffs: --no-trial overrides file config; file config defaults to true
        let use_trial = if self.no_trial {
            false
        } else {
            file_trial.unwrap_or(true)
        };
        let extra_bonuses: Vec<BonusData> = if use_trial {
            TRIAL_DUMMY_BUFFS.clone()
        } else {
            Vec::new()
        };

        // Validate skill count
        if skills.len() != BUILD_CONSTRAINTS.skill_count {
            logger::error(&format!(
                "Exactly {} skills required, got {}",
                BUILD_CONSTRAINTS.skill_count,
                skills.len()
            ));
            std::process::exit(1);
        }

        // Validate champion point count
        if champion_points.len() != BUILD_CONSTRAINTS.champion_point_count {
            logger::error(&format!(
                "Exactly {} champion points required, got {}",
                BUILD_CONSTRAINTS.champion_point_count,
                champion_points.len()
            ));
            std::process::exit(1);
        }

        logger::info("Calculating build damage...");

        // Collect unique skill lines from the selected skills
        let skill_lines: HashSet<SkillLineName> = skills.iter().map(|s| s.skill_line).collect();

        // Get passives from the skill lines
        let passives_service = PassivesService::new(PassivesServiceOptions::default());
        let mut passive_bonuses: Vec<BonusData> = skill_lines
            .iter()
            .flat_map(|sl| passives_service.get_passives_by_skill_line(*sl))
            .flat_map(|p| p.bonuses.iter().cloned())
            .collect();

        // Add armor passives based on armor weight
        let armor_weight = self.armor_weight.unwrap_or(ArmorWeight::Medium);
        passive_bonuses.extend(armor_passives(armor_weight));

        // Add Undaunted Mettle bonuses based on armor types worn
        passive_bonuses.extend(undaunted_mettle_bonuses(self.armor_types));

        // Add potion bonuses (default: weapon-power)
        let potion = self.potion.unwrap_or(Potion::WeaponPower);
        passive_bonuses.extend(potion.bonuses());

        // Collect set bonuses
        let active_sets: Vec<&'static SetData> = if file_sets.is_empty() {
            let mut sets: Vec<&'static SetData> = Vec::new();
            if let Some(s) = &self.sets {
                sets.extend(s.iter());
            }
            if let Some(m) = &self.monster_sets {
                sets.extend(m.iter());
            }
            if let Some(m) = &self.mythic {
                sets.push(m);
            }
            sets
        } else {
            file_sets
        };

        let mut set_bonuses: Vec<BonusData> = Vec::new();
        let mut set_names: Vec<(String, u8)> = Vec::new();
        let mut set_proc_effects: Vec<SetProcEffect> = Vec::new();
        for set in &active_sets {
            let piece_count = set.set_type.max_pieces();
            let bonuses = set.bonuses_at(piece_count);
            set_bonuses.extend(bonuses.into_iter().cloned());
            set_proc_effects.extend(
                set.proc_effects_at(piece_count)
                    .into_iter()
                    .cloned(),
            );
            set_names.push((set.name.clone(), piece_count));
        }

        // Create the build (for stats resolution and display)
        let build = Build::new_with_extra(
            skills.clone(),
            &champion_points,
            &passive_bonuses,
            &set_bonuses,
            set_names,
            stats,
            &extra_bonuses,
        );

        if !skills.iter().any(|s| s.spammable) {
            logger::warn("This build has no spammable skill. Every rotation needs at least one instant-cast filler.");
        }

        let pure_spammable_count = skills
            .iter()
            .filter(|s| s.spammable && s.bonuses.is_none())
            .count();
        if pure_spammable_count > 1 {
            logger::warn(&format!(
                "This build has {} pure spammable skills (spammable without bonuses). Only the highest-damage one would be used as filler; the rest waste a slot.",
                pure_spammable_count
            ));
        }

        // Display the build summary (existing behavior)
        logger::info(&build.to_string());

        // --- Fight simulation ---
        // Determine weapon types
        let bar1_weapon = self
            .bar1_weapon
            .or(file_weapons.0);
        let bar2_weapon = self
            .bar2_weapon
            .or(file_weapons.1);

        let (bar1_weapon, bar2_weapon) = match (bar1_weapon, bar2_weapon) {
            (Some(w1), Some(w2)) => (w1, w2),
            (Some(w1), None) => (w1, w1),
            (None, Some(w2)) => (w2, w2),
            (None, None) => match infer_weapons(&skills) {
                Ok(weapons) => weapons,
                Err(e) => {
                    logger::warn(&format!(
                        "Could not infer weapons for simulation: {}. Skipping fight simulation.",
                        e
                    ));
                    return;
                }
            },
        };

        logger::info(&format!(
            "Running fight simulation (Bar1: {}, Bar2: {})...",
            bar1_weapon, bar2_weapon
        ));

        let distributions = generate_distributions(&skills, bar1_weapon, bar2_weapon);

        // Use the build's effective stats and resolved bonuses for the simulator
        let effective_stats = build.effective_stats();
        let resolved_bonuses = build.resolved_bonuses();

        let mut suppressed = if use_trial {
            TRIAL_BUFF_NAMES.clone()
        } else {
            HashSet::new()
        };
        // Suppress potion buff names to prevent double-counting with skill buffs
        for bonus in potion.bonuses() {
            suppressed.insert(bonus.name.clone());
        }
        let bar1_enchant = self.bar1_enchant.or(Some(WeaponEnchant::Flame));
        let bar2_enchant = self.bar2_enchant.or(Some(WeaponEnchant::Flame));
        let simulator = FightSimulator::new(effective_stats, resolved_bonuses, suppressed)
            .with_enchants(bar1_enchant, bar2_enchant)
            .with_set_procs(set_proc_effects);

        if self.verbose {
            let buffed = simulator.compute_buffed_stats(&distributions[0]);
            logger::trace(&format_buffed_stats(&buffed));
        }

        let mut results: Vec<(usize, crate::domain::SimulationResult)> = distributions
            .iter()
            .enumerate()
            .map(|(i, dist)| (i, simulator.simulate(dist)))
            .collect();

        results.sort_by(|a, b| b.1.dps.partial_cmp(&a.1.dps).unwrap());

        if let Some((best_idx, best_result)) = results.first() {
            let best_dist = &distributions[*best_idx];
            display_simulation_result(best_result, best_dist, distributions.len(), build.set_names());
        }
    }

    fn get_from_args(&self) -> (Vec<&'static SkillData>, Vec<BonusData>) {
        let skills = self.skills.clone().unwrap_or_else(|| {
            logger::error("Either --skills/-s or --file/-f must be provided");
            std::process::exit(1);
        });

        let champion_points = self.champion_points.clone().unwrap_or_else(|| {
            logger::error("Either --champion-points/-p or --file/-f must be provided");
            std::process::exit(1);
        });

        (skills, champion_points)
    }

    fn load_from_file(
        &self,
        path: &PathBuf,
    ) -> (
        Vec<&'static SkillData>,
        Vec<BonusData>,
        (Option<WeaponType>, Option<WeaponType>),
        Vec<&'static SetData>,
        CharacterStats,
        bool,
    ) {
        let content = fs::read_to_string(path).unwrap_or_else(|e| {
            logger::error(&format!("Failed to read file '{}': {}", path.display(), e));
            std::process::exit(1);
        });

        let config: BuildConfig = serde_json::from_str(&content).unwrap_or_else(|e| {
            logger::error(&format!("Failed to parse build config: {}", e));
            std::process::exit(1);
        });

        let skills: Vec<&'static SkillData> = config
            .skills
            .iter()
            .map(|name| {
                parse_skill(name).unwrap_or_else(|e| {
                    logger::error(&e);
                    std::process::exit(1);
                })
            })
            .collect();

        let champion_points: Vec<BonusData> = config
            .champion_points
            .iter()
            .map(|name| {
                parse_champion_point(name).unwrap_or_else(|e| {
                    logger::error(&e);
                    std::process::exit(1);
                })
            })
            .collect();

        let bar1 = config
            .bar1_weapon
            .as_ref()
            .and_then(|w| WeaponType::parse(w).ok());
        let bar2 = config
            .bar2_weapon
            .as_ref()
            .and_then(|w| WeaponType::parse(w).ok());

        let sets: Vec<&'static SetData> = config
            .sets
            .iter()
            .map(|name| {
                parse_set(name).unwrap_or_else(|e| {
                    logger::error(&e);
                    std::process::exit(1);
                })
            })
            .collect();

        (skills, champion_points, (bar1, bar2), sets, config.character_stats, config.trial)
    }
}

fn format_buffed_stats(stats: &CharacterStats) -> String {
    let fmt_stat = |val: f64| format::format_number(val as u64);
    let fmt_pct = |val: f64| format!("{:.2}%", val * 100.0);
    let fmt_crit_dmg = |val: f64| format!("{:.2}%", (val - 1.0) * 100.0);

    let data: Vec<Vec<String>> = vec![
        vec!["Max Magicka".into(), fmt_stat(stats.max_magicka)],
        vec!["Max Stamina".into(), fmt_stat(stats.max_stamina)],
        vec!["Weapon Damage".into(), fmt_stat(stats.weapon_damage)],
        vec!["Spell Damage".into(), fmt_stat(stats.spell_damage)],
        vec!["Critical Chance".into(), fmt_pct(stats.critical_chance())],
        vec!["Critical Damage".into(), fmt_crit_dmg(stats.critical_damage)],
        vec!["Penetration".into(), fmt_stat(stats.penetration)],
        vec!["Target Armor".into(), fmt_stat(stats.target_armor)],
    ];

    table(
        &data,
        table::TableOptions {
            title: Some("Buffed Character Stats".into()),
            columns: vec![
                ColumnDefinition::new("Stat", 20),
                ColumnDefinition::new("Value", 12).align_right(),
            ],
            footer: None,
        },
    )
}
