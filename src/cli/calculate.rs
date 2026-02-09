use super::parsers::{parse_champion_point, parse_class_skill_line, parse_weapon_skill_line};
use crate::domain::{BonusData, Build, SkillLineName, BUILD_CONSTRAINTS};
use crate::infrastructure::logger;
use crate::services::{
    MorphSelectionOptions, PassivesService, PassivesServiceOptions, SkillsFilter, SkillsService,
    SkillsServiceOptions,
};
use clap::Args;

/// Calculate total damage for a specific build configuration
#[derive(Args, Debug)]
pub struct CalculateArgs {
    /// 3 class skill lines (comma-separated)
    #[arg(short = 'c', long, value_delimiter = ',', value_parser = parse_class_skill_line, required = true)]
    pub class_skill_lines: Vec<SkillLineName>,

    /// 2 weapon skill lines (comma-separated)
    #[arg(short = 'w', long, value_delimiter = ',', value_parser = parse_weapon_skill_line, required = true)]
    pub weapon_skill_lines: Vec<SkillLineName>,

    /// 4 champion points (comma-separated)
    #[arg(short = 'p', long, value_delimiter = ',', value_parser = parse_champion_point, required = true)]
    pub champion_points: Vec<BonusData>,

    /// Force specific morph selections (comma-separated morph names)
    #[arg(short = 'm', long, value_delimiter = ',')]
    pub morphs: Option<Vec<String>>,
}

impl CalculateArgs {
    pub fn run(&self) {
        // Validate class skill line count
        if self.class_skill_lines.len() != BUILD_CONSTRAINTS.class_skill_line_count {
            logger::error(&format!(
                "Exactly {} class skill lines required, got {}",
                BUILD_CONSTRAINTS.class_skill_line_count,
                self.class_skill_lines.len()
            ));
            std::process::exit(1);
        }

        // Validate weapon skill line count
        if self.weapon_skill_lines.len() != BUILD_CONSTRAINTS.weapon_skill_line_count {
            logger::error(&format!(
                "Exactly {} weapon skill lines required, got {}",
                BUILD_CONSTRAINTS.weapon_skill_line_count,
                self.weapon_skill_lines.len()
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

        // Validate class skill lines are actually class skill lines (not weapon)
        for sl in &self.class_skill_lines {
            if sl.is_weapon() {
                logger::error(&format!(
                    "'{}' is a weapon skill line, not a class skill line",
                    sl
                ));
                std::process::exit(1);
            }
        }

        // Validate weapon skill lines are actually weapon skill lines
        for sl in &self.weapon_skill_lines {
            if !sl.is_weapon() {
                logger::error(&format!(
                    "'{}' is a class skill line, not a weapon skill line",
                    sl
                ));
                std::process::exit(1);
            }
        }

        logger::info("Calculating build damage...");

        // Combine all skill lines
        let all_skill_lines: Vec<SkillLineName> = self
            .class_skill_lines
            .iter()
            .chain(self.weapon_skill_lines.iter())
            .copied()
            .collect();

        // Get skills with morph selection and filtering
        let skills_service = SkillsService::new(SkillsServiceOptions::default())
            .with_morph_selection(MorphSelectionOptions {
                forced_morphs: self.morphs.clone().unwrap_or_default(),
            })
            .with_filter(SkillsFilter {
                exclude_ultimates: true,
                exclude_non_damaging: true,
            });

        // Collect all skills from the skill lines
        let mut all_skills: Vec<_> = all_skill_lines
            .iter()
            .flat_map(|sl| skills_service.get_skills_by_skill_line(*sl))
            .collect();

        // Sort by damage (descending) and take top 10
        all_skills.sort_by(|a, b| {
            let damage_a = a.calculate_damage_per_cast(&[]);
            let damage_b = b.calculate_damage_per_cast(&[]);
            damage_b
                .partial_cmp(&damage_a)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        let selected_skills: Vec<_> = all_skills
            .into_iter()
            .take(BUILD_CONSTRAINTS.skill_count)
            .collect();

        if selected_skills.len() < BUILD_CONSTRAINTS.skill_count {
            logger::warn(&format!(
                "Only {} skills available from selected skill lines (expected {})",
                selected_skills.len(),
                BUILD_CONSTRAINTS.skill_count
            ));
        }

        // Get passives from the skill lines
        let passives_service = PassivesService::new(PassivesServiceOptions::default());
        let passive_bonuses: Vec<BonusData> = all_skill_lines
            .iter()
            .flat_map(|sl| passives_service.get_passives_by_skill_line(*sl))
            .flat_map(|p| p.bonuses.iter().cloned())
            .collect();

        // Create the build
        let build = Build::new(
            selected_skills,
            self.champion_points.clone(),
            &passive_bonuses,
        );

        logger::info(&build.to_string());
    }
}
