use super::{
    formulas, BonusData, BonusTarget, CharacterStats, ClassName, ResolveContext, SkillData,
    SkillLineName,
};
use crate::{
    domain::character_stats::MAX_CRITICAL_DAMAGE,
    infrastructure::{format, table},
};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
pub struct Build {
    skills: Vec<SkillData>,
    all_bonuses: Vec<BonusData>,
    // champion_bonuses: Vec<BonusData>,
    // resolved_bonuses: Vec<BonusData>,
    // resolved_passive_bonuses: Vec<BonusData>,

    // pub total_damage: f64,
    // crit_damage: f64,
    // conditional_bonuses: Vec<BonusData>,
    // character_stats: CharacterStats,
    // effective_stats: CharacterStats,
    // passive_effective_stats: CharacterStats,
}

// Constructor
impl Build {
    pub fn new(
        skills: Vec<&'static SkillData>,
        all_bonuses: Vec<BonusData>,
        character_stats: CharacterStats,
    ) -> Self {
        // Clone skills for storage (better cache locality in parallel execution)
        let skills: Vec<SkillData> = skills.into_iter().map(|s| s.clone()).collect();

        // FIXME: some passives are only active while on that bar,
        // do we wanna apply combination here too?

        // --- Partition bonuses into regular vs alternatives ---
        let mut regular_bonuses: Vec<BonusData> = Vec::new();
        let mut alt_groups: HashMap<u16, Vec<BonusData>> = HashMap::new();
        for bonus in all_bonuses {
            if let Some(group) = bonus.alternatives_group {
                alt_groups.entry(group).or_default().push(bonus);
            } else {
                regular_bonuses.push(bonus);
            }
        }

        // --- Resolve alternatives: pick one from each group to maximize damage ---
        let alternatives_selections = if !alt_groups.is_empty() {
            Self::resolve_alternatives(&skills, &regular_bonuses, &alt_groups, &character_stats)
        } else {
            Vec::new()
        };

        // --- Recombine: merge selected alternatives back into bonuses ---
        for selection in &alternatives_selections {
            regular_bonuses.push(selection.selected.clone());
        }
        let all_bonuses = regular_bonuses;

        // --- Existing flow ---
        // Aggregate crit damage from non-conditional bonuses to build context
        let crit_damage_bonus = Self::aggregate_crit_damage(&all_bonuses);
        // Use character stats crit damage as base, add bonus from passives
        let base_crit_damage = character_stats.critical_damage - 1.0; // Convert from multiplier to bonus
        let crit_damage = (base_crit_damage + crit_damage_bonus).min(MAX_CRITICAL_DAMAGE - 1.0);
        let ctx = ResolveContext::new(crit_damage);

        // Store conditional bonuses for Display (minimal overhead - usually 0-2 items)
        let conditional_bonuses: Vec<BonusData> = all_bonuses
            .iter()
            .filter(|b| b.is_conditional())
            .cloned()
            .collect();

        // Resolve all bonuses
        let resolved_bonuses = Self::resolve_bonuses(&all_bonuses, &ctx);

        // Apply stat-based bonuses to character stats for accurate damage calculation
        let effective_stats =
            Self::apply_stat_bonuses_to_stats(&resolved_bonuses, &character_stats);

        // Resolve passive-only bonuses for tooltip damage (excludes champion points)
        let cp_names: HashSet<&str> = champion_bonuses.iter().map(|b| b.name.as_str()).collect();
        let passive_only: Vec<_> = all_bonuses
            .iter()
            .filter(|b| !cp_names.contains(b.name.as_str()))
            .cloned()
            .collect();
        let resolved_passive_bonuses = Self::resolve_bonuses(&passive_only, &ctx);
        let passive_effective_stats =
            Self::apply_stat_bonuses_to_stats(&resolved_passive_bonuses, &character_stats);

        let mut total_damage = 0.0;
        for skill in &skills {
            total_damage += skill.calculate_damage_with_stats(&resolved_bonuses, &effective_stats);
        }

        Self {
            skills,
            champion_bonuses,
            resolved_bonuses,
            resolved_passive_bonuses,
            total_damage,
            crit_damage,
            conditional_bonuses,
            character_stats,
            effective_stats,
            passive_effective_stats,
            alternatives_selections,
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
                BonusData::new(
                    &bonus.name,
                    bonus.source,
                    bonus.bonus_trigger,
                    target,
                    value,
                )
                .with_duration(bonus.duration.unwrap_or(0.0))
                .with_cooldown(bonus.cooldown.unwrap_or(0.0))
            })
            .collect()
    }

    /// Apply stat-based bonuses to a cloned CharacterStats.
    /// Stat-based bonuses (weapon damage, crit damage, crit rating, penetration)
    /// don't affect damage through percentage modifiers — they must be folded into
    /// the character stats used by `calculate_damage_with_stats`.
    fn apply_stat_bonuses_to_stats(
        bonuses: &[BonusData],
        base_stats: &CharacterStats,
    ) -> CharacterStats {
        let mut stats = base_stats.clone();
        for bonus in bonuses {
            match bonus.target {
                BonusTarget::WeaponAndSpellDamageFlat => {
                    stats.weapon_damage += bonus.value;
                    stats.spell_damage += bonus.value;
                }
                BonusTarget::CriticalDamage => {
                    stats.critical_damage += bonus.value;
                }
                BonusTarget::CriticalRating => {
                    stats.critical_chance += formulas::crit_rating_to_bonus_chance(bonus.value);
                }
                BonusTarget::PhysicalAndSpellPenetration => {
                    stats.penetration += bonus.value;
                }
                _ => {}
            }
        }
        stats.clamp_caps();
        stats
    }

    /// Resolve mutually exclusive alternative groups by exhaustively evaluating
    /// all combinations and picking the one that maximizes total damage.
    fn resolve_alternatives(
        skills: &[SkillData],
        regular_bonuses: &[BonusData],
        alt_groups: &HashMap<u16, Vec<BonusData>>,
        character_stats: &CharacterStats,
    ) -> Vec<AlternativeSelection> {
        // Resolve regular bonuses for comparison
        let crit_damage_bonus = Self::aggregate_crit_damage(regular_bonuses);
        let base_crit_damage = character_stats.critical_damage - 1.0;
        let crit_damage = (base_crit_damage + crit_damage_bonus).min(MAX_CRITICAL_DAMAGE - 1.0);
        let ctx = ResolveContext::new(crit_damage);
        let resolved_regular = Self::resolve_bonuses(regular_bonuses, &ctx);

        // Apply stat-based regular bonuses to base stats
        let base_effective_stats =
            Self::apply_stat_bonuses_to_stats(&resolved_regular, character_stats);

        // Build ordered group list for cartesian product
        let group_ids: Vec<u16> = alt_groups.keys().copied().collect();
        let group_options: Vec<&Vec<BonusData>> =
            group_ids.iter().map(|id| &alt_groups[id]).collect();

        // Generate cartesian product: one pick per group
        let mut combinations: Vec<Vec<usize>> = vec![vec![]];
        for options in &group_options {
            let mut new_combinations = Vec::new();
            for combo in &combinations {
                for idx in 0..options.len() {
                    let mut new_combo = combo.clone();
                    new_combo.push(idx);
                    new_combinations.push(new_combo);
                }
            }
            combinations = new_combinations;
        }

        // Evaluate each combination to find the one with maximum damage
        let mut best_combo: Vec<usize> = vec![0; group_ids.len()];
        let mut best_damage = f64::NEG_INFINITY;

        for combo in &combinations {
            let mut stats = base_effective_stats.clone();
            let mut percentage_bonuses = resolved_regular.clone();

            for (group_idx, &option_idx) in combo.iter().enumerate() {
                let bonus = &group_options[group_idx][option_idx];
                match bonus.target {
                    BonusTarget::WeaponAndSpellDamageFlat => {
                        stats.weapon_damage += bonus.value;
                        stats.spell_damage += bonus.value;
                    }
                    BonusTarget::CriticalDamage => {
                        stats.critical_damage += bonus.value;
                    }
                    BonusTarget::CriticalRating => {
                        stats.critical_chance += formulas::crit_rating_to_bonus_chance(bonus.value);
                    }
                    BonusTarget::PhysicalAndSpellPenetration => {
                        stats.penetration += bonus.value;
                    }
                    _ => {
                        // Percentage types go into the bonus list
                        percentage_bonuses.push(bonus.clone());
                    }
                }
            }

            stats.clamp_caps();

            let total: f64 = skills
                .iter()
                .map(|skill| skill.calculate_damage_with_stats(&percentage_bonuses, &stats))
                .sum();

            if total > best_damage {
                best_damage = total;
                best_combo = combo.clone();
            }
        }

        // Build selections from best combination
        group_ids
            .iter()
            .enumerate()
            .map(|(group_idx, &group_id)| AlternativeSelection {
                group: group_id,
                selected: group_options[group_idx][best_combo[group_idx]].clone(),
                options: alt_groups[&group_id].clone(),
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

    fn fmt_character_stats(&self) -> String {
        let base = &self.character_stats;
        let eff = &self.effective_stats;

        let fmt_stat = |base_val: f64, eff_val: f64| -> (String, String) {
            (
                format::format_number(base_val as u64),
                format::format_number(eff_val as u64),
            )
        };

        let fmt_pct = |base_val: f64, eff_val: f64| -> (String, String) {
            (
                format!("{:.2}%", base_val * 100.0),
                format!("{:.2}%", eff_val * 100.0),
            )
        };

        let fmt_crit_dmg = |base_val: f64, eff_val: f64| -> (String, String) {
            (
                format!("{:.2}%", (base_val - 1.0) * 100.0),
                format!("{:.2}%", (eff_val - 1.0) * 100.0),
            )
        };

        let stats: Vec<(&str, String, String)> = vec![
            {
                let (b, e) = fmt_stat(base.max_magicka, eff.max_magicka);
                ("Max Magicka", b, e)
            },
            {
                let (b, e) = fmt_stat(base.max_stamina, eff.max_stamina);
                ("Max Stamina", b, e)
            },
            {
                let (b, e) = fmt_stat(base.weapon_damage, eff.weapon_damage);
                ("Weapon Damage", b, e)
            },
            {
                let (b, e) = fmt_stat(base.spell_damage, eff.spell_damage);
                ("Spell Damage", b, e)
            },
            {
                let (b, e) = fmt_pct(base.critical_chance, eff.critical_chance);
                ("Critical Chance", b, e)
            },
            {
                let (b, e) = fmt_crit_dmg(base.critical_damage, eff.critical_damage);
                ("Critical Damage", b, e)
            },
            {
                let (b, e) = fmt_stat(base.penetration, eff.penetration);
                ("Penetration", b, e)
            },
            {
                let (b, e) = fmt_stat(base.target_armor, eff.target_armor);
                ("Target Armor", b, e)
            },
        ];

        let data: Vec<Vec<String>> = stats
            .into_iter()
            .map(|(name, b, e)| vec![name.to_string(), b, e])
            .collect();

        table(
            &data,
            table::TableOptions {
                title: Some("Character Stats".to_string()),
                columns: vec![
                    table::ColumnDefinition::new("Stat", 20),
                    table::ColumnDefinition::new("Base", 12).align_right(),
                    table::ColumnDefinition::new("Effective", 12).align_right(),
                ],
                footer: None,
            },
        )
    }

    fn fmt_build_summary(&self) -> Vec<String> {
        let mut skill_line_counts: HashMap<SkillLineName, usize> = HashMap::new();
        for skill in &self.skills {
            *skill_line_counts.entry(skill.skill_line).or_insert(0) += 1;
        }

        let class_names: HashSet<_> = skill_line_counts
            .keys()
            .map(|sl| sl.get_class())
            .filter(|c| *c != ClassName::Weapon)
            .collect();

        let mut class_names: Vec<_> = class_names.iter().map(|c| c.to_string()).collect();
        class_names.sort();

        let mut class_skill_lines: Vec<_> = skill_line_counts
            .keys()
            .filter(|sl| !sl.is_weapon())
            .map(|sl| sl.to_string())
            .collect();
        class_skill_lines.sort();

        let mut weapon_skill_lines: Vec<_> = skill_line_counts
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
        let mut skills_with_damage: Vec<_> = self
            .skills
            .iter()
            .map(|skill| {
                let tooltip = skill.calculate_tooltip_damage_with_stats(
                    &self.resolved_passive_bonuses,
                    &self.passive_effective_stats,
                );
                let effective = skill
                    .calculate_damage_with_stats(&self.resolved_bonuses, &self.effective_stats);
                (skill, tooltip, effective)
            })
            .collect();
        skills_with_damage.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap());

        let skills_data: Vec<Vec<String>> = skills_with_damage
            .iter()
            .enumerate()
            .map(|(i, (skill, tooltip, effective))| {
                let type_str = if skill.spammable {
                    format!("{} *", skill.mechanic())
                } else {
                    skill.mechanic().to_string()
                };
                vec![
                    (i + 1).to_string(),
                    skill.name.to_string(),
                    skill.class_name.to_string(),
                    skill.skill_line.to_string(),
                    type_str,
                    format::format_number(*tooltip as u64),
                    format::format_number(*effective as u64),
                ]
            })
            .collect();

        let result = table(
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
                    table::ColumnDefinition::new("Eff. Damage", 12).align_right(),
                ],
                footer: Some("*Spammable skill".to_string()),
            },
        );

        result
    }

    fn fmt_bonuses(&self) -> String {
        let mut bonuses = self.resolved_bonuses.clone();
        bonuses.sort_by(|a, b| a.name.cmp(&b.name));

        let bonuses_data: Vec<Vec<String>> = bonuses
            .iter()
            .enumerate()
            .map(|(i, bonus)| {
                let value_str = fmt_bonus_value(bonus.value);
                vec![
                    (i + 1).to_string(),
                    bonus.name.clone(),
                    bonus.source.to_string(),
                    bonus.target.to_string(),
                    value_str,
                ]
            })
            .collect();

        table(
            &bonuses_data,
            table::TableOptions {
                title: Some("Applied Bonuses".to_string()),
                columns: vec![
                    table::ColumnDefinition::new("#", 4).align_right(),
                    table::ColumnDefinition::new("Name", 30),
                    table::ColumnDefinition::new("Source", 27),
                    table::ColumnDefinition::new("Target", 27),
                    table::ColumnDefinition::new("Value", 10).align_right(),
                ],
                footer: None,
            },
        )
    }

    fn fmt_alternatives_selections(&self) -> Vec<String> {
        if self.alternatives_selections.is_empty() {
            return Vec::new();
        }

        let mut lines = vec![
            String::new(),
            "Weapon Type Selections:".to_string(),
            "-".repeat(73),
        ];

        for selection in &self.alternatives_selections {
            lines.push(format!("  {}:", alternatives_group_name(selection.group)));
            for option in &selection.options {
                let marker = if option.name == selection.selected.name {
                    ">>>"
                } else {
                    "   "
                };
                lines.push(format!(
                    "    {} {:<30} {:<25} {}",
                    marker,
                    option.name,
                    option.target.to_string(),
                    fmt_bonus_value(option.value),
                ));
            }
        }

        lines
    }

    fn fmt_conditional_buffs(&self) -> Vec<String> {
        if self.conditional_bonuses.is_empty() {
            return Vec::new();
        }

        let ctx = ResolveContext::new(self.crit_damage);
        let mut lines = vec![
            String::new(),
            "Conditional Buff Selections:".to_string(),
            "-".repeat(73),
        ];

        for bonus in &self.conditional_bonuses {
            if let Some(info) = bonus.selection_info(&ctx) {
                let selected = if info.used_alternative { ">>>" } else { "   " };
                let not_selected = if info.used_alternative { "   " } else { ">>>" };

                lines.push(format!(
                    "  {} (crit damage: {:.1}%, breakpoint: {:.1}%)",
                    bonus.name,
                    self.crit_damage * 100.0,
                    info.crit_damage_breakpoint * 100.0,
                ));
                lines.push(format!(
                    "    {} Primary:     {} {}",
                    not_selected,
                    info.primary_target,
                    fmt_bonus_value(info.primary_value),
                ));
                lines.push(format!(
                    "    {} Alternative: {} {}",
                    selected,
                    info.alternative_target,
                    fmt_bonus_value(info.alternative_value),
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
        lines.push(self.fmt_character_stats());
        lines.extend(self.fmt_build_summary());
        lines.push(self.fmt_skills_table());
        lines.push(self.fmt_bonuses());
        lines.extend(self.fmt_alternatives_selections());
        lines.extend(self.fmt_conditional_buffs());
        write!(f, "{}", lines.join("\n"))
    }
}
