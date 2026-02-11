use crate::data::bonuses::MINOR_SAVAGERY;
use crate::domain::{BonusData, BonusSource, PassiveData};
use crate::domain::{BonusTarget, BonusTrigger, ClassName, SkillLineName};
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
                BonusSource::Passive,
                BonusTrigger::Flanking,
                BonusTarget::CriticalRating,
                1448.0,
            )],
        ),
        PassiveData::new(
            "Executioner",
            ClassName::Nightblade,
            SkillLineName::Assassination,
            vec![], // TODO: Restore 1000 Magicka and Stamina when enemy dies within 2s of being damaged
        ),
        PassiveData::new(
            "Pressure Points",
            ClassName::Nightblade,
            SkillLineName::Assassination,
            vec![BonusData::new(
                "Pressure Points",
                BonusSource::Passive,
                BonusTrigger::AbilitySlottedCount,
                BonusTarget::CriticalRating,
                548.0,
            )],
        ),
        PassiveData::new(
            "Hemorrhage",
            ClassName::Nightblade,
            SkillLineName::Assassination,
            vec![
                BonusData::new(
                    "Hemorrhage",
                    BonusSource::Passive,
                    BonusTrigger::SkillLineSlotted,
                    BonusTarget::CriticalDamage,
                    0.1,
                ),
                MINOR_SAVAGERY // TODO: with condition skill line ability slotted
                    .clone()
                    .with_trigger(BonusTrigger::CriticalDamageDealt),
            ],
        ),
        // === SHADOW ===
        PassiveData::new(
            "Refreshing Shadows",
            ClassName::Nightblade,
            SkillLineName::Shadow,
            vec![], // Increase resource recovery 15%
        ),
        PassiveData::new(
            "Shadow Barrier",
            ClassName::Nightblade,
            SkillLineName::Shadow,
            vec![], // Mayor resolve 12 secs
        ),
        PassiveData::new(
            "Dark Vigor",
            ClassName::Nightblade,
            SkillLineName::Shadow,
            vec![], // Increase max health
        ),
        PassiveData::new(
            "Dark Veil",
            ClassName::Nightblade,
            SkillLineName::Shadow,
            vec![BonusData::new(
                "Dark Veil",
                BonusSource::Passive,
                BonusTrigger::Passive,
                BonusTarget::DurationSkillLineFlat,
                2.0,
            )],
        ),
        // === SIPHONING ===
        PassiveData::new(
            "Catalyst",
            ClassName::Nightblade,
            SkillLineName::Siphoning,
            vec![], // TODO: Ultimate gain on potion
        ),
        PassiveData::new(
            "Magicka Flood",
            ClassName::Nightblade,
            SkillLineName::Siphoning,
            vec![
                BonusData::new(
                    "Magicka Flood (Stamina)",
                    BonusSource::Passive,
                    BonusTrigger::SkillLineSlotted,
                    BonusTarget::MaxStamina,
                    0.06,
                ),
                BonusData::new(
                    "Magicka Flood (Magicka)",
                    BonusSource::Passive,
                    BonusTrigger::SkillLineSlotted,
                    BonusTarget::MaxMagicka,
                    0.06,
                ),
            ],
        ),
        PassiveData::new(
            "Soul Siphoner",
            ClassName::Nightblade,
            SkillLineName::Siphoning,
            vec![], // Increase healing
        ),
        PassiveData::new(
            "Transfer",
            ClassName::Nightblade,
            SkillLineName::Siphoning,
            vec![], //  TODO: Generate 2 ultimate
        ),
    ]
});
