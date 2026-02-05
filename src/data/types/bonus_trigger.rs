use serde::{Deserialize, Serialize};

/// How a bonus is applied
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum BonusTrigger {
    AbilitySlotted,
    AbilitySlottedCount,
    ArcanistCrux,
    BowEquipped,
    BurningOrPoisonDamageDealt,
    Cast,
    CriticalDamageDealt,
    DestructionStuffEquipped,
    DualWieldEquipped,
    Flanking,
    MagickaOrStaminaRestored,
    Passive,
    SkillLineSkillCast,
    SkillLineSlotted,
    TwoHandedEquipped,
    // Weapon type specific triggers
    TwoHandedSwordEquipped,
    TwoHandedAxeEquipped,
    TwoHandedMaceEquipped,
    DualWieldSwordEquipped,
    DualWieldAxeEquipped,
    DualWieldMaceEquipped,
    DualWieldDaggerEquipped,
    InfernoStaffEquipped,
    LightningStaffEquipped,
    IceStaffEquipped,
    // Status effect triggers
    EnemyOffBalance,
    EnemyChilled,
    EnemyBurning,
    EnemyPoisoned,
    EnemyHasStatusEffect,
}
