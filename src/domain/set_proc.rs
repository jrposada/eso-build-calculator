use super::BonusTarget;
use super::DamageFlags;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SetProcTrigger {
    OnLightAttack,
    OnDirectDamage,
    OnDealDamage,
}

#[derive(Debug, Clone)]
pub enum SetProcAction {
    /// Instant hit + optional follow-up DoT
    DamageProc {
        hit_damage: f64,
        hit_flags: DamageFlags,
        dot_total_damage: f64,
        dot_duration: f64,
        dot_flags: DamageFlags,
    },
    /// Relequen-style: each trigger adds a stack, DoT damage scales with stack count
    StackingDot {
        damage_per_stack_per_tick: f64,
        tick_interval: f64,
        max_stacks: u32,
        stack_duration: f64,
        stack_cooldown: f64,
        flags: DamageFlags,
    },
    /// Kinras/Tzogvin-style: stacking buff, reward at max stacks
    StackingBuff {
        per_stack_target: Option<BonusTarget>,
        per_stack_value: f64,
        max_stacks: u32,
        stack_duration: f64,
        stack_cooldown: f64,
        at_max_buff_name: String,
        at_max_buff_target: BonusTarget,
        at_max_buff_value: f64,
        at_max_buff_duration: f64,
    },
    /// Flat bonus added to every light attack's base damage
    FlatLightAttackBonus {
        value: f64,
    },
}

#[derive(Debug, Clone)]
pub struct SetProcEffect {
    pub name: String,
    pub trigger: SetProcTrigger,
    pub action: SetProcAction,
    pub cooldown: f64,
}

impl SetProcEffect {
    /// Rough DPS estimate for set optimizer scoring (before modifiers).
    pub fn estimated_dps(&self) -> f64 {
        let modifier_estimate = 2.5;
        match &self.action {
            SetProcAction::StackingDot {
                damage_per_stack_per_tick,
                tick_interval,
                max_stacks,
                ..
            } => {
                damage_per_stack_per_tick * (*max_stacks as f64) / tick_interval * modifier_estimate
            }
            SetProcAction::DamageProc {
                hit_damage,
                dot_total_damage,
                ..
            } => {
                let cd = if self.cooldown > 0.0 { self.cooldown } else { 1.0 };
                (hit_damage + dot_total_damage) / cd * modifier_estimate
            }
            SetProcAction::StackingBuff {
                at_max_buff_value, ..
            } => {
                // Estimated base DPS × buff % × uptime
                80_000.0 * at_max_buff_value * 0.8
            }
            SetProcAction::FlatLightAttackBonus { value } => value * modifier_estimate,
        }
    }
}
