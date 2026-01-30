use crate::data::{ClassName, SkillLineName};
use crate::domain::{BonusData, PassiveData, Skill};
use crate::services::{calculate_passive_bonus, get_passives_by_skill_line};
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
    skills: Vec<Skill>,
    champion_points: Vec<BonusData>,
    used_class_skill_lines: Vec<SkillLineName>,
    used_weapon_skill_lines: Vec<SkillLineName>,
    required_class: Option<ClassName>,
    passives: Vec<PassiveData>,
    skill_damages: HashMap<String, f64>,
    total_damage: f64,
}

impl Build {
    pub fn new(skills: Vec<Skill>, champion_points: Vec<BonusData>) -> Self {
        // Derive skill lines from skills
        let mut class_skill_lines: HashSet<SkillLineName> = HashSet::new();
        let mut weapon_skill_lines: HashSet<SkillLineName> = HashSet::new();

        for skill in &skills {
            let skill_line = skill.skill_line();
            if skill_line.is_weapon() {
                weapon_skill_lines.insert(skill_line);
            } else {
                class_skill_lines.insert(skill_line);
            }
        }

        let used_class_skill_lines: Vec<_> = class_skill_lines.into_iter().collect();
        let used_weapon_skill_lines: Vec<_> = weapon_skill_lines.into_iter().collect();

        // Derive required class
        let required_class = Self::derive_required_class(&used_class_skill_lines);

        // Get passives for skill lines
        let passives =
            Self::get_passives_for_skill_lines(&used_class_skill_lines, &used_weapon_skill_lines);

        // Calculate damages
        let (skill_damages, total_damage) =
            Self::calculate_damages(&skills, &champion_points, &passives);

        Self {
            skills,
            champion_points,
            used_class_skill_lines,
            used_weapon_skill_lines,
            required_class,
            passives,
            skill_damages,
            total_damage,
        }
    }

    fn derive_required_class(class_skill_lines: &[SkillLineName]) -> Option<ClassName> {
        let mut class_names: HashSet<ClassName> = HashSet::new();
        for line in class_skill_lines {
            class_names.insert(line.get_class());
        }
        if class_names.len() == 1 {
            class_names.into_iter().next()
        } else {
            None
        }
    }

    fn get_passives_for_skill_lines(
        class_lines: &[SkillLineName],
        weapon_lines: &[SkillLineName],
    ) -> Vec<PassiveData> {
        let mut passives = Vec::new();
        for line in class_lines {
            passives.extend(get_passives_by_skill_line(*line));
        }
        for line in weapon_lines {
            passives.extend(get_passives_by_skill_line(*line));
        }
        passives
    }

    fn get_skill_line_counts(skills: &[Skill]) -> HashMap<SkillLineName, usize> {
        let mut counts: HashMap<SkillLineName, usize> = HashMap::new();
        for skill in skills {
            *counts.entry(skill.skill_line()).or_insert(0) += 1;
        }
        counts
    }

    fn calculate_damages(
        skills: &[Skill],
        champion_points: &[BonusData],
        passives: &[PassiveData],
    ) -> (HashMap<String, f64>, f64) {
        let mut skill_damages = HashMap::new();
        let skill_line_counts = Self::get_skill_line_counts(skills);
        let mut total_damage = 0.0;

        for skill in skills {
            let base_damage = skill.calculate_damage_per_cast(champion_points);
            let skill_line_count = skill_line_counts
                .get(&skill.skill_line())
                .copied()
                .unwrap_or(0);
            let passive_bonus = calculate_passive_bonus(passives, skill_line_count);
            let damage = base_damage * (1.0 + passive_bonus);

            skill_damages.insert(skill.name().to_string(), damage);
            total_damage += damage;
        }

        (skill_damages, total_damage)
    }

    // Getters
    pub fn skills(&self) -> &[Skill] {
        &self.skills
    }

    pub fn champion_points(&self) -> &[BonusData] {
        &self.champion_points
    }

    pub fn modifiers(&self) -> &[BonusData] {
        &self.champion_points
    }

    pub fn used_class_skill_lines(&self) -> &[SkillLineName] {
        &self.used_class_skill_lines
    }

    pub fn used_weapon_skill_lines(&self) -> &[SkillLineName] {
        &self.used_weapon_skill_lines
    }

    pub fn required_class(&self) -> Option<ClassName> {
        self.required_class
    }

    pub fn passives(&self) -> &[PassiveData] {
        &self.passives
    }

    pub fn total_damage_per_cast(&self) -> f64 {
        self.total_damage
    }

    pub fn get_skill_damage(&self, skill_name: &str) -> f64 {
        self.skill_damages.get(skill_name).copied().unwrap_or(0.0)
    }

    /// Validate build against constraints
    pub fn is_valid(&self) -> bool {
        self.skills.len() <= BUILD_CONSTRAINTS.skill_count
            && self.champion_points.len() <= BUILD_CONSTRAINTS.champion_point_count
            && self.used_class_skill_lines.len() <= BUILD_CONSTRAINTS.class_skill_line_count
            && self.used_weapon_skill_lines.len() <= BUILD_CONSTRAINTS.weapon_skill_line_count
    }

    /// Check if this build is better than another
    pub fn is_better_than(&self, other: Option<&Build>) -> bool {
        match other {
            None => true,
            Some(o) => self.total_damage > o.total_damage,
        }
    }

    /// Format build for display
    pub fn format_display(&self) -> String {
        use crate::infrastructure::table;

        let mut lines = Vec::new();
        let divider = "-".repeat(73);

        lines.push(String::new());
        lines.push("Optimal Build - Maximum Damage Per Cast".to_string());
        lines.push(divider.clone());
        lines.push(format!(
            "Total Damage: {}",
            format_number(self.total_damage)
        ));
        lines.push(String::new());

        let modifier_names: Vec<_> = self
            .champion_points
            .iter()
            .filter_map(|m| m.name.as_ref())
            .map(|s| s.as_str())
            .collect();
        lines.push(format!("Modifiers: {}", modifier_names.join(", ")));

        // Skills table
        let skills_data: Vec<Vec<String>> = self
            .skills
            .iter()
            .enumerate()
            .map(|(i, skill)| {
                vec![
                    (i + 1).to_string(),
                    skill.name().to_string(),
                    skill.class_name().to_string(),
                    skill.skill_line().to_string(),
                    format!("{:.0}", self.get_skill_damage(skill.name())),
                ]
            })
            .collect();

        let class_lines: Vec<_> = self
            .used_class_skill_lines
            .iter()
            .map(|l| l.to_string())
            .collect();
        let weapon_lines: Vec<_> = self
            .used_weapon_skill_lines
            .iter()
            .map(|l| l.to_string())
            .collect();

        let skills_footer = if let Some(required_class) = self.required_class {
            format!(
                "Skill Lines: {} ({}/3 class), {} ({}/2 weapon)\nRequired Class: {}",
                class_lines.join(", "),
                self.used_class_skill_lines.len(),
                weapon_lines.join(", "),
                self.used_weapon_skill_lines.len(),
                required_class
            )
        } else {
            format!(
                "Skill Lines: {} ({}/3 class), {} ({}/2 weapon)",
                class_lines.join(", "),
                self.used_class_skill_lines.len(),
                weapon_lines.join(", "),
                self.used_weapon_skill_lines.len()
            )
        };

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
                footer: Some(skills_footer),
            },
        ));

        // Passives table
        if !self.passives.is_empty() {
            let passives_data: Vec<Vec<String>> = self
                .passives
                .iter()
                .map(|passive| {
                    vec![
                        passive.name.clone(),
                        passive.class_name.to_string(),
                        passive.skill_line.to_string(),
                    ]
                })
                .collect();

            lines.push(table(
                &passives_data,
                table::TableOptions {
                    title: Some("Passives".to_string()),
                    columns: vec![
                        table::ColumnDefinition::new("Name", 30),
                        table::ColumnDefinition::new("Source", 15),
                        table::ColumnDefinition::new("Skill Line", 20),
                    ],
                    footer: None,
                },
            ));
        }

        lines.join("\n")
    }
}

fn format_number(n: f64) -> String {
    let n = n as i64;
    let s = n.to_string();
    let mut result = String::new();
    for (i, c) in s.chars().rev().enumerate() {
        if i > 0 && i % 3 == 0 {
            result.push(',');
        }
        result.push(c);
    }
    result.chars().rev().collect()
}
