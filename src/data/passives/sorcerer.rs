use crate::data::bonuses::MINOR_PROPHECY;
use crate::data::{BonusTarget, BonusTrigger, ClassName, SkillLineName};
use crate::domain::{BonusData, PassiveData};
use once_cell::sync::Lazy;

pub static SORCERER_PASSIVES: Lazy<Vec<PassiveData>> = Lazy::new(|| {
    vec![
        // === DARK MAGIC ===
        PassiveData::new(
            "Unholy Knowledge",
            ClassName::Sorcerer,
            SkillLineName::DarkMagic,
            vec![], // Reduces cost
        ),
        PassiveData::new(
            "Blood Magic",
            ClassName::Sorcerer,
            SkillLineName::DarkMagic,
            vec![], // TODO: To complex, increase max resources
        ),
        PassiveData::new(
            "Persistence",
            ClassName::Sorcerer,
            SkillLineName::DarkMagic,
            vec![], // Cost reduction
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
            vec![], // TODO: to complex, restore resources on ultimate end
        ),
        PassiveData::new(
            "Power Stone",
            ClassName::Sorcerer,
            SkillLineName::DaedricSummoning,
            vec![], // Reduces ultimate cost
        ),
        PassiveData::new(
            "Daedric Protection",
            ClassName::Sorcerer,
            SkillLineName::DaedricSummoning,
            vec![], // Reduces damage taken 5%
        ),
        PassiveData::new(
            "Expert Summoner",
            ClassName::Sorcerer,
            SkillLineName::DaedricSummoning,
            vec![
                BonusData::new(
                    "Expert Summoner 1",
                    BonusTrigger::Passive,
                    BonusTarget::MaxMagicka,
                    0.05,
                ),
                BonusData::new(
                    "Expert Summoner 2",
                    BonusTrigger::Passive,
                    BonusTarget::MaxStamina,
                    0.05,
                ),
            ],
        ),
        // === STORM CALLING ===
        PassiveData::new(
            "Capacitor",
            ClassName::Sorcerer,
            SkillLineName::StormCalling,
            vec![], // Increase resource recovery
        ),
        PassiveData::new(
            "Energized",
            ClassName::Sorcerer,
            SkillLineName::StormCalling,
            vec![
                BonusData::new(
                    "Energized 1",
                    BonusTrigger::Passive,
                    BonusTarget::PhysicalDamage,
                    0.05,
                ),
                BonusData::new(
                    "Energized 1",
                    BonusTrigger::Passive,
                    BonusTarget::ShockDamage,
                    0.05,
                ),
            ],
        ),
        PassiveData::new(
            "Amplitude",
            ClassName::Sorcerer,
            SkillLineName::StormCalling,
            vec![], // TODO: To complex, increase damage base on current health
        ),
        PassiveData::new(
            "Expert Mage",
            ClassName::Sorcerer,
            SkillLineName::StormCalling,
            vec![BonusData::new(
                "Expert Mage",
                BonusTrigger::AbilitySlottedCount,
                BonusTarget::WeaponAndSpellDamage,
                108.0,
            )],
        ),
    ]
});
