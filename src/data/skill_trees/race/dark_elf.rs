
use crate::domain::{BonusData, BonusSource, BonusTarget, BonusTrigger, BonusValue};
use once_cell::sync::Lazy;

pub static DARK_ELF_BONUSES: Lazy<Vec<BonusData>> = Lazy::new(|| {
    vec![
        BonusData::new(
            "Dynamic (Magicka)",
            BonusSource::Passive,
            BonusTrigger::Passive,
            BonusValue::new("Max Magicka", BonusTarget::MaxMagickaFlat, 1910.0),
        )
        .with_skill_id(45267),
        BonusData::new(
            "Dynamic (Stamina)",
            BonusSource::Passive,
            BonusTrigger::Passive,
            BonusValue::new("Max Stamina", BonusTarget::MaxStaminaFlat, 1910.0),
        )
        .with_skill_id(45267),
        BonusData::new(
            "Ruination",
            BonusSource::Passive,
            BonusTrigger::Passive,
            BonusValue::new(
                "Weapon & Spell Damage",
                BonusTarget::WeaponAndSpellDamageFlat,
                258.0,
            ),
        )
        .with_skill_id(45272),
    ]
});

