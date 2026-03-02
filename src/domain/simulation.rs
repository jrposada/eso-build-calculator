use super::BonusTarget;
use super::DamageCoefficients;
use super::DamageFlags;

/// Global cooldown in seconds (1 GCD per action)
pub const GCD: f64 = 1.0;

/// Bar swap animation delay in seconds
pub const BAR_SWAP_DELAY: f64 = 0.3;

/// Trial dummy HP (21 million)
pub const TRIAL_DUMMY_HP: f64 = 21_000_000.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActiveBar {
    Bar1,
    Bar2,
}

impl ActiveBar {
    pub fn opposite(&self) -> Self {
        match self {
            ActiveBar::Bar1 => ActiveBar::Bar2,
            ActiveBar::Bar2 => ActiveBar::Bar1,
        }
    }
}

/// Tracks a running DoT effect on the target.
#[derive(Debug, Clone)]
pub struct ActiveEffect {
    pub source_skill_name: String,
    pub remaining_duration: f64,
    pub next_tick_in: f64,
    pub tick_interval: f64,
    pub tick_count: i32,
    pub total_ticks: i32,
    pub base_value: f64,
    pub flags: DamageFlags,
    pub coefficients: DamageCoefficients,
    pub increase_per_tick: f64,
    pub flat_increase_per_tick: f64,
    pub ignores_modifier: bool,
    /// Damage-done modifier sum snapshotted at cast time (DoT ticks use this, not current buffs)
    pub snapshotted_done_modifier: f64,
    /// EnemyDamageTaken modifier sum snapshotted at cast time (separate multiplicative layer)
    pub snapshotted_taken_modifier: f64,
    /// Armor damage factor snapshotted at cast time
    pub snapshotted_armor_factor: f64,
    /// Critical multiplier snapshotted at cast time
    pub snapshotted_crit_mult: f64,
}

/// Tracks an active buff granted by a skill.
#[derive(Debug, Clone)]
pub struct ActiveBuff {
    /// Dedup key (e.g., "Major Brutality")
    pub name: String,
    /// Which skill granted this buff (for rotation priority checks)
    pub source_skill_name: String,
    /// None = permanent (AbilitySlotted)
    pub remaining_duration: Option<f64>,
    pub target: BonusTarget,
    pub value: f64,
}

/// Results of a fight simulation.
#[derive(Debug, Clone)]
pub struct SimulationResult {
    pub total_damage: f64,
    pub fight_duration: f64,
    pub dps: f64,
    pub skill_breakdown: Vec<SkillBreakdown>,
    pub la_damage: f64,
    pub la_count: u32,
    pub bar_swap_count: u32,
}

#[derive(Debug, Clone)]
pub struct SkillBreakdown {
    pub skill_name: String,
    pub damage: f64,
    pub cast_count: u32,
}
