
use crate::domain::{BonusData, BonusSource, BonusTarget, BonusTrigger, BonusValue};
use once_cell::sync::Lazy;

pub static WOOD_ELF_BONUSES: Lazy<Vec<BonusData>> = Lazy::new(|| {
    vec![
        BonusData::new(
            "Hunter's Eye",
            BonusSource::Passive,
            BonusTrigger::Passive,
            BonusValue::new(
                "Penetration",
                BonusTarget::PhysicalAndSpellPenetration,
                950.0,
            ),
        )
        .with_skill_id(45296),
        BonusData::new(
            "Resist Affliction",
            BonusSource::Passive,
            BonusTrigger::Passive,
            BonusValue::new("Max Stamina", BonusTarget::MaxStaminaFlat, 2000.0),
        )
        .with_skill_id(45319),
    ]
});

