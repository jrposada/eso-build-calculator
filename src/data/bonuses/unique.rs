use crate::data::{BonusTarget, BonusType};
use crate::domain::BonusData;
use once_cell::sync::Lazy;

pub static MINOR_SAVAGERY: Lazy<BonusData> = Lazy::new(|| {
    BonusData::new(
        "Minor Savagery",
        BonusType::Duration,
        BonusTarget::WeaponCriticalChance,
        1314.0,
    )
});

pub static MINOR_PROPHECY: Lazy<BonusData> = Lazy::new(|| {
    BonusData::new(
        "Minor Prophecy",
        BonusType::Duration,
        BonusTarget::SpellCriticalChance,
        1314.0,
    )
});
