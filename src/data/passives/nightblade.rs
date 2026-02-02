use crate::data::bonuses::MINOR_SAVAGERY;
use crate::data::{BonusTarget, BonusTrigger, ClassName, SkillLineName};
use crate::domain::{BonusData, PassiveData};
use once_cell::sync::Lazy;

pub static NIGHTBLADE_PASSIVES: Lazy<Vec<PassiveData>> = Lazy::new(|| {
    vec![
        // === ASSASSINATION ===
        // Master Assassin: +1448 Crit Chance (6.6%) when flanking enemies
        PassiveData::new(
            "Master Assassin",
            ClassName::Nightblade,
            SkillLineName::Assassination,
            vec![BonusData::new(
                "Master Assassin",
                BonusTrigger::Flanking,
                BonusTarget::CriticalChance,
                1448.0,
            )],
        ),
        // Executioner: Restore 1000 Magicka and Stamina when enemy dies within 2s of being damaged
        PassiveData::new(
            "Executioner",
            ClassName::Nightblade,
            SkillLineName::Assassination,
            vec![], // Resource restore on kill effect - not tracked in damage calculations
        ),
        // Pressure Points: +548 Crit Chance (2.5%) per Assassination ability slotted
        PassiveData::new(
            "Pressure Points",
            ClassName::Nightblade,
            SkillLineName::Assassination,
            vec![BonusData::new(
                "Pressure Points",
                BonusTrigger::AbilitySlottedCount,
                BonusTarget::CriticalChance,
                548.0,
            )],
        ),
        // Hemorrhage: +10% Crit Damage always, Minor Savagery to group on dealing crit damage
        PassiveData::new(
            "Hemorrhage",
            ClassName::Nightblade,
            SkillLineName::Assassination,
            vec![
                BonusData::new(
                    "Hemorrhage",
                    BonusTrigger::Passive,
                    BonusTarget::CriticalDamage,
                    0.1,
                ),
                MINOR_SAVAGERY.clone().with_trigger(BonusTrigger::CriticalDamageDealt),
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
