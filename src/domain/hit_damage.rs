use super::{DamageCoefficients, DamageFlags};
use serde::{Deserialize, Serialize};

/// Hit damage data
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HitDamage {
    pub flags: DamageFlags,
    pub coefficients: DamageCoefficients,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delay: Option<f64>,
    /// Only applies when enemy HP is below this threshold (0.0-1.0)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execute_threshold: Option<f64>,
}

impl HitDamage {
    pub fn new(flags: DamageFlags, coef_a: f64, coef_b: f64) -> Self {
        Self {
            flags: flags | DamageFlags::DIRECT,
            coefficients: DamageCoefficients::new(coef_a, coef_b),
            delay: None,
            execute_threshold: None,
        }
    }

    pub fn with_delay(mut self, delay: f64) -> Self {
        self.delay = Some(delay);
        self
    }

    pub fn with_execute_threshold(mut self, threshold: f64) -> Self {
        self.execute_threshold = Some(threshold);
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
