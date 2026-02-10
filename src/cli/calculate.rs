use super::build_config::BuildConfig;
use super::parsers::{parse_champion_point, parse_skill};
use crate::domain::{
    BonusData, Build, CharacterStats, SkillData, SkillLineName, BUILD_CONSTRAINTS,
};
use crate::infrastructure::logger;
use crate::services::{PassivesService, PassivesServiceOptions};
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
    #[arg(short = 'f', long, conflicts_with_all = ["skills", "champion_points"])]
    pub file: Option<PathBuf>,
}

impl CalculateArgs {
    pub fn run(&self) {
        let (skills, champion_points) = if let Some(path) = &self.file {
            self.load_from_file(path)
        } else {
            self.get_from_args()
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
        let passive_bonuses: Vec<BonusData> = skill_lines
            .iter()
            .flat_map(|sl| passives_service.get_passives_by_skill_line(*sl))
            .flat_map(|p| p.bonuses.iter().cloned())
            .collect();

        // Create the build
        let build = Build::new(
            skills.clone(),
            champion_points,
            &passive_bonuses,
            CharacterStats::default(),
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

        logger::info(&build.to_string());
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

    fn load_from_file(&self, path: &PathBuf) -> (Vec<&'static SkillData>, Vec<BonusData>) {
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

        (skills, champion_points)
    }
}
