use crate::data::bonuses::MINOR_SAVAGERY;
use crate::data::{BonusTarget, BonusType, ClassName, SkillLineName};
use crate::domain::{BonusData, PassiveData};
use once_cell::sync::Lazy;

pub static NIGHTBLADE_PASSIVES: Lazy<Vec<PassiveData>> = Lazy::new(|| {
    vec![
        // === ASSASSINATION ===
        PassiveData::new(
            "Master Assassin",
            ClassName::Nightblade,
            SkillLineName::Assassination,
            vec![BonusData::new(
                "Master Assassin",
                BonusType::SkillLine,
                BonusTarget::CriticalChance,
                0.066,
            )],
        ),
        PassiveData::new(
            "Executioner",
            ClassName::Nightblade,
            SkillLineName::Assassination,
            vec![],
        ),
        PassiveData::new(
            "Pressure Point",
            ClassName::Nightblade,
            SkillLineName::Assassination,
            vec![BonusData::new(
                "Pressure Point",
                BonusType::AbilitySlottedCount,
                BonusTarget::CriticalChance,
                0.025,
            )],
        ),
        PassiveData::new(
            "Hemorrhage",
            ClassName::Nightblade,
            SkillLineName::Assassination,
            vec![
                BonusData::new(
                    "Hemorrhage",
                    BonusType::AbilitySlotted,
                    BonusTarget::CriticalDamage,
                    0.1,
                ),
                MINOR_SAVAGERY.clone(),
            ],
        ),
        // === SHADOW ===
        PassiveData::new(
            "Refreshing Shadows",
            ClassName::Nightblade,
            SkillLineName::Shadow,
            vec![],
        ),
        PassiveData::new(
            "Shadow Barrier",
            ClassName::Nightblade,
            SkillLineName::Shadow,
            vec![],
        ),
        PassiveData::new(
            "Dark Vigor",
            ClassName::Nightblade,
            SkillLineName::Shadow,
            vec![],
        ),
        PassiveData::new(
            "Dark Veil",
            ClassName::Nightblade,
            SkillLineName::Shadow,
            vec![BonusData::new(
                "Dark Veil",
                BonusType::SkillLine,
                BonusTarget::Duration,
                2.0,
            )],
        ),
        // === SIPHONING ===
        PassiveData::new(
            "Catalyst",
            ClassName::Nightblade,
            SkillLineName::Siphoning,
            vec![],
        ),
        PassiveData::new(
            "Magicka Flood",
            ClassName::Nightblade,
            SkillLineName::Siphoning,
            vec![BonusData::new(
                "Magicka Flood",
                BonusType::SkillLine,
                BonusTarget::MaxStamina,
                0.06,
            )],
        ),
        PassiveData::new(
            "Soul Siphoner",
            ClassName::Nightblade,
            SkillLineName::Siphoning,
            vec![],
        ),
        PassiveData::new(
            "Transfer",
            ClassName::Nightblade,
            SkillLineName::Siphoning,
            vec![],
        ),
    ]
});
