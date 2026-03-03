
use crate::domain::{BonusData, BonusSource, BonusTarget, BonusTrigger, BonusValue};
use once_cell::sync::Lazy;

pub static ARGONIAN_BONUSES: Lazy<Vec<BonusData>> = Lazy::new(|| {
    vec![
        BonusData::new(
            "Resourceful (Magicka)",
            BonusSource::Passive,
            BonusTrigger::Passive,
            BonusValue::new("Max Magicka", BonusTarget::MaxMagickaFlat, 1000.0),
        )
        .with_skill_id(45247),
        BonusData::new(
            "Resourceful (Stamina)",
            BonusSource::Passive,
            BonusTrigger::Passive,
            BonusValue::new("Max Stamina", BonusTarget::MaxStaminaFlat, 1000.0),
        )
        .with_skill_id(45247),
    ]
});

