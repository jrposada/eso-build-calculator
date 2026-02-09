use super::parsers::{parse_champion_point, parse_skill};
use crate::domain::{BonusData, Build, SkillData, SkillLineName, BUILD_CONSTRAINTS};
use crate::infrastructure::logger;
use crate::services::{PassivesService, PassivesServiceOptions};
use clap::Args;
use std::collections::HashSet;

/// Calculate total damage for a specific build configuration
#[derive(Args, Debug)]
pub struct CalculateArgs {
    /// 10 skills (comma-separated skill names)
    #[arg(short = 's', long, value_delimiter = ',', value_parser = parse_skill, required = true)]
    pub skills: Vec<&'static SkillData>,

    /// 4 champion points (comma-separated)
    #[arg(short = 'p', long, value_delimiter = ',', value_parser = parse_champion_point, required = true)]
    pub champion_points: Vec<BonusData>,
}

impl CalculateArgs {
    pub fn run(&self) {
        // Validate skill count
        if self.skills.len() != BUILD_CONSTRAINTS.skill_count {
            logger::error(&format!(
                "Exactly {} skills required, got {}",
                BUILD_CONSTRAINTS.skill_count,
                self.skills.len()
            ));
            std::process::exit(1);
        }

        // Validate champion point count
        if self.champion_points.len() != BUILD_CONSTRAINTS.champion_point_count {
            logger::error(&format!(
                "Exactly {} champion points required, got {}",
                BUILD_CONSTRAINTS.champion_point_count,
                self.champion_points.len()
            ));
            std::process::exit(1);
        }

        logger::info("Calculating build damage...");

        // Collect unique skill lines from the selected skills
        let skill_lines: HashSet<SkillLineName> =
            self.skills.iter().map(|s| s.skill_line).collect();

        // Get passives from the skill lines
        let passives_service = PassivesService::new(PassivesServiceOptions::default());
        let passive_bonuses: Vec<BonusData> = skill_lines
            .iter()
            .flat_map(|sl| passives_service.get_passives_by_skill_line(*sl))
            .flat_map(|p| p.bonuses.iter().cloned())
            .collect();

        // Create the build
        let build = Build::new(
            self.skills.clone(),
            self.champion_points.clone(),
            &passive_bonuses,
        );

        logger::info(&build.to_string());
    }
}
