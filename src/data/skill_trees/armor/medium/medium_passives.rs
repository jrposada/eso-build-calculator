use crate::domain::{BonusData, BonusSource, BonusTarget, BonusTrigger, BonusValue};
use once_cell::sync::Lazy;

pub static MEDIUM_ARMOR_PASSIVES: Lazy<Vec<BonusData>> = Lazy::new(|| {
    vec![
        BonusData::new(
            "Agility",
            BonusSource::Passive,
            BonusTrigger::Passive,
            BonusValue::new("Agility", BonusTarget::WeaponAndSpellDamageMultiplier, 0.02),
        )
        .with_skill_id(45572),
        BonusData::new(
            "Dexterity",
            BonusSource::Passive,
            BonusTrigger::Passive,
            BonusValue::new("Dexterity", BonusTarget::CriticalDamage, 0.02),
        )
        .with_skill_id(45564),
    ]
});

