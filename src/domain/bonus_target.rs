use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum BonusTarget {
    AoeDamage,
    BurningAndPoisonDamage,
    ChilledStatusEffectChance,
    ChilledStatusEffectDamage,
    CriticalRating,
    CriticalDamage,
    Damage,
    DirectDamage,
    DotDamage,
    DurationSkillLineFlat,
    DurationSkillLineMultiplier,
    EnemyDamageTaken,
    EnemyResistanceReduction,
    FlameDamage,
    FrostDamage,
    HeavyAttackDamage,
    MaxMagicka,
    MaxStamina,
    OffBalanceDamage,
    PhysicalAndSpellPenetration,
    PhysicalDamage,
    RestoreMagickaOrStamina,
    ShockDamage,
    SingleDamage,
    SpellCriticalRating,
    SpellDamage,
    StatusEffectChance,
    StatusEffectDamage,
    WeaponAndSpellDamageFlat,
    WeaponAndSpellDamageMultiplier,
    WeaponCriticalRating,
    WeaponDamage,
}

impl fmt::Display for BonusTarget {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            BonusTarget::AoeDamage => "AoE Damage",
            BonusTarget::BurningAndPoisonDamage => "Burning & Poison Damage",
            BonusTarget::ChilledStatusEffectChance => "Chilled Status Chance",
            BonusTarget::ChilledStatusEffectDamage => "Chilled Status Damage",
            BonusTarget::CriticalRating => "Critical Rating",
            BonusTarget::CriticalDamage => "Critical Damage",
            BonusTarget::Damage => "Damage",
            BonusTarget::DirectDamage => "Direct Damage",
            BonusTarget::DotDamage => "DoT Damage",
            BonusTarget::DurationSkillLineFlat => "Duration (Flat)",
            BonusTarget::DurationSkillLineMultiplier => "Duration (Mult)",
            BonusTarget::EnemyDamageTaken => "Enemy Damage Taken",
            BonusTarget::EnemyResistanceReduction => "Enemy Resistance Reduction",
            BonusTarget::FlameDamage => "Flame Damage",
            BonusTarget::FrostDamage => "Frost Damage",
            BonusTarget::HeavyAttackDamage => "Heavy Attack Damage",
            BonusTarget::MaxMagicka => "Max Magicka",
            BonusTarget::MaxStamina => "Max Stamina",
            BonusTarget::OffBalanceDamage => "Off Balance Damage",
            BonusTarget::PhysicalAndSpellPenetration => "Phys & Spell Penetration",
            BonusTarget::PhysicalDamage => "Physical Damage",
            BonusTarget::RestoreMagickaOrStamina => "Restore Magicka/Stamina",
            BonusTarget::ShockDamage => "Shock Damage",
            BonusTarget::SingleDamage => "Single Target Damage",
            BonusTarget::SpellCriticalRating => "Spell Critical Rating",
            BonusTarget::SpellDamage => "Spell Damage",
            BonusTarget::StatusEffectChance => "Status Effect Chance",
            BonusTarget::StatusEffectDamage => "Status Effect Damage",
            BonusTarget::WeaponAndSpellDamageFlat => "Wpn & Spell Damage (Flat)",
            BonusTarget::WeaponAndSpellDamageMultiplier => "Wpn & Spell Damage (Mult)",
            BonusTarget::WeaponCriticalRating => "Weapon Critical Rating",
            BonusTarget::WeaponDamage => "Weapon Damage",
        };
        write!(f, "{}", s)
    }
}
