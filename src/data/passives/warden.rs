use crate::domain::{BonusData, BonusSource, BonusValue, PassiveData};
use crate::domain::{BonusTarget, BonusTrigger, ClassName, SkillLineName};
use once_cell::sync::Lazy;

pub static WARDEN_PASSIVES: Lazy<Vec<PassiveData>> = Lazy::new(|| {
    vec![
        // === ANIMAL COMPANIONS ===
        PassiveData::new(
            "Bond With Nature",
            ClassName::Warden,
            SkillLineName::AnimalCompanions,
            vec![], // Heal
        ),
        PassiveData::new(
            "Savage Beast",
            ClassName::Warden,
            SkillLineName::AnimalCompanions,
            vec![], // TODO: 4 ultimate generation
        ),
        PassiveData::new(
            "Flourish",
            ClassName::Warden,
            SkillLineName::AnimalCompanions,
            vec![], // Resource recovery rate
        ),
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
        ),
        // === GREEN BALANCE ===
        PassiveData::new(
            "Accelerated Growth",
            ClassName::Warden,
            SkillLineName::GreenBalance,
            vec![], // Minor Mending
        ),
        PassiveData::new(
            "Nature's Gift",
            ClassName::Warden,
            SkillLineName::GreenBalance,
            vec![], // On heal trigger -> restore resource
        ),
        PassiveData::new(
            "Emerald Moss",
            ClassName::Warden,
            SkillLineName::GreenBalance,
            vec![], // Increase healing done *5%
        ),
        PassiveData::new(
            "Maturation",
            ClassName::Warden,
            SkillLineName::GreenBalance,
            vec![], // Minor Toughness on heal
        ),
        // === WINTER'S EMBRACE ===
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
        ),
        PassiveData::new(
            "Frozen Armor",
            ClassName::Warden,
            SkillLineName::WintersEmbrace,
            vec![], // Increase resistance
        ),
        PassiveData::new(
            "Icy Aura",
            ClassName::Warden,
            SkillLineName::WintersEmbrace,
            vec![], // Major Maim
        ),
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
                // Increases the amount of damage you block by 8% (not tracked - defensive)
            ],
        ),
    ]
});
