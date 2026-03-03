use crate::domain::{
    BonusData, BonusSource, BonusTarget, BonusTrigger, BonusValue, ClassName, PassiveData,
    SkillLineName,
};
use once_cell::sync::Lazy;

pub static BOW_PASSIVES: Lazy<Vec<PassiveData>> = Lazy::new(|| {
    vec![
        PassiveData::new(
            "Long Shots",
            ClassName::Weapon,
            SkillLineName::Bow,
            vec![BonusData::new(
                "Long Shots",
                BonusSource::Passive,
                BonusTrigger::BowEquipped,
                BonusValue::new("Long Shots (Damage)", BonusTarget::Damage, 0.05),
            )
            .with_alternative(BonusValue::new(
                "Long Shots (Crit Rating)",
                BonusTarget::CriticalRating,
                1314.0,
            ))],
        ),
        PassiveData::new(
            "Accuracy",
            ClassName::Weapon,
            SkillLineName::Bow,
            vec![BonusData::new(
                "Accuracy",
                BonusSource::Passive,
                BonusTrigger::BowEquipped,
                BonusValue::new("Accuracy", BonusTarget::CriticalRating, 1314.0),
            )],
        ),
        PassiveData::new(
            "Ranger",
            ClassName::Weapon,
            SkillLineName::Bow,
            vec![],
        ),
        PassiveData::new(
            "Hawk Eye",
            ClassName::Weapon,
            SkillLineName::Bow,
            vec![], // TODO: To complex, Stacks per basic of increase damage
        ),
        PassiveData::new(
            "Hasty Retreat",
            ClassName::Weapon,
            SkillLineName::Bow,
            vec![],
        ),
    ]
});
