use super::{
    BonusData, BonusSource, BonusTarget, BonusTrigger, CharacterStats, ClassName, DamageFlags,
    ResolveContext, ResolvedBonus, SkillData, SkillLineName,
};
use crate::infrastructure::{format, table};
use smallvec::SmallVec;
use std::collections::{HashMap, HashSet};

/// Bit-indexed modifier lookup table. Pre-sums bonus values by BonusTarget so that
/// a DamageFlags → modifier query is O(1) instead of O(bonuses).
///
/// Each damage-relevant BonusTarget maps to exactly one DamageFlags bit:
///   Damage → always applies (no specific bit)
///   PhysicalDamage → bit 1, FlameDamage → bit 2, FrostDamage → bit 3,
///   ShockDamage → bit 4, SingleDamage → bit 8, AoeDamage → bit 9,
///   DirectDamage → bit 10, DotDamage → bit 11
pub(crate) struct ModifierLookup {
    damage_sum: f64,
    /// EnemyDamageTaken is a separate multiplicative layer in ESO's damage formula.
    enemy_damage_taken_sum: f64,
    bit_sums: [f64; 12],
}

impl ModifierLookup {
    /// Build lookup from bonuses, including only global bonuses
    /// (no skill_line_filter). Execute-threshold bonuses are included
    /// weighted by their threshold (proportion of fight time they are active).
    /// Filtered bonuses (with skill_line_filter) must be collected and handled
    /// separately by the caller.
    pub fn new(bonuses: &[ResolvedBonus]) -> Self {
        let mut damage_sum = 0.0;
        let mut enemy_damage_taken_sum = 0.0;
        let mut bit_sums = [0.0; 12];

        for b in bonuses {
            if b.skill_line_filter.is_some() {
                continue;
            }
            let value = if let Some(threshold) = b.execute_threshold {
                b.value * threshold
            } else {
                b.value
            };
            match b.target {
                BonusTarget::Damage => damage_sum += value,
                BonusTarget::EnemyDamageTaken => enemy_damage_taken_sum += value,
                BonusTarget::PhysicalDamage => bit_sums[1] += value,
                BonusTarget::FlameDamage => bit_sums[2] += value,
                BonusTarget::FrostDamage => bit_sums[3] += value,
                BonusTarget::ShockDamage => bit_sums[4] += value,
                BonusTarget::SingleDamage => bit_sums[8] += value,
                BonusTarget::AoeDamage => bit_sums[9] += value,
                BonusTarget::DirectDamage => bit_sums[10] += value,
                BonusTarget::DotDamage => bit_sums[11] += value,
                _ => {}
            }
        }

        Self {
            damage_sum,
            enemy_damage_taken_sum,
            bit_sums,
        }
    }

    /// Sum of EnemyDamageTaken bonuses (separate multiplicative layer).
    #[inline(always)]
    pub fn enemy_damage_taken(&self) -> f64 {
        self.enemy_damage_taken_sum
    }

    /// Look up the total damage-done modifier sum for a given DamageFlags pattern.
    /// Does NOT include EnemyDamageTaken (separate multiplicative layer).
    #[inline(always)]
    pub fn modifier_for(&self, flags: DamageFlags) -> f64 {
        let bits = flags.bits();
        let mut m = self.damage_sum;
        if bits & 0x0002 != 0 {
            m += self.bit_sums[1];
        } // PHYSICAL
        if bits & 0x0004 != 0 {
            m += self.bit_sums[2];
        } // FLAME
        if bits & 0x0008 != 0 {
            m += self.bit_sums[3];
        } // FROST
        if bits & 0x0010 != 0 {
            m += self.bit_sums[4];
        } // SHOCK
        if bits & 0x0100 != 0 {
            m += self.bit_sums[8];
        } // SINGLE_TARGET
        if bits & 0x0200 != 0 {
            m += self.bit_sums[9];
        } // AOE
        if bits & 0x0400 != 0 {
            m += self.bit_sums[10];
        } // DIRECT
        if bits & 0x0800 != 0 {
            m += self.bit_sums[11];
        } // DOT
        m
    }
}

/// Pre-computed passive bonus context for a given (skill combo, passive bonuses) pair.
/// Caches per-skill modifier sums from passive bonuses so the inner CP loop only
/// iterates the small CP bonus list (~4-8 items) instead of all ~30 bonuses.
pub struct CachedPassiveContext {
    /// character_stats + passive stat bonuses (unclamped, CP stats added on top per eval)
    pub base_stats: CharacterStats,
    /// Per skill, per hit: cached passive modifier sum (damage-done only)
    pub hit_mods: SmallVec<[SmallVec<[f64; 4]>; 10]>,
    /// Per skill, per dot: cached passive modifier sum (damage-done only)
    pub dot_mods: SmallVec<[SmallVec<[f64; 4]>; 10]>,
    /// Sum of EnemyDamageTaken from passive bonuses (separate multiplicative layer)
    pub enemy_damage_taken: f64,
}

/// Pre-computed evaluation context: values constant across skills for a given
/// (skill combo, bonus set) pair. Used by both the direct and cached paths.
pub(crate) struct EvalContext {
    pub armor_factor: f64,
    pub crit_mult: f64,
    /// Multiplier for EnemyDamageTaken from this context's bonuses.
    pub enemy_damage_taken: f64,
    pub max_stat: f64,
    pub max_power: f64,
    pub lookup: ModifierLookup,
    pub filtered: SmallVec<[ResolvedBonus; 4]>,
}

#[derive(Debug, Clone)]
pub struct Build {
    skills: Vec<&'static SkillData>,
    resolved_bonuses: Vec<BonusData>,
    cp_bonuses: Vec<BonusData>,
    passive_bonuses: Vec<BonusData>,
    extra_bonuses: Vec<BonusData>,
    character_stats: CharacterStats,
    effective_stats: CharacterStats,
    set_names: Vec<(String, u8)>,
    pub total_damage_per_cast: f64,
}

// Constructor
impl Build {
    pub fn new(
        skills: Vec<&'static SkillData>,
        cp_bonuses: &[BonusData],
        passive_bonuses: &[BonusData],
        set_bonuses: &[BonusData],
        set_names: Vec<(String, u8)>,
        character_stats: CharacterStats,
    ) -> Self {
        Self::new_with_extra(
            skills,
            cp_bonuses,
            passive_bonuses,
            set_bonuses,
            set_names,
            character_stats,
            &[],
        )
    }

    pub fn new_with_extra(
        skills: Vec<&'static SkillData>,
        cp_bonuses: &[BonusData],
        passive_bonuses: &[BonusData],
        set_bonuses: &[BonusData],
        set_names: Vec<(String, u8)>,
        character_stats: CharacterStats,
        extra_bonuses: &[BonusData],
    ) -> Self {
        // FIXME: some passives are only active while on that bar,
        // do we wanna apply combination here too?

        // Collect extra bonus names to suppress duplicates from other sources.
        // In a trial environment the dummy provides buffs like Minor Berserk;
        // if a class passive also grants it, only the trial version should apply.
        let extra_names: HashSet<&str> = extra_bonuses.iter().map(|b| b.name.as_str()).collect();

        let mut simple_bonuses: Vec<BonusData> = Vec::new();
        let mut alt_bonuses: Vec<BonusData> = Vec::new();
        for bonus in cp_bonuses
            .iter()
            .chain(passive_bonuses.iter())
            .chain(set_bonuses.iter())
        {
            if !extra_names.is_empty() && extra_names.contains(bonus.name.as_str()) {
                continue; // suppressed by extra bonus with the same name
            }
            if bonus.has_alternative() {
                alt_bonuses.push(bonus.clone());
            } else {
                simple_bonuses.push(bonus.clone());
            }
        }
        for bonus in extra_bonuses.iter().cloned() {
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
            cp_bonuses: cp_bonuses.to_vec(),
            passive_bonuses: passive_bonuses.to_vec(),
            extra_bonuses: extra_bonuses.to_vec(),
            character_stats,
            effective_stats,
            set_names,
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
        // Pass 1: flat bonuses
        for bonus in bonuses {
            let bonus_value = bonus.resolve(&ctx);
            let bonus_multiplier = Self::bonus_multiplier(bonus, skills);
            Self::apply_flat_stat_bonus(
                &mut stats,
                bonus_value.target,
                bonus_value.value * bonus_multiplier,
            );
        }
        // Pass 2: percentage bonuses (applied on top of base + flat)
        for bonus in bonuses {
            let bonus_value = bonus.resolve(&ctx);
            let bonus_multiplier = Self::bonus_multiplier(bonus, skills);
            Self::apply_pct_stat_bonus(
                &mut stats,
                bonus_value.target,
                bonus_value.value * bonus_multiplier,
            );
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

    fn apply_flat_stat_bonus(stats: &mut CharacterStats, target: BonusTarget, value: f64) {
        match target {
            BonusTarget::WeaponAndSpellDamageFlat => {
                stats.weapon_damage += value;
                stats.spell_damage += value;
            }
            BonusTarget::WeaponDamageFlat => {
                stats.weapon_damage += value;
            }
            BonusTarget::SpellDamageFlat => {
                stats.spell_damage += value;
            }
            BonusTarget::MaxMagickaFlat => {
                stats.max_magicka += value;
            }
            BonusTarget::MaxStaminaFlat => {
                stats.max_stamina += value;
            }
            BonusTarget::CriticalDamage => {
                stats.critical_damage += value;
            }
            BonusTarget::CriticalRating => {
                stats.critical_rating += value;
            }
            BonusTarget::WeaponCriticalRating | BonusTarget::SpellCriticalRating => {
                stats.critical_rating += value;
            }
            BonusTarget::PhysicalAndSpellPenetration => {
                stats.penetration += value;
            }
            BonusTarget::EnemyResistanceReduction => {
                stats.penetration += value;
            }
            _ => {}
        }
    }

    fn apply_pct_stat_bonus(stats: &mut CharacterStats, target: BonusTarget, value: f64) {
        match target {
            BonusTarget::MaxMagicka => {
                stats.max_magicka *= 1.0 + value;
            }
            BonusTarget::MaxStamina => {
                stats.max_stamina *= 1.0 + value;
            }
            BonusTarget::WeaponDamage => {
                stats.weapon_damage *= 1.0 + value;
            }
            BonusTarget::SpellDamage => {
                stats.spell_damage *= 1.0 + value;
            }
            BonusTarget::WeaponAndSpellDamageMultiplier => {
                stats.weapon_damage *= 1.0 + value;
                stats.spell_damage *= 1.0 + value;
            }
            _ => {}
        }
    }

    fn is_pct_stat_bonus(target: BonusTarget) -> bool {
        matches!(
            target,
            BonusTarget::MaxMagicka
                | BonusTarget::MaxStamina
                | BonusTarget::WeaponDamage
                | BonusTarget::SpellDamage
                | BonusTarget::WeaponAndSpellDamageMultiplier
        )
    }

    fn apply_or_defer(
        stats: &mut CharacterStats,
        deferred: &mut SmallVec<[(BonusTarget, f64); 8]>,
        target: BonusTarget,
        value: f64,
    ) {
        Self::apply_flat_stat_bonus(stats, target, value);
        if Self::is_pct_stat_bonus(target) {
            deferred.push((target, value));
        }
    }

    fn apply_deferred_pct(stats: &mut CharacterStats, deferred: &[(BonusTarget, f64)]) {
        for &(target, value) in deferred {
            Self::apply_pct_stat_bonus(stats, target, value);
        }
    }

    /// Build evaluation context from all bonuses (direct path).
    /// Contains pre-computed stats, modifier lookup, and filtered bonuses.
    pub(crate) fn compute_eval_context(
        skills: &[&'static SkillData],
        cp_pre_resolved: &[ResolvedBonus],
        cp_ability_count: &[BonusData],
        cp_alt: &[BonusData],
        passive_pre_resolved: &[ResolvedBonus],
        passive_ability_count: &[BonusData],
        passive_alt: &[BonusData],
        character_stats: &CharacterStats,
    ) -> EvalContext {
        let default_ctx = ResolveContext::default();

        let mut intermediate_stats = character_stats.clone();
        let mut effective_stats = character_stats.clone();
        let mut resolved: SmallVec<[ResolvedBonus; 24]> = SmallVec::new();
        let mut deferred_intermediate: SmallVec<[(BonusTarget, f64); 8]> = SmallVec::new();
        let mut deferred_effective: SmallVec<[(BonusTarget, f64); 8]> = SmallVec::new();

        for rb in cp_pre_resolved.iter().chain(passive_pre_resolved.iter()) {
            Self::apply_or_defer(
                &mut intermediate_stats,
                &mut deferred_intermediate,
                rb.target,
                rb.value,
            );
            Self::apply_or_defer(
                &mut effective_stats,
                &mut deferred_effective,
                rb.target,
                rb.value,
            );
            resolved.push(*rb);
        }

        for bonus in cp_ability_count.iter().chain(passive_ability_count.iter()) {
            let bv = bonus.resolve_ref(&default_ctx);
            let multiplier = Self::bonus_multiplier(bonus, skills);
            let applied = bv.value * multiplier;
            Self::apply_or_defer(
                &mut intermediate_stats,
                &mut deferred_intermediate,
                bv.target,
                applied,
            );
            Self::apply_or_defer(
                &mut effective_stats,
                &mut deferred_effective,
                bv.target,
                applied,
            );
            resolved.push(ResolvedBonus {
                target: bv.target,
                value: bv.value,
                skill_line_filter: bonus.skill_line_filter,
                execute_threshold: bonus.execute_threshold,
            });
        }
        Self::apply_deferred_pct(&mut intermediate_stats, &deferred_intermediate);
        intermediate_stats.clamp_caps();

        let resolve_ctx = ResolveContext::new(intermediate_stats);

        for bonus in cp_alt.iter().chain(passive_alt.iter()) {
            let chosen = bonus.resolve_ref(&resolve_ctx);
            let multiplier = Self::bonus_multiplier(bonus, skills);
            let applied = chosen.value * multiplier;
            Self::apply_or_defer(
                &mut effective_stats,
                &mut deferred_effective,
                chosen.target,
                applied,
            );
            resolved.push(ResolvedBonus {
                target: chosen.target,
                value: chosen.value,
                skill_line_filter: bonus.skill_line_filter,
                execute_threshold: bonus.execute_threshold,
            });
        }
        Self::apply_deferred_pct(&mut effective_stats, &deferred_effective);
        effective_stats.clamp_caps();

        let armor_factor = super::formulas::armor_damage_factor(
            effective_stats.target_armor,
            effective_stats.penetration,
        );
        let crit_mult = super::formulas::critical_multiplier(
            effective_stats.critical_chance(),
            effective_stats.critical_damage,
        );

        let lookup = ModifierLookup::new(&resolved);
        let filtered: SmallVec<[ResolvedBonus; 4]> = resolved
            .iter()
            .filter(|b| b.skill_line_filter.is_some())
            .map(|b| {
                if let Some(threshold) = b.execute_threshold {
                    ResolvedBonus {
                        value: b.value * threshold,
                        execute_threshold: None,
                        ..*b
                    }
                } else {
                    *b
                }
            })
            .collect();

        let max_stat = effective_stats.max_stat();
        let max_power = effective_stats.max_power();

        let enemy_damage_taken = lookup.enemy_damage_taken();

        EvalContext {
            armor_factor,
            crit_mult,
            enemy_damage_taken,
            max_stat,
            max_power,
            lookup,
            filtered,
        }
    }

    /// Compute raw damage for a single skill (before armor_factor * crit_mult).
    /// Execute-threshold hits are included weighted by their threshold (proportion
    /// of fight time they are active). Skill-level execute average multiplier is
    /// applied to the total.
    #[inline]
    pub(crate) fn single_skill_damage(skill: &SkillData, ctx: &EvalContext) -> f64 {
        let skill_line = skill.skill_line;
        let mut skill_damage = 0.0;

        if let Some(damage) = &skill.damage {
            if let Some(hits) = &damage.hits {
                for hit in hits {
                    let modifier = ctx.lookup.modifier_for(hit.flags)
                        + Self::filtered_modifier(&ctx.filtered, skill_line, hit.flags);
                    let hit_value = hit.effective_value(ctx.max_stat, ctx.max_power);
                    let hit_dmg = hit_value * (1.0 + modifier);
                    if let Some(threshold) = hit.execute_threshold {
                        skill_damage += hit_dmg * threshold;
                    } else {
                        skill_damage += hit_dmg;
                    }
                }
            }

            if let Some(dots) = &damage.dots {
                for dot in dots {
                    let modifier = ctx.lookup.modifier_for(dot.flags)
                        + Self::filtered_modifier(&ctx.filtered, skill_line, dot.flags);
                    let dot_value = dot.effective_value(ctx.max_stat, ctx.max_power);

                    let interval = dot.interval.unwrap_or(dot.duration);
                    let ticks = (dot.duration / interval).floor() as i32;
                    let increase_per_tick = dot.increase_per_tick.unwrap_or(0.0);
                    let flat_increase_per_tick = dot.flat_increase_per_tick.unwrap_or(0.0);

                    for i in 0..ticks {
                        let pct_mult = 1.0 + (i as f64) * increase_per_tick;
                        let flat_inc = (i as f64) * flat_increase_per_tick;
                        let tick_damage = dot_value * pct_mult + flat_inc;

                        if dot.ignores_modifier.unwrap_or(false) {
                            skill_damage += tick_damage;
                        } else {
                            skill_damage += tick_damage * (1.0 + modifier);
                        }
                    }
                }
            }
        }

        if let Some(execute) = &skill.execute {
            skill_damage *= execute.average_multiplier();
        }

        skill_damage
    }

    /// Fast damage computation without caching. Used when there is only 1 CP combo.
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
        let ctx = Self::compute_eval_context(
            skills,
            cp_pre_resolved,
            cp_ability_count,
            cp_alt,
            passive_pre_resolved,
            passive_ability_count,
            passive_alt,
            character_stats,
        );
        let mut total = 0.0;
        for skill in skills {
            total += Self::single_skill_damage(skill, &ctx);
        }
        total * ctx.armor_factor * ctx.crit_mult * (1.0 + ctx.enemy_damage_taken)
    }

    /// Sum modifier values from filtered bonuses (those with skill_line_filter)
    /// for a specific skill line and damage flags.
    #[inline(always)]
    fn filtered_modifier(
        filtered: &[ResolvedBonus],
        skill_line: SkillLineName,
        flags: DamageFlags,
    ) -> f64 {
        filtered
            .iter()
            .filter(|b| {
                b.skill_line_filter.map_or(true, |sl| sl == skill_line)
                    && flags.matches_bonus_target(b.target)
            })
            .map(|b| b.value)
            .sum()
    }

    /// Build cached passive context for a given (skill combo, passive bonuses) pair.
    /// Pre-computes per-skill, per-hit/dot modifier sums from passive bonuses.
    /// Constant across all CP combos - computed once per skill combo.
    pub fn cache_passive_context(
        skills: &[&'static SkillData],
        passive_pre_resolved: &[ResolvedBonus],
        passive_ability_count: &[BonusData],
        character_stats: &CharacterStats,
    ) -> CachedPassiveContext {
        let default_ctx = ResolveContext::default();
        let mut base_stats = character_stats.clone();
        let mut deferred: SmallVec<[(BonusTarget, f64); 8]> = SmallVec::new();

        // Build passive resolved list and apply stat bonuses
        let mut passive_resolved: SmallVec<[ResolvedBonus; 16]> = SmallVec::new();

        for rb in passive_pre_resolved {
            Self::apply_or_defer(&mut base_stats, &mut deferred, rb.target, rb.value);
            passive_resolved.push(*rb);
        }

        for bonus in passive_ability_count {
            let bv = bonus.resolve_ref(&default_ctx);
            let multiplier = Self::bonus_multiplier(bonus, skills);
            let applied = bv.value * multiplier;
            Self::apply_or_defer(&mut base_stats, &mut deferred, bv.target, applied);
            passive_resolved.push(ResolvedBonus {
                target: bv.target,
                value: bv.value,
                skill_line_filter: bonus.skill_line_filter,
                execute_threshold: bonus.execute_threshold,
            });
        }
        Self::apply_deferred_pct(&mut base_stats, &deferred);

        // Pre-compute per-skill, per-component modifier sums from passive bonuses
        // using the lookup table. Filtered bonuses (with skill_line_filter) are added
        // per-skill since they only apply to specific skill lines.
        let passive_lookup = ModifierLookup::new(&passive_resolved);
        let passive_filtered: SmallVec<[ResolvedBonus; 4]> = passive_resolved
            .iter()
            .filter(|b| b.skill_line_filter.is_some())
            .map(|b| {
                if let Some(threshold) = b.execute_threshold {
                    ResolvedBonus {
                        value: b.value * threshold,
                        execute_threshold: None,
                        ..*b
                    }
                } else {
                    *b
                }
            })
            .collect();

        let mut hit_mods: SmallVec<[SmallVec<[f64; 4]>; 10]> = SmallVec::new();
        let mut dot_mods: SmallVec<[SmallVec<[f64; 4]>; 10]> = SmallVec::new();

        for skill in skills {
            let skill_line = skill.skill_line;
            let mut skill_hit_mods: SmallVec<[f64; 4]> = SmallVec::new();
            let mut skill_dot_mods: SmallVec<[f64; 4]> = SmallVec::new();

            if let Some(damage) = &skill.damage {
                if let Some(hits) = &damage.hits {
                    for hit in hits {
                        let modifier = passive_lookup.modifier_for(hit.flags)
                            + Self::filtered_modifier(&passive_filtered, skill_line, hit.flags);
                        skill_hit_mods.push(modifier);
                    }
                }
                if let Some(dots) = &damage.dots {
                    for dot in dots {
                        let modifier = passive_lookup.modifier_for(dot.flags)
                            + Self::filtered_modifier(&passive_filtered, skill_line, dot.flags);
                        skill_dot_mods.push(modifier);
                    }
                }
            }

            hit_mods.push(skill_hit_mods);
            dot_mods.push(skill_dot_mods);
        }

        let enemy_damage_taken = passive_lookup.enemy_damage_taken();

        CachedPassiveContext {
            base_stats,
            hit_mods,
            dot_mods,
            enemy_damage_taken,
        }
    }

    /// Build evaluation context from CP bonuses (cached path).
    /// Uses pre-computed passive base_stats from CachedPassiveContext.
    pub(crate) fn compute_cp_eval_context(
        skills: &[&'static SkillData],
        passive_ctx: &CachedPassiveContext,
        passive_alt: &[BonusData],
        cp_pre_resolved: &[ResolvedBonus],
        cp_ability_count: &[BonusData],
        cp_alt: &[BonusData],
    ) -> EvalContext {
        let default_ctx = ResolveContext::default();

        let mut intermediate_stats = passive_ctx.base_stats.clone();
        let mut effective_stats = passive_ctx.base_stats.clone();
        let mut cp_resolved: SmallVec<[ResolvedBonus; 8]> = SmallVec::new();
        let mut deferred_intermediate: SmallVec<[(BonusTarget, f64); 8]> = SmallVec::new();
        let mut deferred_effective: SmallVec<[(BonusTarget, f64); 8]> = SmallVec::new();

        for rb in cp_pre_resolved {
            Self::apply_or_defer(
                &mut intermediate_stats,
                &mut deferred_intermediate,
                rb.target,
                rb.value,
            );
            Self::apply_or_defer(
                &mut effective_stats,
                &mut deferred_effective,
                rb.target,
                rb.value,
            );
            cp_resolved.push(*rb);
        }

        for bonus in cp_ability_count {
            let bv = bonus.resolve_ref(&default_ctx);
            let multiplier = Self::bonus_multiplier(bonus, skills);
            let applied = bv.value * multiplier;
            Self::apply_or_defer(
                &mut intermediate_stats,
                &mut deferred_intermediate,
                bv.target,
                applied,
            );
            Self::apply_or_defer(
                &mut effective_stats,
                &mut deferred_effective,
                bv.target,
                applied,
            );
            cp_resolved.push(ResolvedBonus {
                target: bv.target,
                value: bv.value,
                skill_line_filter: bonus.skill_line_filter,
                execute_threshold: bonus.execute_threshold,
            });
        }
        Self::apply_deferred_pct(&mut intermediate_stats, &deferred_intermediate);
        intermediate_stats.clamp_caps();

        let resolve_ctx = ResolveContext::new(intermediate_stats);

        for bonus in cp_alt.iter().chain(passive_alt.iter()) {
            let chosen = bonus.resolve_ref(&resolve_ctx);
            let multiplier = Self::bonus_multiplier(bonus, skills);
            let applied = chosen.value * multiplier;
            Self::apply_or_defer(
                &mut effective_stats,
                &mut deferred_effective,
                chosen.target,
                applied,
            );
            cp_resolved.push(ResolvedBonus {
                target: chosen.target,
                value: chosen.value,
                skill_line_filter: bonus.skill_line_filter,
                execute_threshold: bonus.execute_threshold,
            });
        }
        Self::apply_deferred_pct(&mut effective_stats, &deferred_effective);
        effective_stats.clamp_caps();

        let armor_factor = super::formulas::armor_damage_factor(
            effective_stats.target_armor,
            effective_stats.penetration,
        );
        let crit_mult = super::formulas::critical_multiplier(
            effective_stats.critical_chance(),
            effective_stats.critical_damage,
        );

        let lookup = ModifierLookup::new(&cp_resolved);
        let filtered: SmallVec<[ResolvedBonus; 4]> = cp_resolved
            .iter()
            .filter(|b| b.skill_line_filter.is_some())
            .map(|b| {
                if let Some(threshold) = b.execute_threshold {
                    ResolvedBonus {
                        value: b.value * threshold,
                        execute_threshold: None,
                        ..*b
                    }
                } else {
                    *b
                }
            })
            .collect();

        let enemy_damage_taken = lookup.enemy_damage_taken();
        let max_stat = effective_stats.max_stat();
        let max_power = effective_stats.max_power();

        EvalContext {
            armor_factor,
            crit_mult,
            enemy_damage_taken,
            max_stat,
            max_power,
            lookup,
            filtered,
        }
    }

    /// Compute raw damage for a single skill using cached passive mods + CP eval context.
    /// Execute-threshold hits are included weighted by their threshold.
    /// Skill-level execute average multiplier is applied to the total.
    #[inline]
    pub(crate) fn single_skill_damage_cached(
        skill: &SkillData,
        skill_idx: usize,
        passive_ctx: &CachedPassiveContext,
        cp_ctx: &EvalContext,
    ) -> f64 {
        let skill_line = skill.skill_line;
        let mut skill_damage = 0.0;

        if let Some(damage) = &skill.damage {
            if let Some(hits) = &damage.hits {
                for (hit_idx, hit) in hits.iter().enumerate() {
                    let cp_modifier = cp_ctx.lookup.modifier_for(hit.flags)
                        + Self::filtered_modifier(&cp_ctx.filtered, skill_line, hit.flags);
                    let total_modifier = passive_ctx.hit_mods[skill_idx][hit_idx] + cp_modifier;
                    let hit_value = hit.effective_value(cp_ctx.max_stat, cp_ctx.max_power);
                    let hit_dmg = hit_value * (1.0 + total_modifier);
                    if let Some(threshold) = hit.execute_threshold {
                        skill_damage += hit_dmg * threshold;
                    } else {
                        skill_damage += hit_dmg;
                    }
                }
            }

            if let Some(dots) = &damage.dots {
                for (dot_idx, dot) in dots.iter().enumerate() {
                    let cp_modifier = cp_ctx.lookup.modifier_for(dot.flags)
                        + Self::filtered_modifier(&cp_ctx.filtered, skill_line, dot.flags);
                    let total_modifier = passive_ctx.dot_mods[skill_idx][dot_idx] + cp_modifier;
                    let dot_value = dot.effective_value(cp_ctx.max_stat, cp_ctx.max_power);

                    let interval = dot.interval.unwrap_or(dot.duration);
                    let ticks = (dot.duration / interval).floor() as i32;
                    let increase_per_tick = dot.increase_per_tick.unwrap_or(0.0);
                    let flat_increase_per_tick = dot.flat_increase_per_tick.unwrap_or(0.0);

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

        if let Some(execute) = &skill.execute {
            skill_damage *= execute.average_multiplier();
        }

        skill_damage
    }

    /// Fast damage computation using cached passive context.
    pub fn compute_total_damage_cached(
        skills: &[&'static SkillData],
        passive_ctx: &CachedPassiveContext,
        passive_alt: &[BonusData],
        cp_pre_resolved: &[ResolvedBonus],
        cp_ability_count: &[BonusData],
        cp_alt: &[BonusData],
    ) -> f64 {
        let cp_ctx = Self::compute_cp_eval_context(
            skills,
            passive_ctx,
            passive_alt,
            cp_pre_resolved,
            cp_ability_count,
            cp_alt,
        );
        let mut total = 0.0;
        for (i, skill) in skills.iter().enumerate() {
            total += Self::single_skill_damage_cached(skill, i, passive_ctx, &cp_ctx);
        }
        let enemy_damage_taken = passive_ctx.enemy_damage_taken + cp_ctx.enemy_damage_taken;
        total * cp_ctx.armor_factor * cp_ctx.crit_mult * (1.0 + enemy_damage_taken)
    }

    /// Build passive modifier lookup (constant within a work unit).
    /// Returns (ModifierLookup, filtered_bonuses) for incremental passive mod updates.
    pub(crate) fn compute_passive_lookup(
        passive_pre_resolved: &[ResolvedBonus],
        passive_ability_count: &[BonusData],
    ) -> (ModifierLookup, SmallVec<[ResolvedBonus; 4]>) {
        let default_ctx = ResolveContext::default();
        let mut passive_resolved: SmallVec<[ResolvedBonus; 16]> = SmallVec::new();

        for rb in passive_pre_resolved {
            passive_resolved.push(*rb);
        }
        for bonus in passive_ability_count {
            let bv = bonus.resolve_ref(&default_ctx);
            passive_resolved.push(ResolvedBonus {
                target: bv.target,
                value: bv.value,
                skill_line_filter: bonus.skill_line_filter,
                execute_threshold: bonus.execute_threshold,
            });
        }

        let lookup = ModifierLookup::new(&passive_resolved);
        let filtered: SmallVec<[ResolvedBonus; 4]> = passive_resolved
            .iter()
            .filter(|b| b.skill_line_filter.is_some())
            .map(|b| {
                if let Some(threshold) = b.execute_threshold {
                    ResolvedBonus {
                        value: b.value * threshold,
                        execute_threshold: None,
                        ..*b
                    }
                } else {
                    *b
                }
            })
            .collect();
        (lookup, filtered)
    }

    /// Update passive modifier cache for a single skill position.
    /// Used for incremental evaluation when only 1-2 skills change between combos.
    pub(crate) fn update_passive_mod(
        skill: &SkillData,
        skill_idx: usize,
        passive_lookup: &ModifierLookup,
        passive_filtered: &[ResolvedBonus],
        ctx: &mut CachedPassiveContext,
    ) {
        let skill_line = skill.skill_line;
        ctx.hit_mods[skill_idx].clear();
        ctx.dot_mods[skill_idx].clear();

        if let Some(damage) = &skill.damage {
            if let Some(hits) = &damage.hits {
                for hit in hits {
                    let modifier = passive_lookup.modifier_for(hit.flags)
                        + Self::filtered_modifier(passive_filtered, skill_line, hit.flags);
                    ctx.hit_mods[skill_idx].push(modifier);
                }
            }
            if let Some(dots) = &damage.dots {
                for dot in dots {
                    let modifier = passive_lookup.modifier_for(dot.flags)
                        + Self::filtered_modifier(passive_filtered, skill_line, dot.flags);
                    ctx.dot_mods[skill_idx].push(modifier);
                }
            }
        }
    }
}

// Public getters
impl Build {
    pub fn skills(&self) -> &[&'static SkillData] {
        &self.skills
    }

    /// Get skill names for export
    pub fn skill_names(&self) -> Vec<String> {
        self.skills.iter().map(|s| s.name.clone()).collect()
    }

    pub fn effective_stats(&self) -> &CharacterStats {
        &self.effective_stats
    }

    pub fn resolved_bonuses(&self) -> &[BonusData] {
        &self.resolved_bonuses
    }

    /// Get set names and piece counts
    pub fn set_names(&self) -> &[(String, u8)] {
        &self.set_names
    }

    pub fn cp_bonuses(&self) -> &[BonusData] {
        &self.cp_bonuses
    }

    pub fn passive_bonuses(&self) -> &[BonusData] {
        &self.passive_bonuses
    }

    pub fn extra_bonuses(&self) -> &[BonusData] {
        &self.extra_bonuses
    }

    pub fn character_stats(&self) -> &CharacterStats {
        &self.character_stats
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

        let mut lines = vec![
            format!("Classes: {}", class_names.join(", ")),
            format!("Class Skill Lines: {}", class_skill_lines.join(", ")),
            format!("Weapon Skill Lines: {}", weapon_skill_lines.join(", ")),
            format!("Champion Points: {}", champion_point_names.join(", ")),
        ];

        if !self.set_names.is_empty() {
            let formatted: Vec<String> = self
                .set_names
                .iter()
                .map(|(name, pieces)| format!("{} ({}pc)", name, pieces))
                .collect();
            lines.push(format!("Sets: {}", formatted.join(", ")));
        }

        lines
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
                let type_str = if skill.spammable && skill.execute.is_some() {
                    format!("{} **", skill.mechanic())
                } else if skill.spammable {
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
                footer: Some("*Spammable skill  **Finisher skill".to_string()),
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
                | BonusTarget::MaxMagicka
                | BonusTarget::MaxStamina
                | BonusTarget::WeaponDamage
                | BonusTarget::SpellDamage
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
