use crate::data::bonuses::MINOR_PROPHECY;
use crate::domain::{BonusData, BonusSource, BonusValue, PassiveData};
use crate::domain::{BonusTarget, BonusTrigger, ClassName, SkillLineName};
use once_cell::sync::Lazy;

pub static SORCERER_PASSIVES: Lazy<Vec<PassiveData>> = Lazy::new(|| {
    vec![
        PassiveData::new(
            "Unholy Knowledge",
            ClassName::Sorcerer,
            SkillLineName::DarkMagic,
            vec![],
        )
            .with_skill_id(45176),
        PassiveData::new(
            "Blood Magic",
            ClassName::Sorcerer,
            SkillLineName::DarkMagic,
            vec![], // TODO: To complex, increase max resources with conditions
        )
            .with_skill_id(45172),
        PassiveData::new(
            "Persistence",
            ClassName::Sorcerer,
            SkillLineName::DarkMagic,
            vec![],
        )
            .with_skill_id(45165),
        PassiveData::new(
            "Exploitation",
            ClassName::Sorcerer,
            SkillLineName::DarkMagic,
            vec![MINOR_PROPHECY
                .clone()
                .with_trigger(BonusTrigger::SkillLineSkillCast)],
        )
            .with_skill_id(45181),
        PassiveData::new(
            "Rebate",
            ClassName::Sorcerer,
            SkillLineName::DaedricSummoning,
            vec![], // TODO: to complex, restore resources on non ultimate end
        )
            .with_skill_id(45198),
        PassiveData::new(
            "Power Stone",
            ClassName::Sorcerer,
            SkillLineName::DaedricSummoning,
            vec![],
        )
            .with_skill_id(45196),
        PassiveData::new(
            "Daedric Protection",
            ClassName::Sorcerer,
            SkillLineName::DaedricSummoning,
            vec![],
        )
            .with_skill_id(45200),
        PassiveData::new(
            "Expert Summoner",
            ClassName::Sorcerer,
            SkillLineName::DaedricSummoning,
            vec![
                BonusData::new(
                    "Expert Summoner 1",
                    BonusSource::Passive,
                    BonusTrigger::Passive,
                    BonusValue::new("Expert Summoner (Magicka)", BonusTarget::MaxMagicka, 0.05),
                ),
                BonusData::new(
                    "Expert Summoner 2",
                    BonusSource::Passive,
                    BonusTrigger::Passive,
                    BonusValue::new("Expert Summoner (Stamina)", BonusTarget::MaxStamina, 0.05),
                ),
            ],
        )
            .with_skill_id(45199),
        PassiveData::new(
            "Capacitor",
            ClassName::Sorcerer,
            SkillLineName::StormCalling,
            vec![],
        )
            .with_skill_id(45188),
        PassiveData::new(
            "Energized",
            ClassName::Sorcerer,
            SkillLineName::StormCalling,
            vec![
                BonusData::new(
                    "Energized (Physical)",
                    BonusSource::Passive,
                    BonusTrigger::Passive,
                    BonusValue::new("Energized (Physical)", BonusTarget::PhysicalDamage, 0.05),
                ),
                BonusData::new(
                    "Energized (Shock)",
                    BonusSource::Passive,
                    BonusTrigger::Passive,
                    BonusValue::new("Energized (Shock)", BonusTarget::ShockDamage, 0.05),
                ),
            ],
        )
            .with_skill_id(45190),
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
                BonusSource::Passive,
                BonusTrigger::AbilitySlottedCount,
                BonusValue::new("Expert Mage", BonusTarget::WeaponAndSpellDamageFlat, 108.0),
            )
            .with_skill_line_filter(SkillLineName::StormCalling)],
        )
            .with_skill_id(45195),
    ]
});

