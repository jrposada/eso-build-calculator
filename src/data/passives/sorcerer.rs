use crate::data::bonuses::MINOR_PROPHECY;
use crate::data::{BonusTarget, BonusType, ClassName, SkillLineName};
use crate::domain::{BonusData, PassiveData};
use once_cell::sync::Lazy;

pub static SORCERER_PASSIVES: Lazy<Vec<PassiveData>> = Lazy::new(|| {
    vec![
        // === DARK MAGIC ===
        PassiveData::new(
            "Unholy Knowledge",
            ClassName::Sorcerer,
            SkillLineName::DarkMagic,
            vec![],
        ),
        PassiveData::new(
            "Blood Magic",
            ClassName::Sorcerer,
            SkillLineName::DarkMagic,
            vec![],
        ),
        PassiveData::new(
            "Persistence",
            ClassName::Sorcerer,
            SkillLineName::DarkMagic,
            vec![],
        ),
        PassiveData::new(
            "Exploitation",
            ClassName::Sorcerer,
            SkillLineName::DarkMagic,
            vec![MINOR_PROPHECY.clone()],
        ),
        // === DAEDRIC SUMMONING ===
        PassiveData::new(
            "Rebate",
            ClassName::Sorcerer,
            SkillLineName::DaedricSummoning,
            vec![],
        ),
        PassiveData::new(
            "Power Stone",
            ClassName::Sorcerer,
            SkillLineName::DaedricSummoning,
            vec![],
        ),
        PassiveData::new(
            "Daedric Protection",
            ClassName::Sorcerer,
            SkillLineName::DaedricSummoning,
            vec![],
        ),
        PassiveData::new(
            "Expert Summoner",
            ClassName::Sorcerer,
            SkillLineName::DaedricSummoning,
            vec![BonusData::new(
                "Expert Summoner",
                BonusType::SkillLine,
                BonusTarget::MaxMagicka,
                0.08,
            )],
        ),
        // === STORM CALLING ===
        PassiveData::new(
            "Capacitor",
            ClassName::Sorcerer,
            SkillLineName::StormCalling,
            vec![],
        ),
        PassiveData::new(
            "Energized",
            ClassName::Sorcerer,
            SkillLineName::StormCalling,
            vec![],
        ),
        PassiveData::new(
            "Amplitude",
            ClassName::Sorcerer,
            SkillLineName::StormCalling,
            vec![],
        ),
        PassiveData::new(
            "Expert Mage",
            ClassName::Sorcerer,
            SkillLineName::StormCalling,
            vec![BonusData::new(
                "Expert Mage",
                BonusType::AbilitySlottedCount,
                BonusTarget::SpellDamage,
                0.05,
            )],
        ),
    ]
});
