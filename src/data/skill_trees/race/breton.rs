use crate::domain::{BonusData, BonusSource, BonusTarget, BonusTrigger, BonusValue};
use once_cell::sync::Lazy;

pub static BRETON_BONUSES: Lazy<Vec<BonusData>> = Lazy::new(|| {
    vec![BonusData::new(
        "Gift of Magnus",
        BonusSource::Passive,
        BonusTrigger::Passive,
        BonusValue::new("Max Magicka", BonusTarget::MaxMagickaFlat, 2000.0),
    )
    .with_skill_id(45260)]
});
