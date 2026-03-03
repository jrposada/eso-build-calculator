use crate::domain::{BonusData, BonusSource, BonusTarget, BonusTrigger, BonusValue};
use once_cell::sync::Lazy;

pub static KHAJIIT_BONUSES: Lazy<Vec<BonusData>> = Lazy::new(|| {
    vec![
        BonusData::new(
            "Lunar Blessings (Magicka)",
            BonusSource::Passive,
            BonusTrigger::Passive,
            BonusValue::new("Max Magicka", BonusTarget::MaxMagickaFlat, 915.0),
        )
        .with_skill_id(117848),
        BonusData::new(
            "Lunar Blessings (Stamina)",
            BonusSource::Passive,
            BonusTrigger::Passive,
            BonusValue::new("Max Stamina", BonusTarget::MaxStaminaFlat, 915.0),
        )
        .with_skill_id(117848),
        BonusData::new(
            "Feline Ambush",
            BonusSource::Passive,
            BonusTrigger::Passive,
            BonusValue::new("Critical Damage", BonusTarget::CriticalDamage, 0.12),
        )
        .with_skill_id(45301),
    ]
});
