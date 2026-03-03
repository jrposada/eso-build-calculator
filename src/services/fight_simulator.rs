use crate::data::light_attacks::light_attack_for_weapon;
use crate::domain::simulation::{BAR_SWAP_DELAY, GCD, TRIAL_DUMMY_HP};
use crate::domain::weapon_enchant::WeaponEnchant;
use crate::domain::{
    ActiveBar, ActiveBuff, ActiveEffect, BonusData, BonusTarget, BonusTrigger, BuffUptime,
    CharacterStats, DamageCoefficients, DamageFlags, ResolveContext, SetProcAction, SetProcEffect,
    SetProcTrigger, SimulationResult, SkillBreakdown, SkillData, SkillLineName,
};
use std::collections::{HashMap, HashSet};

use super::bar_distribution::BarDistribution;

pub struct FightSimulator {
    pub target_hp: f64,
    pub effective_stats: CharacterStats,
    pub resolved_bonuses: Vec<BonusData>,
    pub armor_factor: f64,
    pub crit_mult: f64,
    /// Buff names already provided externally (e.g. trial dummy).
    /// Prevents the simulator from double-counting when a skill provides the same buff.
    suppressed_buff_names: HashSet<String>,
    /// Weapon enchants for each bar (None = no enchant modeled)
    pub bar1_enchant: Option<WeaponEnchant>,
    pub bar2_enchant: Option<WeaponEnchant>,
    /// Set proc effects from equipped gear sets
    pub set_procs: Vec<SetProcEffect>,
    /// Average resource percentage (0-100) for resource-scaling set procs
    pub avg_resource_pct: f64,
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
    // Buff uptime tracking: buff name -> total seconds active
    buff_uptimes: HashMap<String, f64>,
    // Weapon enchant cooldown: time when enchant can next proc
    enchant_ready: f64,
    // Weapon enchant damage tracking
    enchant_damage: f64,
    enchant_proc_count: u32,
    // Set proc state
    set_proc_cooldowns: HashMap<String, f64>,
    set_proc_stacks: HashMap<String, (u32, f64)>,
    set_proc_damage: HashMap<String, (f64, u32)>,
    // Flat LA bonus from set procs (summed once at init)
    flat_la_bonus: f64,
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
        suppressed_buff_names: HashSet<String>,
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
            suppressed_buff_names,
            bar1_enchant: None,
            bar2_enchant: None,
            set_procs: Vec::new(),
            avg_resource_pct: 50.0,
        }
    }

    pub fn with_set_procs(mut self, procs: Vec<SetProcEffect>) -> Self {
        self.set_procs = procs;
        self
    }

    pub fn with_avg_resource_pct(mut self, pct: f64) -> Self {
        self.avg_resource_pct = pct;
        self
    }

    pub fn with_enchants(
        mut self,
        bar1_enchant: Option<WeaponEnchant>,
        bar2_enchant: Option<WeaponEnchant>,
    ) -> Self {
        self.bar1_enchant = bar1_enchant;
        self.bar2_enchant = bar2_enchant;
        self
    }

    /// Compute character stats with all AbilitySlotted buffs applied (self-buffed stats).
    pub fn compute_buffed_stats(&self, distribution: &BarDistribution) -> CharacterStats {
        let mut buffs: Vec<ActiveBuff> = Vec::new();

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
                    if self.suppressed_buff_names.contains(&bv.name) {
                        continue;
                    }
                    if !buffs.iter().any(|b| b.name == bv.name) {
                        buffs.push(ActiveBuff {
                            name: bv.name,
                            source_skill_name: skill.name.clone(),
                            remaining_duration: None,
                            target: bv.target,
                            value: bv.value,
                        });
                    }
                }
            }
        }

        self.apply_buffs_to_stats(&buffs)
    }

    /// Apply a set of buffs to effective_stats and return the resulting CharacterStats.
    fn apply_buffs_to_stats(&self, active_buffs: &[ActiveBuff]) -> CharacterStats {
        let mut stats = self.effective_stats.clone();

        // Pass 1: flat stat buffs
        for buff in active_buffs {
            match buff.target {
                BonusTarget::WeaponAndSpellDamageFlat => {
                    stats.weapon_damage += buff.value;
                    stats.spell_damage += buff.value;
                }
                BonusTarget::WeaponDamageFlat => {
                    stats.weapon_damage += buff.value;
                }
                BonusTarget::SpellDamageFlat => {
                    stats.spell_damage += buff.value;
                }
                BonusTarget::MaxMagickaFlat => {
                    stats.max_magicka += buff.value;
                }
                BonusTarget::MaxStaminaFlat => {
                    stats.max_stamina += buff.value;
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

        // Pass 2: percentage multipliers after flats
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
        stats
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

        // Compute flat LA bonus from set procs
        let flat_la_bonus: f64 = self
            .set_procs
            .iter()
            .filter_map(|p| match &p.action {
                SetProcAction::FlatLightAttackBonus { value } => Some(value),
                _ => None,
            })
            .sum();

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
            buff_uptimes: HashMap::new(),
            enchant_ready: 0.0,
            enchant_damage: 0.0,
            enchant_proc_count: 0,
            set_proc_cooldowns: HashMap::new(),
            set_proc_stacks: HashMap::new(),
            set_proc_damage: HashMap::new(),
            flat_la_bonus,
        };

        // Register permanent AbilitySlotted buffs from all skills on both bars
        self.register_ability_slotted_buffs(&mut state, distribution);

        // Register static buffs from ResourceScalingBuff set procs
        for proc in &self.set_procs {
            if let SetProcAction::ResourceScalingBuff {
                target,
                max_value,
                threshold_pct,
            } = &proc.action
            {
                let buff_value = match threshold_pct {
                    Some(t) => {
                        if self.avg_resource_pct < *t {
                            *max_value
                        } else {
                            0.0
                        }
                    }
                    None => max_value * (1.0 - self.avg_resource_pct / 100.0),
                };
                if buff_value > 0.0 {
                    state.active_buffs.push(ActiveBuff {
                        name: proc.name.clone(),
                        source_skill_name: "Set Proc".to_string(),
                        remaining_duration: None, // permanent
                        target: *target,
                        value: buff_value,
                    });
                }
            }
        }

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
                    let (la_done_base, la_taken_base) =
                        self.compute_modifier_for_flags(la_data.flags, None, health_pct);
                    let (la_done_buff, la_taken_buff) =
                        self.compute_buff_modifier_for_flags(la_data.flags, &state.active_buffs);
                    let mut la_dmg = la_data.calculate_damage(
                        la_done_base + la_done_buff,
                        la_taken_base + la_taken_buff,
                        buffed.max_stat,
                        buffed.max_power,
                        buffed.armor_factor,
                        buffed.crit_mult,
                    );
                    // Add flat LA bonus from set procs (applied with same modifiers)
                    if state.flat_la_bonus > 0.0 {
                        la_dmg += state.flat_la_bonus
                            * (1.0 + la_done_base + la_done_buff)
                            * (1.0 + la_taken_base + la_taken_buff)
                            * buffed.armor_factor
                            * buffed.crit_mult;
                    }
                    state.remaining_hp -= la_dmg;
                    state.la_damage += la_dmg;
                    state.la_count += 1;

                    // Increment all proc counters on every light attack
                    for counter in state.proc_counters.values_mut() {
                        *counter += 1;
                    }

                    // 1b. Weapon enchant proc (triggered by light attack)
                    if state.time >= state.enchant_ready {
                        let enchant = match state.active_bar {
                            ActiveBar::Bar1 => self.bar1_enchant,
                            ActiveBar::Bar2 => self.bar2_enchant,
                        };
                        if let Some(enchant) = enchant {
                            let base_dmg = enchant.base_damage();
                            if base_dmg > 0.0 {
                                let flags = enchant.damage_flags();
                                let (done_base, taken_base) =
                                    self.compute_modifier_for_flags(flags, None, health_pct);
                                let (done_buff, taken_buff) = self
                                    .compute_buff_modifier_for_flags(flags, &state.active_buffs);
                                let enchant_dmg = base_dmg
                                    * (1.0 + done_base + done_buff)
                                    * (1.0 + taken_base + taken_buff)
                                    * buffed.armor_factor
                                    * buffed.crit_mult;
                                state.remaining_hp -= enchant_dmg;
                                state.enchant_damage += enchant_dmg;
                                state.enchant_proc_count += 1;
                            }

                            // Register status effect as active DoT
                            if let Some(status) = enchant.status_effect() {
                                let tick_value = status.total_damage;
                                let (done_base, taken_base) =
                                    self.compute_modifier_for_flags(status.flags, None, health_pct);
                                let (done_buff, taken_buff) = self.compute_buff_modifier_for_flags(
                                    status.flags,
                                    &state.active_buffs,
                                );

                                // Remove existing status effect of same type
                                state
                                    .active_effects
                                    .retain(|e| e.source_skill_name != status.name);

                                state.active_effects.push(ActiveEffect {
                                    source_skill_name: status.name.to_string(),
                                    remaining_duration: status.duration,
                                    next_tick_in: status.duration,
                                    tick_interval: status.duration,
                                    tick_count: 0,
                                    total_ticks: 1,
                                    base_value: tick_value,
                                    flags: status.flags,
                                    coefficients: crate::domain::DamageCoefficients::new(0.0, 0.0),
                                    increase_per_tick: 0.0,
                                    flat_increase_per_tick: 0.0,
                                    ignores_modifier: false,
                                    snapshotted_done_modifier: done_base + done_buff,
                                    snapshotted_taken_modifier: taken_base + taken_buff,
                                    snapshotted_armor_factor: buffed.armor_factor,
                                    snapshotted_crit_mult: buffed.crit_mult,
                                });
                            }

                            state.enchant_ready = state.time + enchant.cooldown();
                        }
                    }

                    // 1c. Set proc triggers: OnLightAttack
                    self.process_set_procs(
                        SetProcTrigger::OnLightAttack,
                        &buffed,
                        &mut state,
                        health_pct,
                    );

                    // 2. Skill hit damage (instant portion), gated by proc requirement
                    let hit_dmg = if let Some(threshold) = skill.proc_light_attacks {
                        let counter = state.proc_counters.get(&skill.name).copied().unwrap_or(0);
                        if counter >= threshold {
                            let dmg = self.calc_skill_hits(
                                skill,
                                &buffed,
                                &state.active_buffs,
                                health_pct,
                            );
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

                    // 2b. Set proc triggers: OnDirectDamage (after skill hit)
                    if hit_dmg > 0.0 {
                        self.process_set_procs(
                            SetProcTrigger::OnDirectDamage,
                            &buffed,
                            &mut state,
                            health_pct,
                        );
                    }

                    // 3. Register/refresh DoTs as active effects (snapshot at cast time)
                    if let Some(damage) = &skill.damage {
                        if let Some(dots) = &damage.dots {
                            for dot in dots {
                                let base_value =
                                    dot.effective_value(buffed.max_stat, buffed.max_power);
                                let interval = dot.interval.unwrap_or(dot.duration);
                                let total_ticks = (dot.duration / interval).floor() as i32;
                                let delay = dot.delay.unwrap_or(0.0);

                                // Snapshot modifier at cast time (includes execute bonuses if currently in range)
                                let (snapshotted_done, snapshotted_taken) =
                                    if dot.ignores_modifier.unwrap_or(false) {
                                        (0.0, 0.0)
                                    } else {
                                        let (done_base, taken_base) = self
                                            .compute_modifier_for_flags(
                                                dot.flags,
                                                Some(skill.skill_line),
                                                health_pct,
                                            );
                                        let (done_buff, taken_buff) = self
                                            .compute_buff_modifier_for_flags(
                                                dot.flags,
                                                &state.active_buffs,
                                            );
                                        (done_base + done_buff, taken_base + taken_buff)
                                    };

                                // Remove existing effect from same skill
                                state.active_effects.retain(|e| {
                                    e.source_skill_name != skill.name || e.flags != dot.flags
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
                                    flat_increase_per_tick: dot
                                        .flat_increase_per_tick
                                        .unwrap_or(0.0),
                                    ignores_modifier: dot.ignores_modifier.unwrap_or(false),
                                    snapshotted_done_modifier: snapshotted_done,
                                    snapshotted_taken_modifier: snapshotted_taken,
                                    snapshotted_armor_factor: buffed.armor_factor,
                                    snapshotted_crit_mult: buffed.crit_mult,
                                });
                            }
                        }
                    }

                    // 3b. Set proc triggers: OnDealDamage (after all damage)
                    self.process_set_procs(
                        SetProcTrigger::OnDealDamage,
                        &buffed,
                        &mut state,
                        health_pct,
                    );

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

        // Add weapon enchant damage as a breakdown entry
        if state.enchant_damage > 0.0 {
            skill_breakdown.push(SkillBreakdown {
                skill_name: "Weapon Enchant".to_string(),
                damage: state.enchant_damage,
                cast_count: state.enchant_proc_count,
            });
        }

        // Add set proc damage entries
        for (name, (damage, count)) in &state.set_proc_damage {
            if *damage > 0.0 {
                skill_breakdown.push(SkillBreakdown {
                    skill_name: name.clone(),
                    damage: *damage,
                    cast_count: *count,
                });
            }
        }

        skill_breakdown.sort_by(|a, b| b.damage.partial_cmp(&a.damage).unwrap());

        let total_damage = self.target_hp - state.remaining_hp.max(0.0);

        let mut buff_uptimes: Vec<BuffUptime> = state
            .buff_uptimes
            .into_iter()
            .map(|(name, time)| BuffUptime {
                name,
                uptime: (time / fight_duration).min(1.0),
                external: false,
            })
            .collect();

        // Add 100% uptime for externally-provided buffs (e.g. trial dummy)
        for name in &self.suppressed_buff_names {
            buff_uptimes.push(BuffUptime {
                name: name.clone(),
                uptime: 1.0,
                external: true,
            });
        }

        buff_uptimes.sort_by(|a, b| a.name.cmp(&b.name));

        SimulationResult {
            total_damage,
            fight_duration,
            dps: total_damage / fight_duration,
            skill_breakdown,
            la_damage: state.la_damage,
            la_count: state.la_count,
            bar_swap_count: state.bar_swap_count,
            buff_uptimes,
        }
    }

    /// Compute a BuffedContext by applying active buff stat bonuses on top of base effective_stats.
    fn compute_buffed_context(&self, active_buffs: &[ActiveBuff]) -> BuffedContext {
        let stats = self.apply_buffs_to_stats(active_buffs);

        let armor_factor =
            crate::domain::formulas::armor_damage_factor(stats.target_armor, stats.penetration);
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
    /// Returns (damage_done, enemy_damage_taken) as separate additive layers.
    fn compute_buff_modifier_for_flags(
        &self,
        flags: DamageFlags,
        active_buffs: &[ActiveBuff],
    ) -> (f64, f64) {
        let mut done = 0.0;
        let mut taken = 0.0;
        for b in active_buffs {
            if b.target == BonusTarget::EnemyDamageTaken {
                taken += b.value;
            } else if flags.matches_bonus_target(b.target) {
                done += b.value;
            }
        }
        (done, taken)
    }

    /// Register permanent buffs from AbilitySlotted bonuses on all skills.
    fn register_ability_slotted_buffs(&self, state: &mut SimState, distribution: &BarDistribution) {
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
                    // Skip buffs already provided externally (e.g. trial dummy)
                    if self.suppressed_buff_names.contains(&bv.name) {
                        continue;
                    }
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
                // Skip buffs already provided externally (e.g. trial dummy)
                if self.suppressed_buff_names.contains(&bv.name) {
                    continue;
                }
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

    fn advance_time(&self, state: &mut SimState, target_time: f64) {
        let dt = target_time - state.time;
        if dt <= 0.0 {
            return;
        }

        // Accumulate buff uptimes before expiring
        for buff in &state.active_buffs {
            let active_time = match buff.remaining_duration {
                None => dt,                           // permanent - active full dt
                Some(remaining) => dt.min(remaining), // may expire partway
            };
            *state.buff_uptimes.entry(buff.name.clone()).or_insert(0.0) += active_time;
        }

        // Expire buffs (before ticking DoTs - DoTs already snapshotted so order doesn't matter)
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
                    tick_damage
                        * (1.0 + effect.snapshotted_taken_modifier)
                        * effect.snapshotted_armor_factor
                        * effect.snapshotted_crit_mult
                } else {
                    tick_damage
                        * (1.0 + effect.snapshotted_done_modifier)
                        * (1.0 + effect.snapshotted_taken_modifier)
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

        // Priority 1: Current bar expired DoTs/buffs - recast
        if let Some(idx) = self.find_expired_dot_skill(state, current_skills) {
            return Action::CastSkill(idx);
        }

        // Priority 2: Current bar never-applied DoTs - cast
        if let Some(idx) = self.find_unapplied_dot_skill(state, current_skills) {
            return Action::CastSkill(idx);
        }

        // Priority 3: Other bar has expired or unapplied DoTs - swap
        if self.other_bar_needs_attention(state, other_skills) {
            return Action::BarSwap;
        }

        // Priority 4: Ready proc skill on current bar - fire
        if let Some(idx) = self.find_ready_proc_skill(state, current_skills) {
            return Action::CastSkill(idx);
        }

        // Priority 5: Ready proc skill on other bar - swap
        if self.other_bar_has_ready_proc(state, other_skills) {
            return Action::BarSwap;
        }

        // Priority 6: Finisher on current bar when in execute range
        if let Some(idx) = self.find_execute_filler(current_skills, health_pct) {
            return Action::CastSkill(idx);
        }

        // Priority 7: Finisher on other bar when in execute range - swap
        if self.other_bar_has_execute_filler(other_skills, health_pct) {
            return Action::BarSwap;
        }

        // Priority 8a: Non-execute spammable or channeled filler
        if let Some(idx) = current_skills
            .iter()
            .position(|s| (s.spammable || s.channel_time.is_some()) && s.execute.is_none())
        {
            return Action::CastSkill(idx);
        }

        // Priority 8b: Execute spammable as fallback (above threshold, base damage only)
        if let Some(idx) = current_skills
            .iter()
            .position(|s| s.spammable && s.execute.is_some())
        {
            return Action::CastSkill(idx);
        }

        // Priority 9: Any skill with damage on current bar (exclude unready proc skills)
        if let Some(idx) = current_skills
            .iter()
            .position(|s| s.damage.is_some() && !self.is_unready_proc(state, s))
        {
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
            b.source_skill_name == skill.name && b.remaining_duration.map_or(false, |d| d > 0.0)
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
    fn find_execute_filler(&self, skills: &[&'static SkillData], health_pct: f64) -> Option<usize> {
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
            skill
                .execute
                .as_ref()
                .is_some_and(|e| health_pct < e.threshold)
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
                    let (done_base, taken_base) = self.compute_modifier_for_flags(
                        hit.flags,
                        Some(skill.skill_line),
                        health_pct,
                    );
                    let (done_buff, taken_buff) =
                        self.compute_buff_modifier_for_flags(hit.flags, active_buffs);
                    let base = hit.effective_value(buffed.max_stat, buffed.max_power);
                    let mut dmg = base
                        * (1.0 + done_base + done_buff)
                        * (1.0 + taken_base + taken_buff)
                        * buffed.armor_factor
                        * buffed.crit_mult;
                    if let Some(execute) = &skill.execute {
                        dmg *= execute.calculate_multiplier(health_pct);
                    }
                    total += dmg;
                }
            }
        }

        total
    }

    /// Process all set procs that match the given trigger.
    fn process_set_procs(
        &self,
        trigger: SetProcTrigger,
        buffed: &BuffedContext,
        state: &mut SimState,
        health_pct: f64,
    ) {
        // Collect matching procs (avoid borrow conflict with state)
        let matching: Vec<(usize, &SetProcEffect)> = self
            .set_procs
            .iter()
            .enumerate()
            .filter(|(_, p)| p.trigger == trigger)
            .collect();

        for (_idx, proc) in matching {
            // Check cooldown
            let ready_time = state
                .set_proc_cooldowns
                .get(&proc.name)
                .copied()
                .unwrap_or(0.0);
            if proc.cooldown > 0.0 && state.time < ready_time {
                continue;
            }

            match &proc.action {
                SetProcAction::DamageProc {
                    hit_damage,
                    hit_flags,
                    dot_total_damage,
                    dot_duration,
                    dot_flags,
                } => {
                    // Calculate hit damage
                    let (done_base, taken_base) =
                        self.compute_modifier_for_flags(*hit_flags, None, health_pct);
                    let (done_buff, taken_buff) =
                        self.compute_buff_modifier_for_flags(*hit_flags, &state.active_buffs);
                    let dmg = hit_damage
                        * (1.0 + done_base + done_buff)
                        * (1.0 + taken_base + taken_buff)
                        * buffed.armor_factor
                        * buffed.crit_mult;
                    state.remaining_hp -= dmg;

                    let entry = state
                        .set_proc_damage
                        .entry(proc.name.clone())
                        .or_insert((0.0, 0));
                    entry.0 += dmg;
                    entry.1 += 1;

                    // Register follow-up DoT if present
                    if *dot_total_damage > 0.0 && *dot_duration > 0.0 {
                        let (dot_done_base, dot_taken_base) =
                            self.compute_modifier_for_flags(*dot_flags, None, health_pct);
                        let (dot_done_buff, dot_taken_buff) =
                            self.compute_buff_modifier_for_flags(*dot_flags, &state.active_buffs);

                        // Remove existing DoT from same proc
                        let dot_source = format!("{} DoT", proc.name);
                        state
                            .active_effects
                            .retain(|e| e.source_skill_name != dot_source);

                        state.active_effects.push(ActiveEffect {
                            source_skill_name: dot_source,
                            remaining_duration: *dot_duration,
                            next_tick_in: *dot_duration,
                            tick_interval: *dot_duration,
                            tick_count: 0,
                            total_ticks: 1,
                            base_value: *dot_total_damage,
                            flags: *dot_flags,
                            coefficients: DamageCoefficients::new(0.0, 0.0),
                            increase_per_tick: 0.0,
                            flat_increase_per_tick: 0.0,
                            ignores_modifier: false,
                            snapshotted_done_modifier: dot_done_base + dot_done_buff,
                            snapshotted_taken_modifier: dot_taken_base + dot_taken_buff,
                            snapshotted_armor_factor: buffed.armor_factor,
                            snapshotted_crit_mult: buffed.crit_mult,
                        });
                    }

                    // Set cooldown
                    if proc.cooldown > 0.0 {
                        state
                            .set_proc_cooldowns
                            .insert(proc.name.clone(), state.time + proc.cooldown);
                    }
                }
                SetProcAction::StackingDot {
                    damage_per_stack_per_tick,
                    tick_interval,
                    max_stacks,
                    stack_duration,
                    stack_cooldown,
                    flags,
                } => {
                    // Check stack cooldown
                    let (current_stacks, last_stack_time) = state
                        .set_proc_stacks
                        .get(&proc.name)
                        .copied()
                        .unwrap_or((0, -100.0));

                    if *stack_cooldown > 0.0 && state.time - last_stack_time < *stack_cooldown {
                        continue;
                    }

                    let new_stacks = (current_stacks + 1).min(*max_stacks);
                    state
                        .set_proc_stacks
                        .insert(proc.name.clone(), (new_stacks, state.time));

                    // Snapshot modifiers
                    let (done_base, taken_base) =
                        self.compute_modifier_for_flags(*flags, None, health_pct);
                    let (done_buff, taken_buff) =
                        self.compute_buff_modifier_for_flags(*flags, &state.active_buffs);

                    // Update existing effect in-place (preserve tick timer) or create new
                    if let Some(existing) = state
                        .active_effects
                        .iter_mut()
                        .find(|e| e.source_skill_name == proc.name)
                    {
                        // Refresh duration and update base_value for new stack count
                        existing.remaining_duration = *stack_duration;
                        existing.base_value = *damage_per_stack_per_tick * new_stacks as f64;
                        let new_total = (*stack_duration / *tick_interval).floor() as i32;
                        existing.total_ticks = existing.tick_count + new_total;
                        // Re-snapshot modifiers on stack refresh
                        existing.snapshotted_done_modifier = done_base + done_buff;
                        existing.snapshotted_taken_modifier = taken_base + taken_buff;
                        existing.snapshotted_armor_factor = buffed.armor_factor;
                        existing.snapshotted_crit_mult = buffed.crit_mult;
                    } else {
                        let total_ticks = (*stack_duration / *tick_interval).floor() as i32;
                        state.active_effects.push(ActiveEffect {
                            source_skill_name: proc.name.clone(),
                            remaining_duration: *stack_duration,
                            next_tick_in: *tick_interval,
                            tick_interval: *tick_interval,
                            tick_count: 0,
                            total_ticks,
                            base_value: *damage_per_stack_per_tick * new_stacks as f64,
                            flags: *flags,
                            coefficients: DamageCoefficients::new(0.0, 0.0),
                            increase_per_tick: 0.0,
                            flat_increase_per_tick: 0.0,
                            ignores_modifier: false,
                            snapshotted_done_modifier: done_base + done_buff,
                            snapshotted_taken_modifier: taken_base + taken_buff,
                            snapshotted_armor_factor: buffed.armor_factor,
                            snapshotted_crit_mult: buffed.crit_mult,
                        });
                    }
                }
                SetProcAction::StackingBuff {
                    per_stack_target,
                    per_stack_value,
                    max_stacks,
                    stack_duration,
                    stack_cooldown,
                    at_max_buff_name,
                    at_max_buff_target,
                    at_max_buff_value,
                    at_max_buff_duration,
                } => {
                    // Check stack cooldown
                    let (current_stacks, last_stack_time) = state
                        .set_proc_stacks
                        .get(&proc.name)
                        .copied()
                        .unwrap_or((0, -100.0));

                    if *stack_cooldown > 0.0 && state.time - last_stack_time < *stack_cooldown {
                        continue;
                    }

                    let new_stacks = (current_stacks + 1).min(*max_stacks);
                    state
                        .set_proc_stacks
                        .insert(proc.name.clone(), (new_stacks, state.time));

                    // Apply per-stack buff if applicable
                    if let Some(target) = per_stack_target {
                        let buff_name = format!("{} Stacks", proc.name);
                        let total_value = *per_stack_value * new_stacks as f64;
                        // Skip if suppressed externally
                        if !self.suppressed_buff_names.contains(&buff_name) {
                            if let Some(existing) =
                                state.active_buffs.iter_mut().find(|b| b.name == buff_name)
                            {
                                existing.value = total_value;
                                existing.remaining_duration = Some(*stack_duration);
                            } else {
                                state.active_buffs.push(ActiveBuff {
                                    name: buff_name,
                                    source_skill_name: proc.name.clone(),
                                    remaining_duration: Some(*stack_duration),
                                    target: *target,
                                    value: total_value,
                                });
                            }
                        }
                    }

                    // Apply max-stack reward buff
                    if new_stacks >= *max_stacks {
                        if !self.suppressed_buff_names.contains(at_max_buff_name) {
                            if let Some(existing) = state
                                .active_buffs
                                .iter_mut()
                                .find(|b| b.name == *at_max_buff_name)
                            {
                                existing.remaining_duration = Some(*at_max_buff_duration);
                                existing.value = *at_max_buff_value;
                            } else {
                                state.active_buffs.push(ActiveBuff {
                                    name: at_max_buff_name.clone(),
                                    source_skill_name: proc.name.clone(),
                                    remaining_duration: Some(*at_max_buff_duration),
                                    target: *at_max_buff_target,
                                    value: *at_max_buff_value,
                                });
                            }
                        }
                    }
                }
                SetProcAction::FlatLightAttackBonus { .. } => {
                    // Handled at init time via flat_la_bonus field
                }
                SetProcAction::ResourceScalingBuff { .. } => {
                    // Handled at init time as a static buff
                }
            }
        }
    }

    /// Sum damage modifier values from resolved bonuses matching given DamageFlags.
    /// Returns (damage_done, enemy_damage_taken) as separate additive layers.
    fn compute_modifier_for_flags(
        &self,
        flags: DamageFlags,
        skill_line: Option<SkillLineName>,
        health_pct: f64,
    ) -> (f64, f64) {
        let ctx = ResolveContext::new(self.effective_stats.clone());
        let mut done = 0.0;
        let mut taken = 0.0;
        for b in &self.resolved_bonuses {
            if !b
                .skill_line_filter
                .map_or(true, |sl| skill_line.map_or(false, |s| s == sl))
            {
                continue;
            }
            if !b.execute_threshold.map_or(true, |t| health_pct < t) {
                continue;
            }
            let bv = b.resolve(&ctx);
            if bv.target == BonusTarget::EnemyDamageTaken {
                taken += bv.value;
            } else if flags.matches_bonus_target(bv.target) {
                done += bv.value;
            }
        }
        (done, taken)
    }
}
