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

// ==================== CRITICAL DAMAGE ====================

/// Calculate average critical multiplier for damage calculations.
/// This represents the expected damage multiplier accounting for crit chance.
///
/// Formula: 1 + (crit_chance * (crit_damage - 1))
///
/// Example: With 60% crit chance and 1.75 crit damage (75% bonus):
/// 1 + (0.60 * (1.75 - 1)) = 1 + (0.60 * 0.75) = 1.45 (45% average damage increase)
pub fn critical_multiplier(crit_chance: f64, crit_damage: f64) -> f64 {
    1.0 + (crit_chance * (crit_damage - 1.0))
}

// ==================== ARMOR DAMAGE FACTOR ====================

/// Calculate damage factor after armor mitigation.
/// Returns the fraction of damage that passes through armor.
///
/// Formula: 1 - mitigation = 1 - (effective_armor / (effective_armor + 3300))
///
/// Where effective_armor = max(0, target_armor - penetration)
pub fn armor_damage_factor(target_armor: f64, penetration: f64) -> f64 {
    let eff_armor = effective_armor(target_armor, penetration);
    1.0 - armor_to_mitigation(eff_armor)
}

// ==================== FINAL DAMAGE CALCULATION ====================

/// Calculate final damage combining all factors.
///
/// Formula:
/// modified_damage = base_damage * (1 + modifier_sum)
/// armor_factor = 1 - (effective_armor / (effective_armor + 3300))
/// crit_multiplier = 1 + (crit_chance * (crit_damage - 1))
/// final_damage = modified_damage * armor_factor * crit_multiplier
///
/// # Arguments
/// * `base_damage` - Raw damage from skill (tooltip or coefficient-calculated)
/// * `modifier_sum` - Sum of all applicable damage modifiers (e.g., 0.15 for +15%)
/// * `target_armor` - Target's armor value
/// * `penetration` - Character's armor penetration
/// * `crit_chance` - Critical strike chance (0.0 - 1.0)
/// * `crit_damage` - Critical damage multiplier (e.g., 1.75 for 75% bonus)
pub fn calculate_final_damage(
    base_damage: f64,
    modifier_sum: f64,
    target_armor: f64,
    penetration: f64,
    crit_chance: f64,
    crit_damage: f64,
) -> f64 {
    let modified_damage = base_damage * (1.0 + modifier_sum);
    let armor_factor = armor_damage_factor(target_armor, penetration);
    let crit_mult = critical_multiplier(crit_chance, crit_damage);
    modified_damage * armor_factor * crit_mult
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

    // ==================== CRITICAL MULTIPLIER TESTS ====================

    #[test]
    fn test_critical_multiplier_typical_values() {
        // 60% crit chance, 1.75 crit damage (75% bonus)
        // Expected: 1 + (0.60 * 0.75) = 1.45
        let mult = critical_multiplier(0.60, 1.75);
        assert!(
            (mult - 1.45).abs() < 0.0001,
            "Expected 1.45 crit multiplier, got {}",
            mult
        );
    }

    #[test]
    fn test_critical_multiplier_no_crit() {
        // 0% crit chance should give 1.0 multiplier
        let mult = critical_multiplier(0.0, 2.0);
        assert!(
            (mult - 1.0).abs() < 0.0001,
            "Expected 1.0 multiplier with 0% crit, got {}",
            mult
        );
    }

    #[test]
    fn test_critical_multiplier_100_percent_crit() {
        // 100% crit chance, 2.0 crit damage
        // Expected: 1 + (1.0 * 1.0) = 2.0
        let mult = critical_multiplier(1.0, 2.0);
        assert!(
            (mult - 2.0).abs() < 0.0001,
            "Expected 2.0 multiplier with 100% crit, got {}",
            mult
        );
    }

    // ==================== ARMOR DAMAGE FACTOR TESTS ====================

    #[test]
    fn test_armor_damage_factor_equal_pen_armor() {
        // When penetration equals armor, all damage passes through
        let factor = armor_damage_factor(18200.0, 18200.0);
        assert!(
            (factor - 1.0).abs() < 0.0001,
            "Expected 1.0 damage factor when pen = armor, got {}",
            factor
        );
    }

    #[test]
    fn test_armor_damage_factor_no_pen() {
        // 18200 armor, 0 penetration
        // mitigation = 18200 / (18200 + 3300) = 18200 / 21500 ≈ 0.847
        // factor = 1 - 0.847 ≈ 0.153
        let factor = armor_damage_factor(18200.0, 0.0);
        let expected = 1.0 - (18200.0 / 21500.0);
        assert!(
            (factor - expected).abs() < 0.0001,
            "Expected {} damage factor, got {}",
            expected,
            factor
        );
    }

    #[test]
    fn test_armor_damage_factor_over_pen() {
        // Penetration exceeds armor
        let factor = armor_damage_factor(10000.0, 15000.0);
        assert!(
            (factor - 1.0).abs() < 0.0001,
            "Expected 1.0 damage factor when pen > armor, got {}",
            factor
        );
    }

    // ==================== FINAL DAMAGE CALCULATION TESTS ====================

    #[test]
    fn test_calculate_final_damage_basic() {
        // Simple case: 1000 base, no modifiers, no armor, no crit
        let damage = calculate_final_damage(1000.0, 0.0, 0.0, 0.0, 0.0, 1.0);
        assert!(
            (damage - 1000.0).abs() < 0.01,
            "Expected 1000 damage, got {}",
            damage
        );
    }

    #[test]
    fn test_calculate_final_damage_with_modifiers() {
        // 1000 base, 15% modifier, no armor, no crit
        let damage = calculate_final_damage(1000.0, 0.15, 0.0, 0.0, 0.0, 1.0);
        assert!(
            (damage - 1150.0).abs() < 0.01,
            "Expected 1150 damage with 15% modifier, got {}",
            damage
        );
    }

    #[test]
    fn test_calculate_final_damage_full_calculation() {
        // Realistic scenario:
        // 10000 base, 20% modifier, 18200 armor, 18200 pen (full pen), 60% crit, 1.75 crit damage
        // modified = 10000 * 1.2 = 12000
        // armor_factor = 1.0 (full penetration)
        // crit_mult = 1 + (0.6 * 0.75) = 1.45
        // final = 12000 * 1.0 * 1.45 = 17400
        let damage = calculate_final_damage(10000.0, 0.20, 18200.0, 18200.0, 0.60, 1.75);
        let expected = 10000.0 * 1.2 * 1.45;
        assert!(
            (damage - expected).abs() < 0.01,
            "Expected {} damage, got {}",
            expected,
            damage
        );
    }
}
