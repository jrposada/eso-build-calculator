use crate::data::bonuses::{MINOR_BERSERK, MINOR_SORCERY};
use crate::domain::{BonusData, BonusSource, BonusValue, PassiveData};
use crate::domain::{BonusTarget, BonusTrigger, ClassName, SkillLineName};
use once_cell::sync::Lazy;

pub static TEMPLAR_PASSIVES: Lazy<Vec<PassiveData>> = Lazy::new(|| {
    vec![
        // === AEDRIC SPEAR ===
        PassiveData::new(
            "Piercing Spear",
            ClassName::Templar,
            SkillLineName::AedricSpear,
            vec![BonusData::new(
                "Piercing Spear",
                BonusSource::Passive,
                BonusTrigger::AbilitySlotted,
                BonusValue::new("Piercing Spear", BonusTarget::CriticalDamage, 0.12),
            )],
        ),
        PassiveData::new(
            "Spear Wall",
            ClassName::Templar,
            SkillLineName::AedricSpear,
            vec![
                MINOR_BERSERK
                    .clone()
                    .with_trigger(BonusTrigger::SkillLineSkillCast)
                    .with_duration(6.0),
                // Minor Protection
            ],
        ),
        PassiveData::new(
            "Burning Light",
            ClassName::Templar,
            SkillLineName::AedricSpear,
            vec![], // TODO: To complex, stacks + consume for trigger damage
        ),
        PassiveData::new(
            "Balanced Warrior",
            ClassName::Templar,
            SkillLineName::AedricSpear,
            vec![
                BonusData::new(
                    "Balanced Warrior",
                    BonusSource::Passive,
                    BonusTrigger::SkillLineSlotted,
                    BonusValue::new(
                        "Balanced Warrior",
                        BonusTarget::WeaponAndSpellDamageMultiplier,
                        0.06,
                    ),
                ), // Armor increase
            ],
        ),
        // === DAWN'S WRATH ===
        PassiveData::new(
            "Enduring Rays",
            ClassName::Templar,
            SkillLineName::DawnsWrath,
            vec![], // TODO: duration increase to specific skills
        ),
        PassiveData::new(
            "Prism",
            ClassName::Templar,
            SkillLineName::DawnsWrath,
            vec![], // TODO: generates 3 ultimate
        ),
        PassiveData::new(
            "Illuminate",
            ClassName::Templar,
            SkillLineName::DawnsWrath,
            vec![MINOR_SORCERY
                .clone()
                .with_trigger(BonusTrigger::SkillLineSkillCast)],
        ),
        PassiveData::new(
            "Restoring Spirit",
            ClassName::Templar,
            SkillLineName::DawnsWrath,
            vec![], // Cost reduction
        ),
        // === RESTORING LIGHT ===
        PassiveData::new(
            "Mending",
            ClassName::Templar,
            SkillLineName::RestoringLight,
            vec![], // Increase healing done
        ),
        PassiveData::new(
            "Sacred Ground",
            ClassName::Templar,
            SkillLineName::RestoringLight,
            vec![], // Minor mending
        ),
        PassiveData::new(
            "Light Weaver",
            ClassName::Templar,
            SkillLineName::RestoringLight,
            vec![], // TODO: Ult generation and other
        ),
        PassiveData::new(
            "Master Ritualist",
            ClassName::Templar,
            SkillLineName::RestoringLight,
            vec![], // Other
        ),
    ]
});
