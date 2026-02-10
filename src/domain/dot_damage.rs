use super::{DamageCoefficients, DamageFlags};
use serde::{Deserialize, Serialize};

/// DoT (Damage over Time) damage data
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DotDamage {
    pub duration: f64,
    pub flags: DamageFlags,
    pub coefficients: DamageCoefficients,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delay: Option<f64>,
    /// Defaults to duration if not specified (total damage over duration)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<f64>,
    /// Percentage increase per tick (e.g., 0.12 for 12%)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub increase_per_tick: Option<f64>,
    /// Flat increase per tick
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flat_increase_per_tick: Option<f64>,
    /// If true, this damage ignores modifiers
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignores_modifier: Option<bool>,
}

impl DotDamage {
    pub fn new(duration: f64, flags: DamageFlags, coef_a: f64, coef_b: f64) -> Self {
        Self {
            duration,
            flags: flags | DamageFlags::DOT,
            coefficients: DamageCoefficients::new(coef_a, coef_b),
            delay: None,
            interval: None,
            increase_per_tick: None,
            flat_increase_per_tick: None,
            ignores_modifier: None,
        }
    }

    pub fn with_interval(mut self, interval: f64) -> Self {
        self.interval = Some(interval);
        self
    }

    pub fn with_increase_per_tick(mut self, increase: f64) -> Self {
        self.increase_per_tick = Some(increase);
        self
    }

    pub fn with_flat_increase_per_tick(mut self, increase: f64) -> Self {
        self.flat_increase_per_tick = Some(increase);
        self
    }

    pub fn with_delay(mut self, delay: f64) -> Self {
        self.delay = Some(delay);
        self
    }

    pub fn ignores_modifier(mut self) -> Self {
        self.ignores_modifier = Some(true);
        self
    }

    /// Calculate damage value from character stats
    ///
    /// # Arguments
    /// * `max_stat` - The higher of max_magicka and max_stamina
    /// * `max_power` - The higher of weapon_damage and spell_damage
    pub fn effective_value(&self, max_stat: f64, max_power: f64) -> f64 {
        self.coefficients.calculate_base_damage(max_stat, max_power)
    }
}
