use crate::domain::{
    BonusData, BonusSource, BonusTarget, BonusTrigger, BonusValue, ClassName, PassiveData,
    SkillLineName,
};
use once_cell::sync::Lazy;

pub static TWO_HANDED_PASSIVES: Lazy<Vec<PassiveData>> = Lazy::new(|| {
    vec![
        PassiveData::new(
            "Forceful",
            ClassName::Weapon,
            SkillLineName::TwoHanded,
            vec![],
        ),
        PassiveData::new(
            "Heavy Weapons",
            ClassName::Weapon,
            SkillLineName::TwoHanded,
            vec![BonusData::new(
                "Heavy Weapons",
                BonusSource::Passive,
                BonusTrigger::TwoHandedEquipped,
                BonusValue::new(
                    "Heavy Weapons (Sword)",
                    BonusTarget::WeaponAndSpellDamageFlat,
                    258.0,
                ),
            )
            .with_alternative(BonusValue::new(
                "Heavy Weapons (Axe)",
                BonusTarget::CriticalDamage,
                0.12,
            ))
            .with_alternative(BonusValue::new(
                "Heavy Weapons (Mace)",
                BonusTarget::PhysicalAndSpellPenetration,
                2974.0,
            ))],
        ),
        PassiveData::new(
            "Balanced Blade",
            ClassName::Weapon,
            SkillLineName::TwoHanded,
            vec![],
        ),
        // TODO: Requires Heavy Attack tracking to implement
        PassiveData::new(
            "Follow Up",
            ClassName::Weapon,
            SkillLineName::TwoHanded,
            vec![],
        ),
        PassiveData::new(
            "Battle Rush",
            ClassName::Weapon,
            SkillLineName::TwoHanded,
            vec![],
        ),
    ]
});

