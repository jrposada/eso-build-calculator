use crate::domain::{BonusData, BonusSource, BonusValue, PassiveData};
use crate::domain::{BonusTarget, BonusTrigger, ClassName, SkillLineName};
use once_cell::sync::Lazy;

pub static WARDEN_PASSIVES: Lazy<Vec<PassiveData>> = Lazy::new(|| {
    vec![
        PassiveData::new(
            "Bond With Nature",
            ClassName::Warden,
            SkillLineName::AnimalCompanions,
            vec![],
        )
            .with_skill_id(86065),
        PassiveData::new(
            "Savage Beast",
            ClassName::Warden,
            SkillLineName::AnimalCompanions,
            vec![], // TODO: 4 ultimate generation
        )
            .with_skill_id(86063),
        PassiveData::new(
            "Flourish",
            ClassName::Warden,
            SkillLineName::AnimalCompanions,
            vec![],
        )
            .with_skill_id(86067),
        PassiveData::new(
            "Advanced Species",
            ClassName::Warden,
            SkillLineName::AnimalCompanions,
            vec![BonusData::new(
                "Advanced Species",
                BonusSource::Passive,
                BonusTrigger::AbilitySlottedCount,
                BonusValue::new("Advanced Species", BonusTarget::CriticalDamage, 0.05),
            )
            .with_skill_line_filter(SkillLineName::AnimalCompanions)],
        )
            .with_skill_id(86069),
        PassiveData::new(
            "Accelerated Growth",
            ClassName::Warden,
            SkillLineName::GreenBalance,
            vec![],
        )
            .with_skill_id(85883),
        PassiveData::new(
            "Nature's Gift",
            ClassName::Warden,
            SkillLineName::GreenBalance,
            vec![],
        )
            .with_skill_id(85879),
        PassiveData::new(
            "Emerald Moss",
            ClassName::Warden,
            SkillLineName::GreenBalance,
            vec![],
        )
            .with_skill_id(85877),
        PassiveData::new(
            "Maturation",
            ClassName::Warden,
            SkillLineName::GreenBalance,
            vec![],
        )
            .with_skill_id(85881),
        PassiveData::new(
            "Glacial Presence",
            ClassName::Warden,
            SkillLineName::WintersEmbrace,
            vec![
                BonusData::new(
                    "Glacial Presence 1",
                    BonusSource::Passive,
                    BonusTrigger::Passive,
                    BonusValue::new(
                        "Glacial Presence (Chance)",
                        BonusTarget::ChilledStatusEffectChance,
                        2.5,
                    ),
                ),
                BonusData::new(
                    "Glacial Presence 2",
                    BonusSource::Passive,
                    BonusTrigger::Passive,
                    BonusValue::new(
                        "Glacial Presence (Damage)",
                        BonusTarget::ChilledStatusEffectDamage,
                        105.0, // TODO: scales of WeaponOrSpellDamage
                    ),
                ),
            ],
        )
            .with_skill_id(86192),
        PassiveData::new(
            "Frozen Armor",
            ClassName::Warden,
            SkillLineName::WintersEmbrace,
            vec![],
        )
            .with_skill_id(86190),
        PassiveData::new(
            "Icy Aura",
            ClassName::Warden,
            SkillLineName::WintersEmbrace,
            vec![],
        )
            .with_skill_id(86194),
        PassiveData::new(
            "Piercing Cold",
            ClassName::Warden,
            SkillLineName::WintersEmbrace,
            vec![
                BonusData::new(
                    "Piercing Cold",
                    BonusSource::Passive,
                    BonusTrigger::SkillLineSlotted,
                    BonusValue::new("Piercing Cold", BonusTarget::FrostDamage, 0.15),
                ),
            ],
        )
            .with_skill_id(86196),
    ]
});

