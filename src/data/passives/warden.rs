use crate::domain::{BonusTarget, BonusTrigger, ClassName, SkillLineName};
use crate::domain::{BonusData, PassiveData};
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
                BonusTrigger::AbilitySlottedCount,
                BonusTarget::CriticalDamage,
                0.05,
            )],
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
                    BonusTrigger::Passive,
                    BonusTarget::ChilledStatusEffectChance,
                    2.5,
                ),
                BonusData::new(
                    "Glacial Presence 2",
                    BonusTrigger::Passive,
                    BonusTarget::ChilledStatusEffectDamage,
                    105.0, // TODO: scales of WeaponOrSpellDamage
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
                    BonusTrigger::SkillLineSlotted,
                    BonusTarget::FrostDamage,
                    0.15,
                ),
                // Increases the amount of damage you block by 8% (not tracked - defensive)
            ],
        ),
    ]
});
