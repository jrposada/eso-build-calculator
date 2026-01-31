use crate::data::passives::{ALL_CLASS_PASSIVES, ALL_WEAPON_PASSIVES};
use crate::data::{BonusClassName, BonusType, SkillLineName};
use crate::domain::{BonusData, PassiveData};

// Base crit stats (assumed from gear/CP) - could be made configurable
const BASE_CRIT_CHANCE: f64 = 0.15; // 15% base crit chance
const BASE_CRIT_DAMAGE: f64 = 0.5; // 50% base crit damage

/// Get all passives for a specific skill line
pub fn get_passives_by_skill_line(skill_line: SkillLineName) -> Vec<PassiveData> {
    let class_passives: Vec<PassiveData> = ALL_CLASS_PASSIVES
        .iter()
        .filter(|p| p.skill_line == skill_line)
        .map(|p| (*p).clone())
        .collect();

    if !class_passives.is_empty() {
        return class_passives;
    }

    ALL_WEAPON_PASSIVES
        .iter()
        .filter(|p| p.skill_line == skill_line)
        .map(|p| (*p).clone())
        .collect()
}

/// Check if a bonus applies to a skill and return the applicable bonus value
fn get_applicable_bonus(bonus: &BonusData, skill_line_count: usize) -> f64 {
    // Get base value and apply multiplier based on className
    let multiplied_value = match bonus.class_name {
        BonusClassName::SkillLine | BonusClassName::AbilitySlotted => {
            if skill_line_count > 0 {
                bonus.value
            } else {
                0.0
            }
        }
        BonusClassName::AbilitySlottedCount => bonus.value * skill_line_count as f64,
        BonusClassName::Passive => bonus.value, // Always applied
        BonusClassName::Duration => {
            // Duration buffs not yet implemented
            return 0.0;
        }
    };

    if multiplied_value == 0.0 {
        return 0.0;
    }

    // Convert stat types to expected damage bonus
    match bonus.bonus_type {
        BonusType::CriticalChance
        | BonusType::SpellCriticalChance
        | BonusType::WeaponCriticalChance => {
            // More crit chance = more expected damage: crit_chance_increase * crit_damage
            multiplied_value * (1.0 + BASE_CRIT_DAMAGE)
        }
        BonusType::CriticalDamage => {
            // More crit damage = more expected damage: crit_chance * crit_damage_increase
            BASE_CRIT_CHANCE * multiplied_value
        }
        BonusType::Duration
        | BonusType::MaxStamina
        | BonusType::MaxMagicka
        | BonusType::SpellDamage => {
            // These don't directly affect damage yet (could be expanded later)
            0.0
        }
        _ => 0.0,
    }
}

/// Calculate total passive bonus percentage for a skill
pub fn calculate_passive_bonus(passives: &[PassiveData], skill_line_count: usize) -> f64 {
    let mut total_bonus = 0.0;

    for passive in passives {
        for bonus in &passive.bonuses {
            total_bonus += get_applicable_bonus(bonus, skill_line_count);
        }
    }

    total_bonus
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_passives_by_skill_line() {
        let passives = get_passives_by_skill_line(SkillLineName::ArdentFlame);
        assert!(!passives.is_empty());
        assert!(passives
            .iter()
            .all(|p| p.skill_line == SkillLineName::ArdentFlame));
    }

    #[test]
    fn test_calculate_passive_bonus_no_skills() {
        let passives = get_passives_by_skill_line(SkillLineName::Assassination);
        let bonus = calculate_passive_bonus(&passives, 0);
        // With no skills, most bonuses should be 0
        assert!(bonus >= 0.0);
    }

    #[test]
    fn test_calculate_passive_bonus_with_skills() {
        let passives = get_passives_by_skill_line(SkillLineName::Assassination);
        let bonus = calculate_passive_bonus(&passives, 3);
        // With skills, we should get some bonus from crit passives
        assert!(bonus > 0.0);
    }
}
