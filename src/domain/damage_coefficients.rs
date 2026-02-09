use serde::{Deserialize, Serialize};

/// Damage coefficients for skills, used to calculate base damage from character stats.
///
/// ESO skill damage is calculated as:
/// base_damage = coef_a * max_stat + coef_b * max_power
///
/// Where:
/// - max_stat = max(magicka, stamina)
/// - max_power = max(weapon_damage, spell_damage)
///
/// Coefficients can be found on UESP for each skill.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct DamageCoefficients {
    /// Coefficient for max_stat (magicka/stamina)
    pub coef_a: f64,
    /// Coefficient for max_power (weapon/spell damage)
    pub coef_b: f64,
}

impl DamageCoefficients {
    /// Create new damage coefficients
    pub fn new(coef_a: f64, coef_b: f64) -> Self {
        Self { coef_a, coef_b }
    }

    /// Calculate base damage from character stats
    ///
    /// # Arguments
    /// * `max_stat` - The higher of max_magicka and max_stamina
    /// * `max_power` - The higher of weapon_damage and spell_damage
    pub fn calculate_base_damage(&self, max_stat: f64, max_power: f64) -> f64 {
        self.coef_a * max_stat + self.coef_b * max_power
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_base_damage() {
        // Example: Lava Whip coefficients from UESP
        let coefficients = DamageCoefficients::new(0.09797, 1.02992);

        // With 40,000 magicka and 5,500 spell damage
        let base = coefficients.calculate_base_damage(40_000.0, 5_500.0);

        // Expected: 0.09797 * 40000 + 1.02992 * 5500 = 3918.8 + 5664.56 = 9583.36
        let expected = 0.09797 * 40_000.0 + 1.02992 * 5_500.0;
        assert!(
            (base - expected).abs() < 0.01,
            "Expected {}, got {}",
            expected,
            base
        );
    }

    #[test]
    fn test_zero_coefficients() {
        let coefficients = DamageCoefficients::new(0.0, 0.0);
        let base = coefficients.calculate_base_damage(40_000.0, 5_500.0);
        assert_eq!(base, 0.0);
    }

    #[test]
    fn test_only_stat_coefficient() {
        let coefficients = DamageCoefficients::new(0.1, 0.0);
        let base = coefficients.calculate_base_damage(40_000.0, 5_500.0);
        assert_eq!(base, 4_000.0);
    }

    #[test]
    fn test_only_power_coefficient() {
        let coefficients = DamageCoefficients::new(0.0, 1.0);
        let base = coefficients.calculate_base_damage(40_000.0, 5_500.0);
        assert_eq!(base, 5_500.0);
    }
}
