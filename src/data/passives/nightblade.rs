use crate::data::{BonusClassName, BonusType, ClassName, SkillLineName};
use crate::domain::{BonusData, PassiveData};
use once_cell::sync::Lazy;

pub static NIGHTBLADE_PASSIVES: Lazy<Vec<PassiveData>> = Lazy::new(|| {
    vec![
        // === ASSASSINATION ===
        PassiveData::new("Master Assassin", ClassName::Nightblade, SkillLineName::Assassination, vec![
            BonusData::new(BonusClassName::SkillLine, BonusType::CriticalChance, 0.066),
        ]),
        PassiveData::new("Executioner", ClassName::Nightblade, SkillLineName::Assassination, vec![]),
        PassiveData::new("Pressure Point", ClassName::Nightblade, SkillLineName::Assassination, vec![
            BonusData::new(BonusClassName::AbilitySlottedCount, BonusType::CriticalChance, 0.025),
        ]),
        PassiveData::new("Hemorrhage", ClassName::Nightblade, SkillLineName::Assassination, vec![
            BonusData::new(BonusClassName::AbilitySlotted, BonusType::CriticalDamage, 0.1),
            BonusData::new(BonusClassName::Duration, BonusType::WeaponCriticalChance, 1314.0)
                .with_id("Minor Savagery"),
        ]),

        // === SHADOW ===
        PassiveData::new("Refreshing Shadows", ClassName::Nightblade, SkillLineName::Shadow, vec![]),
        PassiveData::new("Shadow Barrier", ClassName::Nightblade, SkillLineName::Shadow, vec![]),
        PassiveData::new("Dark Vigor", ClassName::Nightblade, SkillLineName::Shadow, vec![]),
        PassiveData::new("Dark Veil", ClassName::Nightblade, SkillLineName::Shadow, vec![
            BonusData::new(BonusClassName::SkillLine, BonusType::Duration, 2.0),
        ]),

        // === SIPHONING ===
        PassiveData::new("Catalyst", ClassName::Nightblade, SkillLineName::Siphoning, vec![]),
        PassiveData::new("Magicka Flood", ClassName::Nightblade, SkillLineName::Siphoning, vec![
            BonusData::new(BonusClassName::SkillLine, BonusType::MaxStamina, 0.06),
        ]),
        PassiveData::new("Soul Siphoner", ClassName::Nightblade, SkillLineName::Siphoning, vec![]),
        PassiveData::new("Transfer", ClassName::Nightblade, SkillLineName::Siphoning, vec![]),
    ]
});
