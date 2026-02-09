//! ESO attribute conversion formulas and constants.
//!
//! This module contains all the mathematical formulas used to convert
//! between different ESO attribute representations (ratings to percentages, etc.)

// ==================== CONSTANTS ====================

/// Maximum Critical Value at CP160 (level 66)
/// Formula: MCV = 2 × Level × (100 + Level) = 2 × 66 × 166 = 21912
/// Source: https://en.uesp.net/wiki/Online:Weapon_Critical_(effect)
pub const MAX_CRIT_VALUE_CP160: f64 = 21912.0;

/// Base critical chance all characters have (10%)
/// Source: https://en.uesp.net/wiki/Online:Weapon_Critical_(effect)
pub const BASE_CRIT_CHANCE: f64 = 0.10;

/// Armor formula constant: 50 × level = 50 × 66 = 3300
/// Used in armor mitigation calculations at CP160
pub const ARMOR_LEVEL_CONSTANT: f64 = 3300.0;

// ==================== CRITICAL ====================

/// Converts critical rating to total critical chance (includes base 10%)
/// Formula: Crit% = min(100, 10 + 100 × (CritRating / MCV))
pub fn crit_rating_to_chance(crit_rating: f64) -> f64 {
    (BASE_CRIT_CHANCE + crit_rating / MAX_CRIT_VALUE_CP160).min(1.0)
}

/// Converts critical rating to bonus critical chance (excludes base 10%)
/// This is the additional crit chance provided by the rating alone.
pub fn crit_rating_to_bonus_chance(crit_rating: f64) -> f64 {
    crit_rating / MAX_CRIT_VALUE_CP160
}

// ==================== ARMOR ====================

/// Calculate armor mitigation percentage.
/// Formula: Mitigation = armor / (armor + 3300)
/// This is an asymptotic formula - you get diminishing returns as armor increases.
/// At 33000 armor, mitigation is ~90.9%
pub fn armor_to_mitigation(armor: f64) -> f64 {
    if armor <= 0.0 {
        return 0.0;
    }
    armor / (armor + ARMOR_LEVEL_CONSTANT)
}

/// Calculate effective armor after penetration is applied.
/// Penetration reduces enemy armor, but cannot go below 0.
pub fn effective_armor(armor: f64, penetration: f64) -> f64 {
    (armor - penetration).max(0.0)
}

// ==================== RESOURCE TO DAMAGE ====================

/// Convert stamina/magicka resource to equivalent weapon/spell damage bonus.
/// ESO formula: every 10.5 stamina/magicka = 1 weapon/spell damage
pub fn resource_to_damage_bonus(resource: f64) -> f64 {
    resource / 10.5
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
        let total_chance = crit_rating_to_chance(3000.0);
        assert!(
            (total_chance - 0.237).abs() < 0.001,
            "Expected 23.7% total crit chance, got {}%",
            total_chance * 100.0
        );
    }

    #[test]
    fn test_zero_rating_gives_base_10_percent() {
        // With no crit rating, should have base 10% crit chance
        let total_chance = crit_rating_to_chance(0.0);
        assert!(
            (total_chance - 0.10).abs() < 0.0001,
            "Expected 10% base crit chance, got {}%",
            total_chance * 100.0
        );
    }

    #[test]
    fn test_max_rating_gives_100_percent_total() {
        // At MCV (21912), total should be 100% (10% base + 90% from rating)
        let total_chance = crit_rating_to_chance(MAX_CRIT_VALUE_CP160);
        assert!(
            (total_chance - 1.0).abs() < 0.0001,
            "Expected 100% total crit chance at MCV, got {}%",
            total_chance * 100.0
        );
    }

    #[test]
    fn test_total_chance_caps_at_100_percent() {
        // Even with excessive rating, should cap at 100%
        let total_chance = crit_rating_to_chance(30000.0);
        assert!(
            (total_chance - 1.0).abs() < 0.0001,
            "Expected 100% capped crit chance, got {}%",
            total_chance * 100.0
        );
    }

    #[test]
    fn test_bonus_chance_at_max_rating_gives_100_percent() {
        // At MCV, bonus (without base) should be 100%
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

    // ==================== ARMOR TESTS ====================

    #[test]
    fn test_armor_mitigation_at_zero() {
        let mitigation = armor_to_mitigation(0.0);
        assert!(
            mitigation.abs() < 0.0001,
            "Expected 0% mitigation at 0 armor, got {}%",
            mitigation * 100.0
        );
    }

    #[test]
    fn test_armor_mitigation_at_3300() {
        // At armor = constant (3300), mitigation = 50%
        let mitigation = armor_to_mitigation(ARMOR_LEVEL_CONSTANT);
        assert!(
            (mitigation - 0.5).abs() < 0.0001,
            "Expected 50% mitigation at 3300 armor, got {}%",
            mitigation * 100.0
        );
    }

    #[test]
    fn test_armor_mitigation_at_33000() {
        // At 33000 armor: 33000 / (33000 + 3300) = 33000 / 36300 ≈ 90.9%
        let mitigation = armor_to_mitigation(33000.0);
        let expected = 33000.0 / 36300.0;
        assert!(
            (mitigation - expected).abs() < 0.0001,
            "Expected ~90.9% mitigation at 33000 armor, got {}%",
            mitigation * 100.0
        );
    }

    #[test]
    fn test_armor_mitigation_negative_armor() {
        // Negative armor should return 0 mitigation
        let mitigation = armor_to_mitigation(-1000.0);
        assert!(
            mitigation.abs() < 0.0001,
            "Expected 0% mitigation for negative armor, got {}%",
            mitigation * 100.0
        );
    }

    // ==================== PENETRATION TESTS ====================

    #[test]
    fn test_effective_armor_basic() {
        let effective = effective_armor(20000.0, 5000.0);
        assert!(
            (effective - 15000.0).abs() < 0.0001,
            "Expected 15000 effective armor, got {}",
            effective
        );
    }

    #[test]
    fn test_effective_armor_full_penetration() {
        // Penetration exceeds armor - should clamp to 0
        let effective = effective_armor(10000.0, 15000.0);
        assert!(
            effective.abs() < 0.0001,
            "Expected 0 effective armor when pen > armor, got {}",
            effective
        );
    }

    #[test]
    fn test_effective_armor_no_penetration() {
        let effective = effective_armor(18000.0, 0.0);
        assert!(
            (effective - 18000.0).abs() < 0.0001,
            "Expected unchanged armor with no pen, got {}",
            effective
        );
    }

    // ==================== RESOURCE TO DAMAGE TESTS ====================

    #[test]
    fn test_resource_to_damage_basic() {
        // 10.5 stamina = 1 weapon damage
        let damage = resource_to_damage_bonus(10.5);
        assert!(
            (damage - 1.0).abs() < 0.0001,
            "Expected 1 damage bonus from 10.5 resource, got {}",
            damage
        );
    }

    #[test]
    fn test_resource_to_damage_typical_pool() {
        // 30000 stamina / 10.5 ≈ 2857 damage bonus
        let damage = resource_to_damage_bonus(30000.0);
        let expected = 30000.0 / 10.5;
        assert!(
            (damage - expected).abs() < 0.01,
            "Expected ~2857 damage bonus from 30000 resource, got {}",
            damage
        );
    }
}
