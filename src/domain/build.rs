use crate::data::{ClassName, SkillLineName};
use crate::domain::{BonusData, SkillData};
use crate::infrastructure::{format, table};
use std::collections::{HashMap, HashSet};

/// Build constraints
pub const BUILD_CONSTRAINTS: BuildConstraints = BuildConstraints {
    skill_count: 10,
    champion_point_count: 4,
    class_skill_line_count: 3,
    weapon_skill_line_count: 2,
};

#[derive(Debug, Clone, Copy)]
pub struct BuildConstraints {
    pub skill_count: usize,
    pub champion_point_count: usize,
    pub class_skill_line_count: usize,
    pub weapon_skill_line_count: usize,
}

/// A complete build with skills, champion points, and calculated damages
#[derive(Debug, Clone)]
pub struct Build {
    skills: Vec<&'static SkillData>,
    champion_bonuses: Vec<BonusData>,
    skill_line_counts: HashMap<SkillLineName, usize>,
    pub total_damage: f64,
}

impl Build {
    pub fn new(
        skills: Vec<&'static SkillData>,
        champion_bonuses: Vec<BonusData>,
        passive_bonuses: &[BonusData],
    ) -> Self {
        let mut skill_line_counts: HashMap<SkillLineName, usize> = HashMap::new();
        for skill in &skills {
            *skill_line_counts.entry(skill.skill_line).or_insert(0) += 1;
        }

        // FIXME: some passives are only active while on that bar,
        // do we wanna apply combination here too?
        let mut all_bonuses = champion_bonuses.clone();
        all_bonuses.extend(passive_bonuses.iter().cloned());

        let mut total_damage = 0.0;
        for skill in &skills {
            let damage = skill.calculate_damage_per_cast(&all_bonuses);
            total_damage += damage;
        }

        Self {
            skills,
            champion_bonuses,
            skill_line_counts,
            total_damage,
        }
    }
}

impl std::fmt::Display for Build {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut lines = Vec::new();
        let divider = "-".repeat(73);

        lines.push(String::new());
        lines.push("Optimal Build - Maximum Damage Per Cast".to_string());
        lines.push(divider.clone());
        lines.push(format!(
            "Total Damage: {}",
            format::format_number(self.total_damage as u64) // FIXME
        ));
        lines.push(String::new());

        let class_names: HashSet<_> = self
            .skill_line_counts
            .keys()
            .map(|sl| sl.get_class())
            .filter(|c| *c != ClassName::Weapon)
            .collect();
        let mut class_names: Vec<_> = class_names.iter().map(|c| c.to_string()).collect();
        class_names.sort();
        lines.push(format!("Classes: {}", class_names.join(", ")));

        let mut class_skill_lines: Vec<_> = self
            .skill_line_counts
            .keys()
            .filter(|sl| !sl.is_weapon())
            .map(|sl| sl.to_string())
            .collect();
        class_skill_lines.sort();
        lines.push(format!(
            "Class Skill Lines: {}",
            class_skill_lines.join(", ")
        ));

        let mut weapon_skill_lines: Vec<_> = self
            .skill_line_counts
            .keys()
            .filter(|sl| sl.is_weapon())
            .map(|sl| sl.to_string())
            .collect();
        weapon_skill_lines.sort();
        lines.push(format!(
            "Weapon Skill Lines: {}",
            weapon_skill_lines.join(", ")
        ));

        let mut champion_point_names: Vec<_> = self
            .champion_bonuses
            .iter()
            .map(|m| m.name.as_str())
            .collect();
        champion_point_names.sort();
        lines.push(format!(
            "Champion Points: {}",
            champion_point_names.join(", ")
        ));

        // Skills table
        let skills_data: Vec<Vec<String>> = self
            .skills
            .iter()
            .enumerate()
            .map(|(i, skill)| {
                vec![
                    (i + 1).to_string(),
                    skill.name.clone(),
                    skill.class_name.to_string(),
                    skill.skill_line.to_string(),
                ]
            })
            .collect();

        lines.push(table(
            &skills_data,
            table::TableOptions {
                title: Some("Skills".to_string()),
                columns: vec![
                    table::ColumnDefinition::new("#", 4).align_right(),
                    table::ColumnDefinition::new("Name", 25),
                    table::ColumnDefinition::new("Source", 12),
                    table::ColumnDefinition::new("Skill Line", 18),
                    table::ColumnDefinition::new("Damage", 10).align_right(),
                ],
                footer: None,
            },
        ));

        write!(f, "{}", lines.join("\n"))
    }
}
