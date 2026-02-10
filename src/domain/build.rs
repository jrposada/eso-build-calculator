use super::{
    BonusData, BonusTarget, CharacterStats, ClassName, ResolveContext, SkillData, SkillLineName,
};
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
    skills: Vec<SkillData>,
    champion_bonuses: Vec<BonusData>,
    skill_line_counts: HashMap<SkillLineName, usize>,
    pub total_damage: f64,
    crit_damage: f64,
    conditional_bonuses: Vec<BonusData>,
    character_stats: CharacterStats,
}

// Constructor
impl Build {
    pub fn new(
        skills: Vec<&'static SkillData>,
        champion_bonuses: Vec<BonusData>,
        passive_bonuses: &[BonusData],
    ) -> Self {
        Self::with_stats(
            skills,
            champion_bonuses,
            passive_bonuses,
            CharacterStats::default(),
        )
    }

    pub fn with_base_crit_damage(
        skills: Vec<&'static SkillData>,
        champion_bonuses: Vec<BonusData>,
        passive_bonuses: &[BonusData],
        base_crit_damage: f64,
    ) -> Self {
        Self::with_stats(
            skills,
            champion_bonuses,
            passive_bonuses,
            CharacterStats::default().with_critical_damage(1.0 + base_crit_damage),
        )
    }

    /// Create a build with custom character stats
    pub fn with_stats(
        skills: Vec<&'static SkillData>,
        champion_bonuses: Vec<BonusData>,
        passive_bonuses: &[BonusData],
        character_stats: CharacterStats,
    ) -> Self {
        let mut skill_line_counts: HashMap<SkillLineName, usize> = HashMap::new();
        for skill in &skills {
            *skill_line_counts.entry(skill.skill_line).or_insert(0) += 1;
        }

        // Clone skills for storage (better cache locality in parallel execution)
        let skills: Vec<SkillData> = skills.into_iter().map(|s| s.clone()).collect();

        // FIXME: some passives are only active while on that bar,
        // do we wanna apply combination here too?
        let mut all_bonuses = champion_bonuses.clone();
        all_bonuses.extend(passive_bonuses.iter().cloned());

        // Aggregate crit damage from non-conditional bonuses to build context
        let crit_damage_bonus = Self::aggregate_crit_damage(&all_bonuses);
        // Use character stats crit damage as base, add bonus from passives
        let base_crit_damage = character_stats.critical_damage - 1.0; // Convert from multiplier to bonus
        let crit_damage = base_crit_damage + crit_damage_bonus;
        let ctx = ResolveContext::new(crit_damage);

        // Store conditional bonuses for Display (minimal overhead - usually 0-2 items)
        let conditional_bonuses: Vec<BonusData> = all_bonuses
            .iter()
            .filter(|b| b.is_conditional())
            .cloned()
            .collect();

        // Resolve all bonuses
        let resolved_bonuses = Self::resolve_bonuses(&all_bonuses, &ctx);

        let mut total_damage = 0.0;
        for skill in &skills {
            let damage = skill.calculate_damage_per_cast(&resolved_bonuses);
            total_damage += damage;
        }

        Self {
            skills,
            champion_bonuses,
            skill_line_counts,
            total_damage,
            crit_damage,
            conditional_bonuses,
            character_stats,
        }
    }

    /// Aggregate crit damage from non-conditional bonuses
    fn aggregate_crit_damage(bonuses: &[BonusData]) -> f64 {
        bonuses
            .iter()
            .filter(|b| !b.is_conditional() && b.target == BonusTarget::CriticalDamage)
            .map(|b| b.value)
            .sum()
    }

    /// Resolve all bonuses using the build context.
    fn resolve_bonuses(bonuses: &[BonusData], ctx: &ResolveContext) -> Vec<BonusData> {
        bonuses
            .iter()
            .map(|bonus| {
                let (target, value) = bonus.resolve(ctx);
                BonusData::new(&bonus.name, bonus.bonus_trigger, target, value)
                    .with_duration(bonus.duration.unwrap_or(0.0))
                    .with_cooldown(bonus.cooldown.unwrap_or(0.0))
            })
            .collect()
    }
}

// Public getters
impl Build {
    /// Get skill names for export
    pub fn skill_names(&self) -> Vec<String> {
        self.skills.iter().map(|s| s.name.clone()).collect()
    }

    /// Get champion point names for export
    pub fn champion_point_names(&self) -> Vec<String> {
        self.champion_bonuses
            .iter()
            .map(|b| b.name.clone())
            .collect()
    }

    /// Get character stats
    pub fn character_stats(&self) -> &CharacterStats {
        &self.character_stats
    }
}

// Format
impl Build {
    fn fmt_header(&self) -> Vec<String> {
        let divider = "-".repeat(73);
        vec![
            String::new(),
            "Optimal Build - Maximum Damage Per Cast".to_string(),
            divider,
            format!(
                "Total Damage: {}",
                format::format_number(self.total_damage as u64) // FIXME
            ),
            String::new(),
        ]
    }

    fn fmt_build_summary(&self) -> Vec<String> {
        let class_names: HashSet<_> = self
            .skill_line_counts
            .keys()
            .map(|sl| sl.get_class())
            .filter(|c| *c != ClassName::Weapon)
            .collect();
        let mut class_names: Vec<_> = class_names.iter().map(|c| c.to_string()).collect();
        class_names.sort();

        let mut class_skill_lines: Vec<_> = self
            .skill_line_counts
            .keys()
            .filter(|sl| !sl.is_weapon())
            .map(|sl| sl.to_string())
            .collect();
        class_skill_lines.sort();

        let mut weapon_skill_lines: Vec<_> = self
            .skill_line_counts
            .keys()
            .filter(|sl| sl.is_weapon())
            .map(|sl| sl.to_string())
            .collect();
        weapon_skill_lines.sort();

        let mut champion_point_names: Vec<_> = self
            .champion_bonuses
            .iter()
            .map(|m| m.name.as_str())
            .collect();
        champion_point_names.sort();

        vec![
            format!("Classes: {}", class_names.join(", ")),
            format!("Class Skill Lines: {}", class_skill_lines.join(", ")),
            format!("Weapon Skill Lines: {}", weapon_skill_lines.join(", ")),
            format!("Champion Points: {}", champion_point_names.join(", ")),
        ]
    }

    fn fmt_skills_table(&self) -> String {
        let skills_data: Vec<Vec<String>> = self
            .skills
            .iter()
            .enumerate()
            .map(|(i, skill)| {
                vec![
                    (i + 1).to_string(),
                    skill.name.to_string(),
                    skill.class_name.to_string(),
                    skill.skill_line.to_string(),
                    skill.mechanic().to_string(),
                ]
            })
            .collect();

        table(
            &skills_data,
            table::TableOptions {
                title: Some("Skills".to_string()),
                columns: vec![
                    table::ColumnDefinition::new("#", 4).align_right(),
                    table::ColumnDefinition::new("Name", 25),
                    table::ColumnDefinition::new("Source", 12),
                    table::ColumnDefinition::new("Skill Line", 18),
                    table::ColumnDefinition::new("Type", 10),
                    table::ColumnDefinition::new("Damage", 10).align_right(),
                ],
                footer: None,
            },
        )
    }

    fn fmt_conditional_buffs(&self) -> Vec<String> {
        if self.conditional_bonuses.is_empty() {
            return Vec::new();
        }

        let ctx = ResolveContext::new(self.crit_damage);
        let mut lines = vec![
            String::new(),
            "Conditional Buff Selections:".to_string(),
            "-".repeat(40),
        ];

        for bonus in &self.conditional_bonuses {
            if let Some((used_alt, breakpoint)) = bonus.selection_info(&ctx) {
                let option = if used_alt { "alternative" } else { "primary" };
                lines.push(format!(
                    "  {}: {} (crit damage: {:.1}%, breakpoint: {:.1}%)",
                    bonus.name,
                    option,
                    self.crit_damage * 100.0,
                    breakpoint * 100.0
                ));
            }
        }

        lines
    }
}

impl std::fmt::Display for Build {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut lines = Vec::new();
        lines.extend(self.fmt_header());
        lines.extend(self.fmt_build_summary());
        lines.push(self.fmt_skills_table());
        lines.extend(self.fmt_conditional_buffs());
        write!(f, "{}", lines.join("\n"))
    }
}
