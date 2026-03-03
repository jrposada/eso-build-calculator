
use crate::domain::{BonusData, BonusSource, BonusTarget, BonusTrigger, BonusValue};
use once_cell::sync::Lazy;

pub static ORC_BONUSES: Lazy<Vec<BonusData>> = Lazy::new(|| {
    vec![
        BonusData::new(
            "Brawny",
            BonusSource::Passive,
            BonusTrigger::Passive,
            BonusValue::new("Max Stamina", BonusTarget::MaxStaminaFlat, 1000.0),
        )
        .with_skill_id(45309),
        BonusData::new(
            "Swift Warrior",
            BonusSource::Passive,
            BonusTrigger::Passive,
            BonusValue::new(
                "Weapon & Spell Damage",
                BonusTarget::WeaponAndSpellDamageFlat,
                258.0,
            ),
        )
        .with_skill_id(45312),
    ]
});

