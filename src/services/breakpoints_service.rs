use crate::domain::{formulas, BonusTarget};

/// Service for breakpoint calculations
pub struct BreakpointsService;

impl BreakpointsService {
    /// Calculate the crit damage breakpoint where two bonus options provide equal expected damage.
    ///
    /// For a flat damage bonus vs a crit rating bonus:
    /// - Flat damage: expected_multiplier = 1 + flat_value
    /// - Crit rating: expected_multiplier = 1 + (crit_rating / 21912) * crit_damage
    ///
    /// At breakpoint: 1 + flat_value = 1 + crit_increase * crit_damage
    /// => crit_damage = flat_value / crit_increase
    ///
    /// Below breakpoint: flat damage is better (crit damage too low to leverage crit rating)
    /// Above breakpoint: crit rating is better (high crit damage amplifies crit rating value)
    ///
    /// Returns None if breakpoint cannot be calculated (e.g., incompatible bonus types)
    pub fn calculate_breakpoint(
        primary_target: BonusTarget,
        primary_value: f64,
        alt_target: BonusTarget,
        alt_value: f64,
    ) -> Option<f64> {
        // Currently only supports Damage vs CriticalChance comparison
        match (primary_target, alt_target) {
            (BonusTarget::Damage, BonusTarget::CriticalChance) => {
                // Primary is flat damage, alternative is crit rating
                // Breakpoint: flat_value = crit_increase * crit_damage
                // crit_damage = flat_value / crit_increase
                let crit_increase = if alt_value > 1.0 {
                    alt_value / formulas::MAX_CRIT_VALUE_CP160
                } else {
                    alt_value
                };
                if crit_increase > 0.0 {
                    Some(primary_value / crit_increase)
                } else {
                    None
                }
            }
            (BonusTarget::CriticalChance, BonusTarget::Damage) => {
                // Primary is crit rating, alternative is flat damage
                // Breakpoint: crit_increase * crit_damage = flat_value
                // crit_damage = flat_value / crit_increase
                let crit_increase = if primary_value > 1.0 {
                    primary_value / formulas::MAX_CRIT_VALUE_CP160
                } else {
                    primary_value
                };
                if crit_increase > 0.0 {
                    Some(alt_value / crit_increase)
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_breakpoint_flat_vs_crit_rating() {
        // 5% flat damage vs 1314 crit rating
        // Breakpoint: 0.05 / (1314/21912) = 0.05 / 0.05997 ≈ 0.834
        let breakpoint = BreakpointsService::calculate_breakpoint(
            BonusTarget::Damage,
            0.05,
            BonusTarget::CriticalChance,
            1314.0,
        );

        assert!(breakpoint.is_some());
        let bp = breakpoint.unwrap();
        let expected = 0.05 / (1314.0 / formulas::MAX_CRIT_VALUE_CP160);
        assert!(
            (bp - expected).abs() < 0.001,
            "Expected breakpoint ~{}, got {}",
            expected,
            bp
        );
    }

    #[test]
    fn test_breakpoint_crit_rating_vs_flat() {
        // Reverse order: 1314 crit rating vs 5% flat damage
        let breakpoint = BreakpointsService::calculate_breakpoint(
            BonusTarget::CriticalChance,
            1314.0,
            BonusTarget::Damage,
            0.05,
        );

        assert!(breakpoint.is_some());
        let bp = breakpoint.unwrap();
        let expected = 0.05 / (1314.0 / formulas::MAX_CRIT_VALUE_CP160);
        assert!(
            (bp - expected).abs() < 0.001,
            "Expected breakpoint ~{}, got {}",
            expected,
            bp
        );
    }

    #[test]
    fn test_breakpoint_unsupported_types() {
        // CriticalDamage vs Damage - not supported
        let breakpoint = BreakpointsService::calculate_breakpoint(
            BonusTarget::CriticalDamage,
            0.10,
            BonusTarget::Damage,
            0.05,
        );

        assert!(breakpoint.is_none());
    }
}
