use crate::data::bonuses::MINOR_SAVAGERY;
use crate::data::{BonusTarget, BonusTrigger, ClassName, SkillLineName};
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
                BonusTrigger::Passive,
                BonusTarget::CriticalChance,
                1448.0,
            )],
        ),
        PassiveData::new(
            "Executioner",
            ClassName::Nightblade,
            SkillLineName::Assassination,
            vec![], // On dead effect, restore magicka and stamina
        ),
        PassiveData::new(
            "Pressure Point",
            ClassName::Nightblade,
            SkillLineName::Assassination,
            vec![BonusData::new(
                "Pressure Point",
                BonusTrigger::AbilitySlottedCount,
                BonusTarget::CriticalChance,
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
                    BonusTrigger::AbilitySlotted,
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
            vec![], // Ultimate gain
        ),
        PassiveData::new(
            "Magicka Flood",
            ClassName::Nightblade,
            SkillLineName::Siphoning,
            vec![
                BonusData::new(
                    "Magicka Flood",
                    BonusTrigger::SkillLineSlotted,
                    BonusTarget::MaxStamina,
                    0.06,
                ),
                BonusData::new(
                    "Magicka Flood",
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
            vec![], // Ultimate gen
        ),
    ]
});
