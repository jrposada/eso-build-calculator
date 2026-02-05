use serde::{Deserialize, Serialize};

/// What stat the bonus affects
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum BonusTarget {
    AoeDamage,
    Damage,
    BurningAndPoisonDamage,
    CriticalChance,
    CriticalDamage,
    DirectDamage,
    DotDamage,
    DurationSkillLineFlat,
    DurationSkillLineMultiplier,
    MaxMagicka,
    MaxStamina,
    OffBalanceDamage,
    PhysicalAndSpellPenetration,
    PhysicalDamage,
    RestoreMagickaOrStamina,
    ShockDamage,
    FrostDamage,
    FlameDamage,
    SingleDamage,
    SpellCriticalChance,
    WeaponAndSpellDamageFlat,
    WeaponAndSpellDamageMultiplier,
    WeaponCriticalChance,
    WeaponDamage,
    SpellDamage,
    HeavyAttackDamage,
    StatusEffectDamage,
    StatusEffectChance,
    ChilledStatusEffectChance,
    ChilledStatusEffectDamage,
    // Enemy debuffs
    EnemyDamageTaken,
    EnemyResistanceReduction,
}
