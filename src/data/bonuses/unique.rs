use crate::data::{BonusTarget, BonusTrigger};
use crate::domain::BonusData;
use once_cell::sync::Lazy;

pub static MINOR_SAVAGERY: Lazy<BonusData> = Lazy::new(|| {
    BonusData::new(
        "Minor Savagery",
        BonusTrigger::Cast,
        BonusTarget::WeaponCriticalChance,
        1314.0,
    ).with_duration(20.0)
});

pub static MINOR_PROPHECY: Lazy<BonusData> = Lazy::new(|| {
    BonusData::new(
        "Minor Prophecy",
        BonusTrigger::Cast,
        BonusTarget::SpellCriticalChance,
        1314.0,
    )
});

pub static MINOR_BRUTALITY: Lazy<BonusData> = Lazy::new(|| {
    BonusData::new(
        "Minor Brutality",
        BonusTrigger::Cast,
        BonusTarget::WeaponDamage,
        0.1,
    )
});
