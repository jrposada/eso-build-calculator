use crate::domain::{BonusData, BonusSource, BonusTarget, BonusTrigger, BonusValue};
use once_cell::sync::Lazy;

pub static LIGHT_ARMOR_PASSIVES: Lazy<Vec<BonusData>> = Lazy::new(|| {
    vec![
        BonusData::new(
            "Prodigy",
            BonusSource::Passive,
            BonusTrigger::Passive,
            BonusValue::new("Prodigy", BonusTarget::CriticalRating, 219.0),
        )
        .with_skill_id(45561),
        BonusData::new(
            "Concentration",
            BonusSource::Passive,
            BonusTrigger::Passive,
            BonusValue::new(
                "Concentration",
                BonusTarget::PhysicalAndSpellPenetration,
                939.0,
            ),
        )
        .with_skill_id(45562),
    ]
});
