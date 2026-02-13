use super::{
    BonusData, BonusSource, BonusTarget, BonusTrigger, CharacterStats, ClassName, ResolvedBonus,
    ResolveContext, SkillData, SkillLineName,
};
use crate::infrastructure::{format, table};
use smallvec::SmallVec;
use std::collections::{HashMap, HashSet};

/// Pre-computed passive bonus context for a given (skill combo, passive bonuses) pair.
/// Caches per-skill modifier sums from passive bonuses so the inner CP loop only
/// iterates the small CP bonus list (~4-8 items) instead of all ~30 bonuses.
pub struct CachedPassiveContext {
    /// character_stats + passive stat bonuses (unclamped, CP stats added on top per eval)
    pub base_stats: CharacterStats,
    /// Per skill, per hit: cached passive modifier sum
    pub hit_mods: SmallVec<[SmallVec<[f64; 4]>; 10]>,
    /// Per skill, per dot: cached passive modifier sum
    pub dot_mods: SmallVec<[SmallVec<[f64; 4]>; 10]>,
}

#[derive(Debug, Clone)]
pub struct Build {
    skills: Vec<&'static SkillData>,
    resolved_bonuses: Vec<BonusData>,
    character_stats: CharacterStats,
    effective_stats: CharacterStats,
    pub total_damage_per_cast: f64,
}

// Constructor
impl Build {
    pub fn new(
        skills: Vec<&'static SkillData>,
        cp_bonuses: &[BonusData],
        passive_bonuses: &[BonusData],
        character_stats: CharacterStats,
    ) -> Self {
        // FIXME: some passives are only active while on that bar,
        // do we wanna apply combination here too?

        let mut simple_bonuses: Vec<BonusData> = Vec::new();
        let mut alt_bonuses: Vec<BonusData> = Vec::new();
        for bonus in cp_bonuses.iter().chain(passive_bonuses.iter()).cloned() {
            if bonus.has_alternative() {
                alt_bonuses.push(bonus);
            } else {
                simple_bonuses.push(bonus);
            }
        }

        let intermediate_stats =
            Self::apply_stat_bonuses_to_stats(&simple_bonuses, &character_stats, &skills);

        let ctx = ResolveContext::new(intermediate_stats);
        let resolved_alts: Vec<BonusData> = alt_bonuses
            .into_iter()
            .map(|bonus| {
                let chosen = bonus.resolve(&ctx);
                BonusData::new(&bonus.name, bonus.source, bonus.trigger, chosen)
                    .with_cooldown(bonus.cooldown.unwrap_or(0.0))
                    .with_duration(bonus.duration.unwrap_or(0.0))
            })
            .collect();

        let mut resolved_bonuses = simple_bonuses;
        resolved_bonuses.extend(resolved_alts);

        let effective_stats =
            Self::apply_stat_bonuses_to_stats(&resolved_bonuses, &character_stats, &skills);

        // --- Calculate total damage ---
        let mut total_damage_per_cast = 0.0;
        for skill in &skills {
            total_damage_per_cast +=
                skill.calculate_damage_per_cast(&resolved_bonuses, &effective_stats, None);
        }

        Self {
            skills,
            resolved_bonuses,
            character_stats,
            effective_stats,
            total_damage_per_cast,
        }
    }

    fn apply_stat_bonuses_to_stats(
        bonuses: &[BonusData],
        base_stats: &CharacterStats,
        skills: &[&SkillData],
    ) -> CharacterStats {
        let mut stats = base_stats.clone();
        let ctx = ResolveContext::new(base_stats.clone());
        for bonus in bonuses {
            let bonus_value = bonus.resolve(&ctx);
            let bonus_multiplier = Self::bonus_multiplier(bonus, skills);
            // Bonus value has to be multiply according to trigger. If trigger is
            match bonus_value.target {
                BonusTarget::WeaponAndSpellDamageFlat => {
                    stats.weapon_damage += bonus_value.value * bonus_multiplier;
                    stats.spell_damage += bonus_value.value * bonus_multiplier;
                }
                BonusTarget::CriticalDamage => {
                    stats.critical_damage += bonus_value.value * bonus_multiplier;
                }
                BonusTarget::CriticalRating => {
                    stats.critical_rating += bonus_value.value * bonus_multiplier;
                }
                BonusTarget::PhysicalAndSpellPenetration => {
                    stats.penetration += bonus_value.value * bonus_multiplier;
                }
                _ => {}
            }
        }
        stats.clamp_caps();
        stats
    }

    fn bonus_multiplier(bonus: &BonusData, skills: &[&SkillData]) -> f64 {
        match bonus.trigger {
            BonusTrigger::AbilitySlottedCount => match bonus.skill_line_filter {
                Some(sl) => skills.iter().filter(|s| s.skill_line == sl).count() as f64,
                None => skills.len() as f64,
            },
            _ => 1.0,
        }
    }

    fn apply_stat_bonus(stats: &mut CharacterStats, target: BonusTarget, value: f64) {
        match target {
            BonusTarget::WeaponAndSpellDamageFlat => {
                stats.weapon_damage += value;
                stats.spell_damage += value;
            }
            BonusTarget::CriticalDamage => {
                stats.critical_damage += value;
            }
            BonusTarget::CriticalRating => {
                stats.critical_rating += value;
            }
            BonusTarget::PhysicalAndSpellPenetration => {
                stats.penetration += value;
            }
            _ => {}
        }
    }

    /// Fast damage computation without caching. Used when there is only 1 CP combo
    /// (caching overhead not amortized).
    pub fn compute_total_damage(
        skills: &[&'static SkillData],
        cp_pre_resolved: &[ResolvedBonus],
        cp_ability_count: &[BonusData],
        cp_alt: &[BonusData],
        passive_pre_resolved: &[ResolvedBonus],
        passive_ability_count: &[BonusData],
        passive_alt: &[BonusData],
        character_stats: &CharacterStats,
    ) -> f64 {
        let default_ctx = ResolveContext::default();

        let mut intermediate_stats = character_stats.clone();
        let mut effective_stats = character_stats.clone();
        let mut resolved: SmallVec<[ResolvedBonus; 24]> = SmallVec::new();

        for rb in cp_pre_resolved.iter().chain(passive_pre_resolved.iter()) {
            Self::apply_stat_bonus(&mut intermediate_stats, rb.target, rb.value);
            Self::apply_stat_bonus(&mut effective_stats, rb.target, rb.value);
            resolved.push(*rb);
        }

        for bonus in cp_ability_count.iter().chain(passive_ability_count.iter()) {
            let bv = bonus.resolve_ref(&default_ctx);
            let multiplier = Self::bonus_multiplier(bonus, skills);
            let applied = bv.value * multiplier;
            Self::apply_stat_bonus(&mut intermediate_stats, bv.target, applied);
            Self::apply_stat_bonus(&mut effective_stats, bv.target, applied);
            resolved.push(ResolvedBonus {
                target: bv.target,
                value: bv.value,
                skill_line_filter: bonus.skill_line_filter,
                execute_threshold: bonus.execute_threshold,
            });
        }
        intermediate_stats.clamp_caps();

        let resolve_ctx = ResolveContext::new(intermediate_stats);

        for bonus in cp_alt.iter().chain(passive_alt.iter()) {
            let chosen = bonus.resolve_ref(&resolve_ctx);
            let multiplier = Self::bonus_multiplier(bonus, skills);
            Self::apply_stat_bonus(&mut effective_stats, chosen.target, chosen.value * multiplier);
            resolved.push(ResolvedBonus {
                target: chosen.target,
                value: chosen.value,
                skill_line_filter: bonus.skill_line_filter,
                execute_threshold: bonus.execute_threshold,
            });
        }
        effective_stats.clamp_caps();

        let armor_factor =
            super::formulas::armor_damage_factor(effective_stats.target_armor, effective_stats.penetration);
        let crit_mult = super::formulas::critical_multiplier(
            effective_stats.critical_chance(),
            effective_stats.critical_damage,
        );

        let mut total = 0.0;
        for skill in skills {
            total += skill.calculate_damage_per_cast_fast(
                &resolved,
                &effective_stats,
                None,
                armor_factor,
                crit_mult,
            );
        }
        total
    }

    /// Build cached passive context for a given (skill combo, passive bonuses) pair.
    /// Pre-computes per-skill, per-hit/dot modifier sums from passive bonuses.
    /// Constant across all CP combos — computed once per skill combo.
    pub fn cache_passive_context(
        skills: &[&'static SkillData],
        passive_pre_resolved: &[ResolvedBonus],
        passive_ability_count: &[BonusData],
        character_stats: &CharacterStats,
    ) -> CachedPassiveContext {
        let default_ctx = ResolveContext::default();
        let mut base_stats = character_stats.clone();

        // Build passive resolved list and apply stat bonuses
        let mut passive_resolved: SmallVec<[ResolvedBonus; 16]> = SmallVec::new();

        for rb in passive_pre_resolved {
            Self::apply_stat_bonus(&mut base_stats, rb.target, rb.value);
            passive_resolved.push(*rb);
        }

        for bonus in passive_ability_count {
            let bv = bonus.resolve_ref(&default_ctx);
            let multiplier = Self::bonus_multiplier(bonus, skills);
            let applied = bv.value * multiplier;
            Self::apply_stat_bonus(&mut base_stats, bv.target, applied);
            passive_resolved.push(ResolvedBonus {
                target: bv.target,
                value: bv.value,
                skill_line_filter: bonus.skill_line_filter,
                execute_threshold: bonus.execute_threshold,
            });
        }

        // Pre-compute per-skill, per-component modifier sums from passive bonuses.
        // This is the expensive part that we avoid redoing per CP combo.
        let mut hit_mods: SmallVec<[SmallVec<[f64; 4]>; 10]> = SmallVec::new();
        let mut dot_mods: SmallVec<[SmallVec<[f64; 4]>; 10]> = SmallVec::new();

        for skill in skills {
            let skill_line = skill.skill_line;
            let mut skill_hit_mods: SmallVec<[f64; 4]> = SmallVec::new();
            let mut skill_dot_mods: SmallVec<[f64; 4]> = SmallVec::new();

            if let Some(damage) = &skill.damage {
                if let Some(hits) = &damage.hits {
                    for hit in hits {
                        let modifier: f64 = passive_resolved
                            .iter()
                            .filter(|b| {
                                b.skill_line_filter.map_or(true, |sl| sl == skill_line)
                                    && b.execute_threshold.is_none()
                                    && hit.flags.matches_bonus_target(b.target)
                            })
                            .map(|b| b.value)
                            .sum();
                        skill_hit_mods.push(modifier);
                    }
                }
                if let Some(dots) = &damage.dots {
                    for dot in dots {
                        let modifier: f64 = passive_resolved
                            .iter()
                            .filter(|b| {
                                b.skill_line_filter.map_or(true, |sl| sl == skill_line)
                                    && b.execute_threshold.is_none()
                                    && dot.flags.matches_bonus_target(b.target)
                            })
                            .map(|b| b.value)
                            .sum();
                        skill_dot_mods.push(modifier);
                    }
                }
            }

            hit_mods.push(skill_hit_mods);
            dot_mods.push(skill_dot_mods);
        }

        CachedPassiveContext {
            base_stats,
            hit_mods,
            dot_mods,
        }
    }

    /// Fast damage computation using cached passive context.
    /// Per-skill modifier sums from passive bonuses are pre-computed in the cache.
    /// Only iterates the small CP bonus list (~4-8 items) per skill per component,
    /// instead of the full ~30 bonus list.
    pub fn compute_total_damage_cached(
        skills: &[&'static SkillData],
        passive_ctx: &CachedPassiveContext,
        passive_alt: &[BonusData],
        cp_pre_resolved: &[ResolvedBonus],
        cp_ability_count: &[BonusData],
        cp_alt: &[BonusData],
    ) -> f64 {
        let default_ctx = ResolveContext::default();

        // Start from passive_ctx.base_stats (character_stats + passive stat bonuses).
        let mut intermediate_stats = passive_ctx.base_stats.clone();
        let mut effective_stats = passive_ctx.base_stats.clone();

        // Build small list of CP + alt resolved bonuses (only these are iterated per skill)
        let mut cp_resolved: SmallVec<[ResolvedBonus; 8]> = SmallVec::new();

        for rb in cp_pre_resolved {
            Self::apply_stat_bonus(&mut intermediate_stats, rb.target, rb.value);
            Self::apply_stat_bonus(&mut effective_stats, rb.target, rb.value);
            cp_resolved.push(*rb);
        }

        for bonus in cp_ability_count {
            let bv = bonus.resolve_ref(&default_ctx);
            let multiplier = Self::bonus_multiplier(bonus, skills);
            let applied = bv.value * multiplier;
            Self::apply_stat_bonus(&mut intermediate_stats, bv.target, applied);
            Self::apply_stat_bonus(&mut effective_stats, bv.target, applied);
            cp_resolved.push(ResolvedBonus {
                target: bv.target,
                value: bv.value,
                skill_line_filter: bonus.skill_line_filter,
                execute_threshold: bonus.execute_threshold,
            });
        }
        intermediate_stats.clamp_caps();

        // Resolve alternatives (both CP and passive) using intermediate_stats
        let resolve_ctx = ResolveContext::new(intermediate_stats);

        for bonus in cp_alt.iter().chain(passive_alt.iter()) {
            let chosen = bonus.resolve_ref(&resolve_ctx);
            let multiplier = Self::bonus_multiplier(bonus, skills);
            Self::apply_stat_bonus(&mut effective_stats, chosen.target, chosen.value * multiplier);
            cp_resolved.push(ResolvedBonus {
                target: chosen.target,
                value: chosen.value,
                skill_line_filter: bonus.skill_line_filter,
                execute_threshold: bonus.execute_threshold,
            });
        }
        effective_stats.clamp_caps();

        let armor_factor = super::formulas::armor_damage_factor(
            effective_stats.target_armor,
            effective_stats.penetration,
        );
        let crit_mult = super::formulas::critical_multiplier(
            effective_stats.critical_chance(),
            effective_stats.critical_damage,
        );

        let max_stat = effective_stats.max_stat();
        let max_power = effective_stats.max_power();

        // Inline damage calculation using cached passive modifiers + CP-only iteration
        let mut total = 0.0;

        for (skill_idx, skill) in skills.iter().enumerate() {
            let skill_line = skill.skill_line;
            let mut skill_damage = 0.0;

            if let Some(damage) = &skill.damage {
                if let Some(hits) = &damage.hits {
                    for (hit_idx, hit) in hits.iter().enumerate() {
                        // Skip hits gated by execute_threshold (enemy_health is always None)
                        if hit.execute_threshold.is_some() {
                            continue;
                        }

                        // CP modifier: iterate only the small CP resolved list
                        let cp_modifier: f64 = cp_resolved
                            .iter()
                            .filter(|b| {
                                b.skill_line_filter.map_or(true, |sl| sl == skill_line)
                                    && b.execute_threshold.is_none()
                                    && hit.flags.matches_bonus_target(b.target)
                            })
                            .map(|b| b.value)
                            .sum();

                        let total_modifier =
                            passive_ctx.hit_mods[skill_idx][hit_idx] + cp_modifier;
                        let hit_value = hit.effective_value(max_stat, max_power);
                        skill_damage += hit_value * (1.0 + total_modifier);
                    }
                }

                if let Some(dots) = &damage.dots {
                    for (dot_idx, dot) in dots.iter().enumerate() {
                        let cp_modifier: f64 = cp_resolved
                            .iter()
                            .filter(|b| {
                                b.skill_line_filter.map_or(true, |sl| sl == skill_line)
                                    && b.execute_threshold.is_none()
                                    && dot.flags.matches_bonus_target(b.target)
                            })
                            .map(|b| b.value)
                            .sum();

                        let total_modifier =
                            passive_ctx.dot_mods[skill_idx][dot_idx] + cp_modifier;
                        let dot_value = dot.effective_value(max_stat, max_power);

                        let interval = dot.interval.unwrap_or(dot.duration);
                        let ticks = (dot.duration / interval).floor() as i32;
                        let increase_per_tick = dot.increase_per_tick.unwrap_or(0.0);
                        let flat_increase_per_tick =
                            dot.flat_increase_per_tick.unwrap_or(0.0);

                        for i in 0..ticks {
                            let pct_mult = 1.0 + (i as f64) * increase_per_tick;
                            let flat_inc = (i as f64) * flat_increase_per_tick;
                            let tick_damage = dot_value * pct_mult + flat_inc;

                            if dot.ignores_modifier.unwrap_or(false) {
                                skill_damage += tick_damage;
                            } else {
                                skill_damage += tick_damage * (1.0 + total_modifier);
                            }
                        }
                    }
                }
            }

            // Skill-level execute: no-op when enemy_health is None
            total += skill_damage;
        }

        total * armor_factor * crit_mult
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
        self.resolved_bonuses
            .iter()
            .filter(|b| b.source == BonusSource::ChampionPointSlottable)
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
                "Total Damage per Cast: {}",
                format::format_number(self.total_damage_per_cast as u64) // FIXME: cast
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
                let (b, e) = fmt_pct(base.critical_chance(), eff.critical_chance());
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
            .resolved_bonuses
            .iter()
            .filter(|b| b.source == BonusSource::ChampionPointSlottable)
            .map(|b| b.name.as_str())
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
        // Compute passive-only bonuses for tooltip damage (excludes champion points)
        let passive_bonuses: Vec<_> = self
            .resolved_bonuses
            .iter()
            .filter(|b| b.source != BonusSource::ChampionPointSlottable)
            .cloned()
            .collect();
        let passive_stats = Self::apply_stat_bonuses_to_stats(
            &passive_bonuses,
            &self.character_stats,
            &self.skills,
        );

        let mut skills_with_damage: Vec<_> = self
            .skills
            .iter()
            .map(|skill| {
                let mut tooltip_stats = passive_stats.clone();
                tooltip_stats.target_armor = 0.0;
                tooltip_stats.penetration = 0.0;
                tooltip_stats.critical_rating = 0.0;
                tooltip_stats.critical_damage = 1.0;
                let tooltip =
                    skill.calculate_damage_per_cast(&passive_bonuses, &tooltip_stats, None);
                let effective = skill.calculate_damage_per_cast(
                    &self.resolved_bonuses,
                    &self.effective_stats,
                    None,
                );
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
                    table::ColumnDefinition::new("Eff. Damage", 12).align_right(),
                ],
                footer: Some("*Spammable skill".to_string()),
            },
        )
    }

    fn fmt_bonuses(&self) -> String {
        let fmt_bonus_value = |target: BonusTarget, value: f64| -> String {
            match target {
                BonusTarget::CriticalDamage
                | BonusTarget::Damage
                | BonusTarget::DirectDamage
                | BonusTarget::DotDamage
                | BonusTarget::AoeDamage
                | BonusTarget::SingleDamage
                | BonusTarget::FlameDamage
                | BonusTarget::FrostDamage
                | BonusTarget::ShockDamage
                | BonusTarget::PhysicalDamage
                | BonusTarget::EnemyDamageTaken
                | BonusTarget::StatusEffectChance
                | BonusTarget::StatusEffectDamage
                | BonusTarget::ChilledStatusEffectChance
                | BonusTarget::ChilledStatusEffectDamage
                | BonusTarget::BurningAndPoisonDamage
                | BonusTarget::HeavyAttackDamage
                | BonusTarget::OffBalanceDamage
                | BonusTarget::WeaponAndSpellDamageMultiplier
                | BonusTarget::DurationSkillLineMultiplier => {
                    format!("{:.1}%", value * 100.0)
                }
                _ => format::format_number(value as u64),
            }
        };

        let ctx = ResolveContext::new(self.effective_stats.clone());
        let mut resolved: Vec<_> = self
            .resolved_bonuses
            .iter()
            .map(|bonus| {
                let bv = bonus.resolve(&ctx);
                (bonus, bv)
            })
            .collect();
        resolved.sort_by(|a, b| a.1.target.to_string().cmp(&b.1.target.to_string()));

        let bonuses_data: Vec<Vec<String>> = resolved
            .iter()
            .enumerate()
            .map(|(i, (bonus, bv))| {
                let value_str = fmt_bonus_value(bv.target, bv.value);
                let multiplier = Self::bonus_multiplier(bonus, &self.skills);
                let multiplier_str = if multiplier > 1.0 {
                    format!("x{}", multiplier as u64)
                } else {
                    String::new()
                };
                vec![
                    (i + 1).to_string(),
                    bonus.name.clone(),
                    bonus.source.to_string(),
                    bv.target.to_string(),
                    value_str,
                    multiplier_str,
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
                    table::ColumnDefinition::new("Count", 6).align_right(),
                ],
                footer: None,
            },
        )
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
        write!(f, "{}", lines.join("\n"))
    }
}
