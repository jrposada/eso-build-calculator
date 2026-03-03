use crate::domain::{BonusData, BonusSource, BonusTarget, BonusTrigger, BonusValue};
use once_cell::sync::Lazy;

pub static NORD_BONUSES: Lazy<Vec<BonusData>> = Lazy::new(|| {
    vec![BonusData::new(
        "Stalwart",
        BonusSource::Passive,
        BonusTrigger::Passive,
        BonusValue::new("Max Stamina", BonusTarget::MaxStaminaFlat, 1500.0),
    )
    .with_skill_id(45298)]
});
