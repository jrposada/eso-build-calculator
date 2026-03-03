use crate::data::bonuses::MINOR_SAVAGERY;
use crate::domain::{BonusData, BonusSource, BonusValue, PassiveData};
use crate::domain::{BonusTarget, BonusTrigger, ClassName, SkillLineName};
use once_cell::sync::Lazy;

pub static NIGHTBLADE_PASSIVES: Lazy<Vec<PassiveData>> = Lazy::new(|| {
    vec![
        PassiveData::new(
            "Master Assassin",
            ClassName::Nightblade,
            SkillLineName::Assassination,
            vec![BonusData::new(
                "Master Assassin",
                BonusSource::Passive,
                BonusTrigger::Flanking,
                BonusValue::new("Master Assassin", BonusTarget::CriticalRating, 1448.0),
            )],
        )
            .with_skill_id(45038),
        PassiveData::new(
            "Executioner",
            ClassName::Nightblade,
            SkillLineName::Assassination,
            vec![], // TODO: Restore 1000 Magicka and Stamina when enemy dies within 2s of being damaged
        )
            .with_skill_id(45048),
        PassiveData::new(
            "Pressure Points",
            ClassName::Nightblade,
            SkillLineName::Assassination,
            vec![BonusData::new(
                "Pressure Points",
                BonusSource::Passive,
                BonusTrigger::AbilitySlottedCount,
                BonusValue::new("Pressure Points", BonusTarget::CriticalRating, 548.0),
            )
            .with_skill_line_filter(SkillLineName::Assassination)],
        )
            .with_skill_id(45053),
        PassiveData::new(
            "Hemorrhage",
            ClassName::Nightblade,
            SkillLineName::Assassination,
            vec![
                BonusData::new(
                    "Hemorrhage",
                    BonusSource::Passive,
                    BonusTrigger::SkillLineSlotted,
                    BonusValue::new("Hemorrhage", BonusTarget::CriticalDamage, 0.1),
                ),
                MINOR_SAVAGERY // TODO: with condition skill line ability slotted
                    .clone()
                    .with_trigger(BonusTrigger::CriticalDamageDealt),
            ],
        )
            .with_skill_id(45060),
        PassiveData::new(
            "Refreshing Shadows",
            ClassName::Nightblade,
            SkillLineName::Shadow,
            vec![],
        )
            .with_skill_id(45103),
        PassiveData::new(
            "Shadow Barrier",
            ClassName::Nightblade,
            SkillLineName::Shadow,
            vec![],
        )
            .with_skill_id(45071),
        PassiveData::new(
            "Dark Vigor",
            ClassName::Nightblade,
            SkillLineName::Shadow,
            vec![],
        )
            .with_skill_id(45084),
        PassiveData::new(
            "Dark Veil",
            ClassName::Nightblade,
            SkillLineName::Shadow,
            vec![BonusData::new(
                "Dark Veil",
                BonusSource::Passive,
                BonusTrigger::Passive,
                BonusValue::new("Dark Veil", BonusTarget::DurationSkillLineFlat, 2.0),
            )],
        )
            .with_skill_id(45115),
        PassiveData::new(
            "Catalyst",
            ClassName::Nightblade,
            SkillLineName::Siphoning,
            vec![], // TODO: Ultimate gain on potion
        )
            .with_skill_id(45135),
        PassiveData::new(
            "Magicka Flood",
            ClassName::Nightblade,
            SkillLineName::Siphoning,
            vec![
                BonusData::new(
                    "Magicka Flood (Stamina)",
                    BonusSource::Passive,
                    BonusTrigger::SkillLineSlotted,
                    BonusValue::new("Magicka Flood (Stamina)", BonusTarget::MaxStamina, 0.06),
                ),
                BonusData::new(
                    "Magicka Flood (Magicka)",
                    BonusSource::Passive,
                    BonusTrigger::SkillLineSlotted,
                    BonusValue::new("Magicka Flood (Magicka)", BonusTarget::MaxMagicka, 0.06),
                ),
            ],
        )
            .with_skill_id(45150),
        PassiveData::new(
            "Soul Siphoner",
            ClassName::Nightblade,
            SkillLineName::Siphoning,
            vec![],
        )
            .with_skill_id(45155),
        PassiveData::new(
            "Transfer",
            ClassName::Nightblade,
            SkillLineName::Siphoning,
            vec![], //  TODO: Generate 2 ultimate
        )
            .with_skill_id(45145),
    ]
});

