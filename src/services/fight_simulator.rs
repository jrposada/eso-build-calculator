use crate::data::light_attacks::light_attack_for_weapon;
use crate::domain::simulation::{BAR_SWAP_DELAY, GCD, TRIAL_DUMMY_HP};
use crate::domain::{
    ActiveBar, ActiveBuff, ActiveEffect, BonusData, BonusTarget, BonusTrigger, CharacterStats,
    DamageFlags, ResolveContext, SimulationResult, SkillBreakdown, SkillData, SkillLineName,
};
use std::collections::HashMap;

use super::bar_distribution::BarDistribution;

pub struct FightSimulator {
    pub target_hp: f64,
    pub effective_stats: CharacterStats,
    pub resolved_bonuses: Vec<BonusData>,
    pub armor_factor: f64,
    pub crit_mult: f64,
}

struct SimState {
    time: f64,
    remaining_hp: f64,
    active_bar: ActiveBar,
    active_effects: Vec<ActiveEffect>,
    active_buffs: Vec<ActiveBuff>,
    gcd_ready: f64,
    // Proc tracking: skill name -> accumulated light attack count
    proc_counters: HashMap<String, u32>,
    // Tracking
    skill_damage: HashMap<String, (f64, u32)>,
    la_damage: f64,
    la_count: u32,
    bar_swap_count: u32,
}

/// Pre-computed stats with active buffs applied.
struct BuffedContext {
    max_stat: f64,
    max_power: f64,
    armor_factor: f64,
    crit_mult: f64,
}

#[derive(Debug)]
enum Action {
    CastSkill(usize),
    BarSwap,
}

impl FightSimulator {
    pub fn new(
        effective_stats: &CharacterStats,
        resolved_bonuses: &[BonusData],
    ) -> Self {
        let armor_factor = crate::domain::formulas::armor_damage_factor(
            effective_stats.target_armor,
            effective_stats.penetration,
        );
        let crit_mult = crate::domain::formulas::critical_multiplier(
            effective_stats.critical_chance(),
            effective_stats.critical_damage,
        );

        Self {
            target_hp: TRIAL_DUMMY_HP,
            effective_stats: effective_stats.clone(),
            resolved_bonuses: resolved_bonuses.to_vec(),
            armor_factor,
            crit_mult,
        }
    }

    pub fn simulate(&self, distribution: &BarDistribution) -> SimulationResult {
        // Initialize proc counters for all proc skills on both bars
        let mut proc_counters = HashMap::new();
        for skill in distribution
            .bar1
            .skills
            .iter()
            .chain(distribution.bar2.skills.iter())
        {
            if skill.proc_light_attacks.is_some() {
                proc_counters.entry(skill.name.clone()).or_insert(0);
            }
        }

        let mut state = SimState {
            time: 0.0,
            remaining_hp: self.target_hp,
            active_bar: ActiveBar::Bar1,
            active_effects: Vec::new(),
            active_buffs: Vec::new(),
            gcd_ready: 0.0,
            proc_counters,
            skill_damage: HashMap::new(),
            la_damage: 0.0,
            la_count: 0,
            bar_swap_count: 0,
        };

        // Register permanent AbilitySlotted buffs from all skills on both bars
        self.register_ability_slotted_buffs(&mut state, distribution);

        // Safety: prevent infinite loops
        let max_iterations = 1_000_000;
        let mut iterations = 0;

        while state.remaining_hp > 0.0 && iterations < max_iterations {
            iterations += 1;

            // Advance time to next GCD
            let target_time = state.gcd_ready;
            if state.time < target_time {
                self.advance_time(&mut state, target_time);
            }

            let current_skills = match state.active_bar {
                ActiveBar::Bar1 => &distribution.bar1.skills,
                ActiveBar::Bar2 => &distribution.bar2.skills,
            };
            let other_skills = match state.active_bar {
                ActiveBar::Bar1 => &distribution.bar2.skills,
                ActiveBar::Bar2 => &distribution.bar1.skills,
            };

            let action = self.decide_action(&state, current_skills, other_skills);

            match action {
                Action::CastSkill(skill_idx) => {
                    let skill = current_skills[skill_idx];
                    let current_weapon = match state.active_bar {
                        ActiveBar::Bar1 => distribution.bar1.weapon_type,
                        ActiveBar::Bar2 => distribution.bar2.weapon_type,
                    };

                    // Compute buffed context from current active buffs
                    let buffed = self.compute_buffed_context(&state.active_buffs);

                    let health_pct = state.remaining_hp / self.target_hp;

                    // 1. Light attack damage (uses current buffs)
                    let la_data = light_attack_for_weapon(current_weapon);
                    let la_modifier = self.compute_modifier_for_flags(la_data.flags, None, health_pct)
                        + self.compute_buff_modifier_for_flags(la_data.flags, &state.active_buffs);
                    let la_dmg = la_data.calculate_damage(
                        la_modifier,
                        buffed.max_stat,
                        buffed.max_power,
                        buffed.armor_factor,
                        buffed.crit_mult,
                    );
                    state.remaining_hp -= la_dmg;
                    state.la_damage += la_dmg;
                    state.la_count += 1;

                    // Increment all proc counters on every light attack
                    for counter in state.proc_counters.values_mut() {
                        *counter += 1;
                    }

                    // 2. Skill hit damage (instant portion), gated by proc requirement
                    let hit_dmg = if let Some(threshold) = skill.proc_light_attacks {
                        let counter = state.proc_counters.get(&skill.name).copied().unwrap_or(0);
                        if counter >= threshold {
                            let dmg = self.calc_skill_hits(skill, &buffed, &state.active_buffs, health_pct);
                            state.proc_counters.insert(skill.name.clone(), 0);
                            dmg
                        } else {
                            0.0
                        }
                    } else {
                        self.calc_skill_hits(skill, &buffed, &state.active_buffs, health_pct)
                    };
                    state.remaining_hp -= hit_dmg;

                    let entry = state
                        .skill_damage
                        .entry(skill.name.clone())
                        .or_insert((0.0, 0));
                    entry.0 += hit_dmg;
                    entry.1 += 1;

                    // 3. Register/refresh DoTs as active effects (snapshot at cast time)
                    if let Some(damage) = &skill.damage {
                        if let Some(dots) = &damage.dots {
                            for dot in dots {
                                let base_value = dot.effective_value(buffed.max_stat, buffed.max_power);
                                let interval = dot.interval.unwrap_or(dot.duration);
                                let total_ticks = (dot.duration / interval).floor() as i32;
                                let delay = dot.delay.unwrap_or(0.0);

                                // Snapshot modifier at cast time (includes execute bonuses if currently in range)
                                let snapshotted_modifier = if dot.ignores_modifier.unwrap_or(false) {
                                    0.0
                                } else {
                                    self.compute_modifier_for_flags(dot.flags, Some(skill.skill_line), health_pct)
                                        + self.compute_buff_modifier_for_flags(dot.flags, &state.active_buffs)
                                };

                                // Remove existing effect from same skill
                                state.active_effects.retain(|e| {
                                    e.source_skill_name != skill.name
                                        || e.flags != dot.flags
                                });

                                state.active_effects.push(ActiveEffect {
                                    source_skill_name: skill.name.clone(),
                                    remaining_duration: dot.duration + delay,
                                    next_tick_in: interval + delay,
                                    tick_interval: interval,
                                    tick_count: 0,
                                    total_ticks,
                                    base_value,
                                    flags: dot.flags,
                                    coefficients: dot.coefficients,
                                    increase_per_tick: dot.increase_per_tick.unwrap_or(0.0),
                                    flat_increase_per_tick: dot.flat_increase_per_tick.unwrap_or(0.0),
                                    ignores_modifier: dot.ignores_modifier.unwrap_or(false),
                                    snapshotted_modifier,
                                    snapshotted_armor_factor: buffed.armor_factor,
                                    snapshotted_crit_mult: buffed.crit_mult,
                                });
                            }
                        }
                    }

                    // 4. Register/refresh Cast buffs from skill bonuses
                    self.register_cast_buffs(&mut state, skill);

                    // 5. Advance GCD
                    let cast_time = skill.channel_time.unwrap_or(GCD);
                    state.gcd_ready = state.time + cast_time;
                }
                Action::BarSwap => {
                    state.active_bar = state.active_bar.opposite();
                    state.gcd_ready = state.time + BAR_SWAP_DELAY;
                    state.bar_swap_count += 1;
                }
            }
        }

        let fight_duration = state.time.max(0.001);

        let mut skill_breakdown: Vec<SkillBreakdown> = state
            .skill_damage
            .into_iter()
            .map(|(name, (damage, count))| SkillBreakdown {
                skill_name: name,
                damage,
                cast_count: count,
            })
            .collect();
        skill_breakdown.sort_by(|a, b| b.damage.partial_cmp(&a.damage).unwrap());

        let total_damage = self.target_hp - state.remaining_hp.max(0.0);

        SimulationResult {
            total_damage,
            fight_duration,
            dps: total_damage / fight_duration,
            skill_breakdown,
            la_damage: state.la_damage,
            la_count: state.la_count,
            bar_swap_count: state.bar_swap_count,
        }
    }

    /// Compute a BuffedContext by applying active buff stat bonuses on top of base effective_stats.
    fn compute_buffed_context(&self, active_buffs: &[ActiveBuff]) -> BuffedContext {
        let mut stats = self.effective_stats.clone();

        // Pass 1: apply flat stat buffs
        for buff in active_buffs {
            match buff.target {
                BonusTarget::WeaponAndSpellDamageFlat => {
                    stats.weapon_damage += buff.value;
                    stats.spell_damage += buff.value;
                }
                BonusTarget::CriticalDamage => {
                    stats.critical_damage += buff.value;
                }
                BonusTarget::CriticalRating
                | BonusTarget::WeaponCriticalRating
                | BonusTarget::SpellCriticalRating => {
                    stats.critical_rating += buff.value;
                }
                BonusTarget::PhysicalAndSpellPenetration
                | BonusTarget::EnemyResistanceReduction => {
                    stats.penetration += buff.value;
                }
                _ => {}
            }
        }

        // Pass 2: apply percentage multipliers after flats
        for buff in active_buffs {
            match buff.target {
                BonusTarget::WeaponDamage => {
                    stats.weapon_damage *= 1.0 + buff.value;
                }
                BonusTarget::SpellDamage => {
                    stats.spell_damage *= 1.0 + buff.value;
                }
                BonusTarget::WeaponAndSpellDamageMultiplier => {
                    stats.weapon_damage *= 1.0 + buff.value;
                    stats.spell_damage *= 1.0 + buff.value;
                }
                _ => {}
            }
        }

        stats.clamp_caps();

        let armor_factor = crate::domain::formulas::armor_damage_factor(
            stats.target_armor,
            stats.penetration,
        );
        let crit_mult = crate::domain::formulas::critical_multiplier(
            stats.critical_chance(),
            stats.critical_damage,
        );

        BuffedContext {
            max_stat: stats.max_stat(),
            max_power: stats.max_power(),
            armor_factor,
            crit_mult,
        }
    }

    /// Sum damage modifier values from active buffs matching given DamageFlags.
    fn compute_buff_modifier_for_flags(
        &self,
        flags: DamageFlags,
        active_buffs: &[ActiveBuff],
    ) -> f64 {
        active_buffs
            .iter()
            .filter(|b| flags.matches_bonus_target(b.target))
            .map(|b| b.value)
            .sum()
    }

    /// Register permanent buffs from AbilitySlotted bonuses on all skills.
    fn register_ability_slotted_buffs(
        &self,
        state: &mut SimState,
        distribution: &BarDistribution,
    ) {
        for skill in distribution
            .bar1
            .skills
            .iter()
            .chain(distribution.bar2.skills.iter())
        {
            if let Some(bonuses) = &skill.bonuses {
                let ctx = ResolveContext::new(self.effective_stats.clone());
                for bonus in bonuses {
                    if bonus.trigger != BonusTrigger::AbilitySlotted {
                        continue;
                    }
                    let bv = bonus.resolve(&ctx);
                    // Deduplicate by bonus name
                    if !state.active_buffs.iter().any(|b| b.name == bv.name) {
                        state.active_buffs.push(ActiveBuff {
                            name: bv.name,
                            source_skill_name: skill.name.clone(),
                            remaining_duration: None, // permanent
                            target: bv.target,
                            value: bv.value,
                        });
                    }
                }
            }
        }
    }

    /// Register/refresh buffs from Cast-triggered bonuses when a skill is cast.
    fn register_cast_buffs(&self, state: &mut SimState, skill: &SkillData) {
        if let Some(bonuses) = &skill.bonuses {
            let ctx = ResolveContext::new(self.effective_stats.clone());
            for bonus in bonuses {
                if bonus.trigger != BonusTrigger::Cast {
                    continue;
                }
                let duration = match bonus.duration {
                    Some(d) => d,
                    None => continue, // Cast buffs without duration are ignored
                };
                let bv = bonus.resolve(&ctx);
                // Refresh existing buff or add new one
                if let Some(existing) = state.active_buffs.iter_mut().find(|b| b.name == bv.name) {
                    existing.remaining_duration = Some(duration);
                    existing.value = bv.value;
                    existing.source_skill_name = skill.name.clone();
                } else {
                    state.active_buffs.push(ActiveBuff {
                        name: bv.name,
                        source_skill_name: skill.name.clone(),
                        remaining_duration: Some(duration),
                        target: bv.target,
                        value: bv.value,
                    });
                }
            }
        }
    }

    fn advance_time(
        &self,
        state: &mut SimState,
        target_time: f64,
    ) {
        let dt = target_time - state.time;
        if dt <= 0.0 {
            return;
        }

        // Expire buffs (before ticking DoTs — DoTs already snapshotted so order doesn't matter)
        state.active_buffs.retain_mut(|buff| {
            match &mut buff.remaining_duration {
                None => true, // permanent buffs never expire
                Some(remaining) => {
                    *remaining -= dt;
                    *remaining > 0.0
                }
            }
        });

        // Tick all active DoT effects (using snapshotted values)
        let mut effects_to_remove = Vec::new();
        for (idx, effect) in state.active_effects.iter_mut().enumerate() {
            effect.remaining_duration -= dt;
            effect.next_tick_in -= dt;

            // Process any ticks that occurred during this time window
            while effect.next_tick_in <= 0.0 && effect.tick_count < effect.total_ticks {
                let pct_mult = 1.0 + (effect.tick_count as f64) * effect.increase_per_tick;
                let flat_inc = (effect.tick_count as f64) * effect.flat_increase_per_tick;
                let tick_damage = effect.base_value * pct_mult + flat_inc;

                let final_damage = if effect.ignores_modifier {
                    tick_damage * effect.snapshotted_armor_factor * effect.snapshotted_crit_mult
                } else {
                    tick_damage
                        * (1.0 + effect.snapshotted_modifier)
                        * effect.snapshotted_armor_factor
                        * effect.snapshotted_crit_mult
                };

                state.remaining_hp -= final_damage;

                // Track DoT damage under the source skill
                let entry = state
                    .skill_damage
                    .entry(effect.source_skill_name.clone())
                    .or_insert((0.0, 0));
                entry.0 += final_damage;

                effect.tick_count += 1;
                effect.next_tick_in += effect.tick_interval;
            }

            if effect.remaining_duration <= 0.0 || effect.tick_count >= effect.total_ticks {
                effects_to_remove.push(idx);
            }
        }

        // Remove expired effects (reverse order to preserve indices)
        for idx in effects_to_remove.into_iter().rev() {
            state.active_effects.remove(idx);
        }

        state.time = target_time;
    }

    fn decide_action(
        &self,
        state: &SimState,
        current_skills: &[&'static SkillData],
        other_skills: &[&'static SkillData],
    ) -> Action {
        let health_pct = state.remaining_hp / self.target_hp;

        // Priority 1: Current bar expired DoTs/buffs — recast
        if let Some(idx) = self.find_expired_dot_skill(state, current_skills) {
            return Action::CastSkill(idx);
        }

        // Priority 2: Current bar never-applied DoTs — cast
        if let Some(idx) = self.find_unapplied_dot_skill(state, current_skills) {
            return Action::CastSkill(idx);
        }

        // Priority 3: Other bar has expired or unapplied DoTs — swap
        if self.other_bar_needs_attention(state, other_skills) {
            return Action::BarSwap;
        }

        // Priority 4: Ready proc skill on current bar — fire
        if let Some(idx) = self.find_ready_proc_skill(state, current_skills) {
            return Action::CastSkill(idx);
        }

        // Priority 5: Ready proc skill on other bar — swap
        if self.other_bar_has_ready_proc(state, other_skills) {
            return Action::BarSwap;
        }

        // Priority 6: Finisher on current bar when in execute range
        if let Some(idx) = self.find_execute_filler(current_skills, health_pct) {
            return Action::CastSkill(idx);
        }

        // Priority 7: Finisher on other bar when in execute range — swap
        if self.other_bar_has_execute_filler(other_skills, health_pct) {
            return Action::BarSwap;
        }

        // Priority 8: Current bar spammable or channeled filler
        if let Some(idx) = current_skills
            .iter()
            .position(|s| s.spammable || s.channel_time.is_some())
        {
            return Action::CastSkill(idx);
        }

        // Priority 9: Any skill with damage on current bar (exclude unready proc skills)
        if let Some(idx) = current_skills.iter().position(|s| {
            s.damage.is_some() && !self.is_unready_proc(state, s)
        }) {
            return Action::CastSkill(idx);
        }

        // Fallback: bar swap
        Action::BarSwap
    }

    /// Check if a skill has an active effect (DoT) or active buff sourced from it.
    fn skill_has_active_presence(&self, state: &SimState, skill: &SkillData) -> bool {
        let has_active_effect = state
            .active_effects
            .iter()
            .any(|e| e.source_skill_name == skill.name && e.remaining_duration > 0.0);
        let has_active_buff = state.active_buffs.iter().any(|b| {
            b.source_skill_name == skill.name
                && b.remaining_duration.map_or(false, |d| d > 0.0)
        });
        has_active_effect || has_active_buff
    }

    /// Check if a skill has an active effect/buff with enough remaining duration
    /// to survive a bar swap + one GCD.
    fn skill_has_durable_presence(&self, state: &SimState, skill: &SkillData) -> bool {
        let threshold = BAR_SWAP_DELAY + GCD;
        let has_durable_effect = state
            .active_effects
            .iter()
            .any(|e| e.source_skill_name == skill.name && e.remaining_duration > threshold);
        let has_durable_buff = state.active_buffs.iter().any(|b| {
            b.source_skill_name == skill.name
                && b.remaining_duration.map_or(false, |d| d > threshold)
        });
        has_durable_effect || has_durable_buff
    }

    fn find_expired_dot_skill(
        &self,
        state: &SimState,
        skills: &[&'static SkillData],
    ) -> Option<usize> {
        let mut best: Option<(usize, f64)> = None;

        for (idx, skill) in skills.iter().enumerate() {
            if !self.skill_has_dot_or_buff(skill) {
                continue;
            }
            // Check if this skill's effects/buffs have expired
            if !self.skill_has_active_presence(state, skill) {
                // Check if it was ever applied (has entry in damage tracking or was cast)
                let was_applied = state.skill_damage.contains_key(&skill.name);
                if was_applied {
                    let dpc = self.estimate_skill_dpc(skill);
                    if best.map_or(true, |(_, best_dpc)| dpc > best_dpc) {
                        best = Some((idx, dpc));
                    }
                }
            }
        }

        best.map(|(idx, _)| idx)
    }

    fn find_unapplied_dot_skill(
        &self,
        state: &SimState,
        skills: &[&'static SkillData],
    ) -> Option<usize> {
        let mut best: Option<(usize, f64)> = None;

        for (idx, skill) in skills.iter().enumerate() {
            if !self.skill_has_dot_or_buff(skill) {
                continue;
            }
            let has_presence = self.skill_has_active_presence(state, skill);
            let was_cast = state.skill_damage.contains_key(&skill.name);

            if !has_presence && !was_cast {
                let dpc = self.estimate_skill_dpc(skill);
                if best.map_or(true, |(_, best_dpc)| dpc > best_dpc) {
                    best = Some((idx, dpc));
                }
            }
        }

        best.map(|(idx, _)| idx)
    }

    fn other_bar_needs_attention(
        &self,
        state: &SimState,
        other_skills: &[&'static SkillData],
    ) -> bool {
        for skill in other_skills {
            if !self.skill_has_dot_or_buff(skill) {
                continue;
            }
            if !self.skill_has_durable_presence(state, skill) {
                return true;
            }
        }
        false
    }

    fn skill_has_dot_or_buff(&self, skill: &SkillData) -> bool {
        if skill.spammable {
            return false;
        }
        // Channeled skills deal damage during their channel but don't leave persistent effects
        if skill.channel_time.is_some() {
            return false;
        }
        // Proc skills are handled by the proc priority system, not as DoTs/buffs
        if skill.proc_light_attacks.is_some() {
            return false;
        }
        let has_dot = skill
            .damage
            .as_ref()
            .and_then(|d| d.dots.as_ref())
            .is_some_and(|dots| !dots.is_empty());
        let has_buff = skill
            .bonuses
            .as_ref()
            .is_some_and(|bonuses| bonuses.iter().any(|b| b.duration.is_some()));
        has_dot || has_buff
    }

    fn find_ready_proc_skill(
        &self,
        state: &SimState,
        skills: &[&'static SkillData],
    ) -> Option<usize> {
        skills.iter().position(|s| {
            if let Some(threshold) = s.proc_light_attacks {
                let counter = state.proc_counters.get(&s.name).copied().unwrap_or(0);
                counter >= threshold
            } else {
                false
            }
        })
    }

    fn other_bar_has_ready_proc(
        &self,
        state: &SimState,
        other_skills: &[&'static SkillData],
    ) -> bool {
        other_skills.iter().any(|s| {
            if let Some(threshold) = s.proc_light_attacks {
                let counter = state.proc_counters.get(&s.name).copied().unwrap_or(0);
                counter >= threshold
            } else {
                false
            }
        })
    }

    fn is_unready_proc(&self, state: &SimState, skill: &SkillData) -> bool {
        if let Some(threshold) = skill.proc_light_attacks {
            let counter = state.proc_counters.get(&skill.name).copied().unwrap_or(0);
            counter < threshold
        } else {
            false
        }
    }

    fn estimate_skill_dpc(&self, skill: &SkillData) -> f64 {
        skill.calculate_damage_per_cast(&self.resolved_bonuses, &self.effective_stats, None)
    }

    /// Find a finisher skill on the current bar whose execute threshold the enemy is below.
    /// If multiple finishers qualify, pick the one with highest expected damage at current health.
    fn find_execute_filler(
        &self,
        skills: &[&'static SkillData],
        health_pct: f64,
    ) -> Option<usize> {
        let mut best: Option<(usize, f64)> = None;
        for (idx, skill) in skills.iter().enumerate() {
            if let Some(execute) = &skill.execute {
                if health_pct < execute.threshold {
                    let mult = execute.calculate_multiplier(health_pct);
                    if best.map_or(true, |(_, best_mult)| mult > best_mult) {
                        best = Some((idx, mult));
                    }
                }
            }
        }
        best.map(|(idx, _)| idx)
    }

    /// Check if the other bar has a finisher skill whose execute threshold the enemy is below.
    fn other_bar_has_execute_filler(
        &self,
        other_skills: &[&'static SkillData],
        health_pct: f64,
    ) -> bool {
        other_skills.iter().any(|skill| {
            skill.execute.as_ref().is_some_and(|e| health_pct < e.threshold)
        })
    }

    fn calc_skill_hits(
        &self,
        skill: &SkillData,
        buffed: &BuffedContext,
        active_buffs: &[ActiveBuff],
        health_pct: f64,
    ) -> f64 {
        let mut total = 0.0;

        if let Some(damage) = &skill.damage {
            if let Some(hits) = &damage.hits {
                for hit in hits {
                    if let Some(threshold) = hit.execute_threshold {
                        if health_pct >= threshold {
                            continue;
                        }
                    }
                    let modifier = self.compute_modifier_for_flags(
                        hit.flags,
                        Some(skill.skill_line),
                        health_pct,
                    ) + self.compute_buff_modifier_for_flags(hit.flags, active_buffs);
                    let base = hit.effective_value(buffed.max_stat, buffed.max_power);
                    let mut dmg = base * (1.0 + modifier) * buffed.armor_factor * buffed.crit_mult;
                    if let Some(execute) = &skill.execute {
                        dmg *= execute.calculate_multiplier(health_pct);
                    }
                    total += dmg;
                }
            }
        }

        total
    }

    fn compute_modifier_for_flags(
        &self,
        flags: DamageFlags,
        skill_line: Option<SkillLineName>,
        health_pct: f64,
    ) -> f64 {
        let ctx = ResolveContext::new(self.effective_stats.clone());
        self.resolved_bonuses
            .iter()
            .filter(|b| {
                b.skill_line_filter
                    .map_or(true, |sl| skill_line.map_or(false, |s| s == sl))
                    && b.execute_threshold.map_or(true, |t| health_pct < t)
            })
            .map(|b| {
                let bv = b.resolve(&ctx);
                if flags.matches_bonus_target(bv.target) {
                    bv.value
                } else {
                    0.0
                }
            })
            .sum()
    }
}
