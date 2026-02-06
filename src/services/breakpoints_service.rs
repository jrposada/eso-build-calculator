use crate::domain::BonusTarget;

/// Maximum Critical Value at CP160 (level 66)
/// Formula: MCV = 2 × Level × (100 + Level) = 2 × 66 × 166 = 21912
/// Source: https://en.uesp.net/wiki/Online:Weapon_Critical_(effect)
pub const MAX_CRIT_VALUE_CP160: f64 = 21912.0;

/// Base critical chance all characters have (10%)
/// Source: https://en.uesp.net/wiki/Online:Weapon_Critical_(effect)
pub const BASE_CRIT_CHANCE: f64 = 0.10;

/// Converts critical rating to total critical chance (includes base 10%)
/// Formula: Crit% = min(100, 10 + 100 × (CritRating / MCV))
pub fn crit_rating_to_total_chance(crit_rating: f64) -> f64 {
    (BASE_CRIT_CHANCE + crit_rating / MAX_CRIT_VALUE_CP160).min(1.0)
}

/// Converts critical rating to bonus critical chance (excludes base 10%)
/// This is the additional crit chance provided by the rating alone.
pub fn crit_rating_to_bonus_chance(crit_rating: f64) -> f64 {
    crit_rating / MAX_CRIT_VALUE_CP160
}

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
                    alt_value / MAX_CRIT_VALUE_CP160
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
                    primary_value / MAX_CRIT_VALUE_CP160
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

    // ==================== CRIT RATING CONVERSION TESTS ====================
    // These tests verify MAX_CRIT_VALUE_CP160 is correct by checking known
    // crit rating to crit chance conversions from the game.
    // Source: https://en.uesp.net/wiki/Online:Weapon_Critical_(effect)

    #[test]
    fn test_uesp_example_3000_rating_gives_23_7_percent_total() {
        // From UESP: "If you are CP160 and have a Weapon Critical of 3000,
        // your Critical Strike Chance% = 10 + 100 × (3000 / 21912) = 23.7%"
        let total_chance = crit_rating_to_total_chance(3000.0);
        assert!(
            (total_chance - 0.237).abs() < 0.001,
            "Expected 23.7% total crit chance, got {}%",
            total_chance * 100.0
        );
    }

    #[test]
    fn test_zero_rating_gives_base_10_percent() {
        // With no crit rating, should have base 10% crit chance
        let total_chance = crit_rating_to_total_chance(0.0);
        assert!(
            (total_chance - 0.10).abs() < 0.0001,
            "Expected 10% base crit chance, got {}%",
            total_chance * 100.0
        );
    }

    #[test]
    fn test_max_rating_gives_100_percent_total() {
        // At MCV (21912), total should be 100% (10% base + 90% from rating)
        let total_chance = crit_rating_to_total_chance(MAX_CRIT_VALUE_CP160);
        assert!(
            (total_chance - 1.0).abs() < 0.0001,
            "Expected 100% total crit chance at MCV, got {}%",
            total_chance * 100.0
        );
    }

    #[test]
    fn test_total_chance_caps_at_100_percent() {
        // Even with excessive rating, should cap at 100%
        let total_chance = crit_rating_to_total_chance(30000.0);
        assert!(
            (total_chance - 1.0).abs() < 0.0001,
            "Expected 100% capped crit chance, got {}%",
            total_chance * 100.0
        );
    }

    #[test]
    fn test_bonus_chance_at_max_rating_gives_90_percent() {
        // At MCV, bonus (without base) should be ~90%
        // Actually at MCV it's exactly 100% bonus, base is separate
        let bonus_chance = crit_rating_to_bonus_chance(MAX_CRIT_VALUE_CP160);
        assert!(
            (bonus_chance - 1.0).abs() < 0.0001,
            "Expected 100% bonus crit chance at MCV, got {}%",
            bonus_chance * 100.0
        );
    }

    #[test]
    fn test_bonus_chance_1314_rating() {
        // 1314 rating (common passive value)
        // Bonus = 1314 / 21912 ≈ 6%
        let bonus_chance = crit_rating_to_bonus_chance(1314.0);
        let expected = 1314.0 / 21912.0;
        assert!(
            (bonus_chance - expected).abs() < 0.0001,
            "Expected ~6% bonus crit chance, got {}%",
            bonus_chance * 100.0
        );
    }

    // ==================== BREAKPOINT TESTS ====================

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
        let expected = 0.05 / (1314.0 / 21912.0);
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
        let expected = 0.05 / (1314.0 / 21912.0);
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
