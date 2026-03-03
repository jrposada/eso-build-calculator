
use crate::domain::{BonusData, BonusSource, BonusTarget, BonusTrigger, BonusValue};
use once_cell::sync::Lazy;

pub static HIGH_ELF_BONUSES: Lazy<Vec<BonusData>> = Lazy::new(|| {
    vec![
        BonusData::new(
            "Elemental Talent",
            BonusSource::Passive,
            BonusTrigger::Passive,
            BonusValue::new(
                "Weapon & Spell Damage",
                BonusTarget::WeaponAndSpellDamageFlat,
                258.0,
            ),
        )
        .with_skill_id(45276),
        BonusData::new(
            "Syrabane's Boon",
            BonusSource::Passive,
            BonusTrigger::Passive,
            BonusValue::new("Max Magicka", BonusTarget::MaxMagickaFlat, 2000.0),
        )
        .with_skill_id(117970),
    ]
});

