use crate::data::{BonusTarget, BonusType, ClassName, SkillLineName};
use crate::domain::{BonusData, PassiveData};
use once_cell::sync::Lazy;

pub static TEMPLAR_PASSIVES: Lazy<Vec<PassiveData>> = Lazy::new(|| {
    vec![
        // === AEDRIC SPEAR ===
        PassiveData::new(
            "Piercing Spear",
            ClassName::Templar,
            SkillLineName::AedricSpear,
            vec![BonusData::new(
                "Piercing Spear",
                BonusType::SkillLine,
                BonusTarget::CriticalDamage,
                0.1,
            )],
        ),
        PassiveData::new(
            "Spear Wall",
            ClassName::Templar,
            SkillLineName::AedricSpear,
            vec![],
        ),
        PassiveData::new(
            "Burning Light",
            ClassName::Templar,
            SkillLineName::AedricSpear,
            vec![],
        ),
        PassiveData::new(
            "Balanced Warrior",
            ClassName::Templar,
            SkillLineName::AedricSpear,
            vec![],
        ),
        // === DAWN'S WRATH ===
        PassiveData::new(
            "Illuminate",
            ClassName::Templar,
            SkillLineName::DawnsWrath,
            vec![],
        ),
        PassiveData::new(
            "Restoring Spirit",
            ClassName::Templar,
            SkillLineName::DawnsWrath,
            vec![],
        ),
        PassiveData::new(
            "Enduring Rays",
            ClassName::Templar,
            SkillLineName::DawnsWrath,
            vec![BonusData::new(
                "Enduring Rays",
                BonusType::SkillLine,
                BonusTarget::Duration,
                3.0,
            )],
        ),
        PassiveData::new(
            "Prism",
            ClassName::Templar,
            SkillLineName::DawnsWrath,
            vec![],
        ),
        // === RESTORING LIGHT ===
        PassiveData::new(
            "Mending",
            ClassName::Templar,
            SkillLineName::RestoringLight,
            vec![],
        ),
        PassiveData::new(
            "Sacred Ground",
            ClassName::Templar,
            SkillLineName::RestoringLight,
            vec![],
        ),
        PassiveData::new(
            "Light Weaver",
            ClassName::Templar,
            SkillLineName::RestoringLight,
            vec![],
        ),
        PassiveData::new(
            "Master Ritualist",
            ClassName::Templar,
            SkillLineName::RestoringLight,
            vec![],
        ),
    ]
});
