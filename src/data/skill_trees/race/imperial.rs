use crate::domain::{BonusData, BonusSource, BonusTarget, BonusTrigger, BonusValue};
use once_cell::sync::Lazy;

pub static IMPERIAL_BONUSES: Lazy<Vec<BonusData>> = Lazy::new(|| {
    vec![BonusData::new(
        "Imperial Mettle",
        BonusSource::Passive,
        BonusTrigger::Passive,
        BonusValue::new("Max Stamina", BonusTarget::MaxStaminaFlat, 2000.0),
    )
    .with_skill_id(45280)]
});
