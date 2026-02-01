use crate::data::{BonusTarget, BonusType, ClassName, SkillLineName};
use crate::domain::{BonusData, PassiveData};
use once_cell::sync::Lazy;

pub static WEAPON_PASSIVES: Lazy<Vec<PassiveData>> = Lazy::new(|| {
    vec![
        // === BOW ===
        PassiveData::new("Long Shots", ClassName::Weapon, SkillLineName::Bow, vec![]),
        PassiveData::new(
            "Accuracy",
            ClassName::Weapon,
            SkillLineName::Bow,
            vec![BonusData::new(
                "Accuracy",
                BonusType::SkillLine,
                BonusTarget::CriticalChance,
                0.08,
            )],
        ),
        PassiveData::new("Ranger", ClassName::Weapon, SkillLineName::Bow, vec![]),
        PassiveData::new(
            "Hawk Eye",
            ClassName::Weapon,
            SkillLineName::Bow,
            vec![BonusData::new(
                "Hawk Eye",
                BonusType::AbilitySlottedCount,
                BonusTarget::CriticalDamage,
                0.02,
            )],
        ),
        // === DUAL WIELD ===
        PassiveData::new(
            "Slaughter",
            ClassName::Weapon,
            SkillLineName::DualWield,
            vec![],
        ),
        PassiveData::new(
            "Dual Wield Expert",
            ClassName::Weapon,
            SkillLineName::DualWield,
            vec![],
        ),
        PassiveData::new(
            "Controlled Fury",
            ClassName::Weapon,
            SkillLineName::DualWield,
            vec![],
        ),
        PassiveData::new(
            "Twin Blade and Blunt",
            ClassName::Weapon,
            SkillLineName::DualWield,
            vec![BonusData::new(
                "Twin Blade and Blunt",
                BonusType::SkillLine,
                BonusTarget::CriticalDamage,
                0.05,
            )],
        ),
        // === TWO HANDED ===
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
                BonusType::SkillLine,
                BonusTarget::CriticalDamage,
                0.12,
            )],
        ),
        PassiveData::new(
            "Balanced Blade",
            ClassName::Weapon,
            SkillLineName::TwoHanded,
            vec![],
        ),
        PassiveData::new(
            "Follow Up",
            ClassName::Weapon,
            SkillLineName::TwoHanded,
            vec![],
        ),
        // === DESTRUCTION STAFF ===
        PassiveData::new(
            "Tri Focus",
            ClassName::Weapon,
            SkillLineName::DestructionStaff,
            vec![],
        ),
        PassiveData::new(
            "Penetrating Magic",
            ClassName::Weapon,
            SkillLineName::DestructionStaff,
            vec![],
        ),
        PassiveData::new(
            "Elemental Force",
            ClassName::Weapon,
            SkillLineName::DestructionStaff,
            vec![],
        ),
        PassiveData::new(
            "Ancient Knowledge",
            ClassName::Weapon,
            SkillLineName::DestructionStaff,
            vec![BonusData::new(
                "Ancient Knowledge",
                BonusType::AbilitySlottedCount,
                BonusTarget::CriticalChance,
                0.04,
            )],
        ),
    ]
});
