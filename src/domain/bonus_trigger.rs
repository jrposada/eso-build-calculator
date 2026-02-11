use serde::{Deserialize, Serialize};

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
    EnemyBurning,
    EnemyChilled,
    EnemyHasStatusEffect,
    EnemyOffBalance,
    EnemyPoisoned,
    Flanking,
    MagickaOrStaminaRestored,
    Passive,
    SkillLineSkillCast,
    SkillLineSlotted,
    TwoHandedEquipped,
}
