use crate::data::{ClassName, SkillLineName};
use crate::domain::BUILD_CONSTRAINTS;
use crate::infrastructure::logger;
use crate::services::{BuildOptimizer, BuildOptimizerOptions};
use clap::Args;

/// Optimize build command arguments
#[derive(Args, Debug)]
pub struct OptimizeArgs {
    /// Require at least 1 skill line from these classes (comma-separated)
    #[arg(short = 'c', long, value_delimiter = ',', value_parser = parse_class_name)]
    pub classes: Option<Vec<ClassName>>,

    /// Require at least 1 skill line from these weapons (comma-separated)
    #[arg(short = 'w', long, value_delimiter = ',', value_parser = parse_weapon)]
    pub weapons: Option<Vec<SkillLineName>>,

    /// Force specific morph selections (comma-separated morph names)
    #[arg(short = 'm', long, value_delimiter = ',')]
    pub morphs: Option<Vec<String>>,

    /// Show optimization progress
    #[arg(short = 'v', long)]
    pub verbose: bool,

    /// Number of parallel threads to use (default: half of available CPUs)
    #[arg(short = 'p', long)]
    pub parallelism: Option<u8>,
}

fn parse_class_name(s: &str) -> Result<ClassName, String> {
    let s = s.trim();
    match s.to_lowercase().as_str() {
        "dragonknight" => Ok(ClassName::Dragonknight),
        "sorcerer" => Ok(ClassName::Sorcerer),
        "nightblade" => Ok(ClassName::Nightblade),
        "warden" => Ok(ClassName::Warden),
        "templar" => Ok(ClassName::Templar),
        "arcanist" => Ok(ClassName::Arcanist),
        _ => Err(format!(
            "Invalid class '{}'. Valid options: Dragonknight, Sorcerer, Nightblade, Warden, Templar, Arcanist",
            s
        )),
    }
}

fn parse_weapon(s: &str) -> Result<SkillLineName, String> {
    let s = s.trim();
    match s.to_lowercase().as_str() {
        "bow" => Ok(SkillLineName::Bow),
        "twohanded" | "two-handed" => Ok(SkillLineName::TwoHanded),
        "destructionstaff" | "destruction-staff" => Ok(SkillLineName::DestructionStaff),
        "dualwield" | "dual-wield" => Ok(SkillLineName::DualWield),
        _ => Err(format!(
            "Invalid weapon '{}'. Valid options: Bow, TwoHanded, DestructionStaff, DualWield",
            s
        )),
    }
}

impl OptimizeArgs {
    pub fn run(&self) {
        // Validate class count
        if let Some(classes) = &self.classes {
            if classes.len() > BUILD_CONSTRAINTS.class_skill_line_count {
                logger::error(&format!(
                    "Maximum {} classes allowed",
                    BUILD_CONSTRAINTS.class_skill_line_count
                ));
                std::process::exit(1);
            }
        }

        // Validate weapon count
        if let Some(weapons) = &self.weapons {
            if weapons.len() > BUILD_CONSTRAINTS.weapon_skill_line_count {
                logger::error(&format!(
                    "Maximum {} weapons allowed",
                    BUILD_CONSTRAINTS.weapon_skill_line_count
                ));
                std::process::exit(1);
            }
        }

        logger::info("Finding optimal build...");

        let optimizer = BuildOptimizer::new(BuildOptimizerOptions {
            verbose: self.verbose,
            required_class_names: self.classes.clone().unwrap_or_default(),
            required_weapon_skill_lines: self.weapons.clone().unwrap_or_default(),
            forced_morphs: self.morphs.clone().unwrap_or_default(),
            parallelism: self
                .parallelism
                .unwrap_or_else(|| (num_cpus::get() / 2).max(1) as u8),
        });

        let build = optimizer.find_optimal_build();

        match build {
            Some(b) => {
                println!("{}", b.format_display());
            }
            None => {
                logger::error("No valid build found with the given constraints.");
                std::process::exit(1);
            }
        }
    }
}
