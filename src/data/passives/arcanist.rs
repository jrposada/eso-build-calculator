use crate::data::{BonusTarget, BonusTrigger, ClassName, SkillLineName};
use crate::domain::{BonusData, PassiveData};
use once_cell::sync::Lazy;

pub static ARCANIST_PASSIVES: Lazy<Vec<PassiveData>> = Lazy::new(|| {
    vec![
        // === HERALD OF THE TOME ===
        PassiveData::new(
            "Fated Fortune",
            ClassName::Arcanist,
            SkillLineName::HeraldOfTheTome,
            vec![BonusData::new(
                "Fated Fortune",
                BonusTrigger::ArcanistCrux,
                BonusTarget::CriticalDamage,
                0.12,
            )
            .with_duration(7.0)],
        ),
        PassiveData::new(
            "Harnessed Quintessence",
            ClassName::Arcanist,
            SkillLineName::HeraldOfTheTome,
            vec![BonusData::new(
                "Harnessed Quintessence W",
                BonusTrigger::MagickaOrStaminaRestored,
                BonusTarget::WeaponAndSpellDamage,
                284.0,
            )
            .with_duration(10.0)],
        ),
        PassiveData::new(
            "Psychic Lesion",
            ClassName::Arcanist,
            SkillLineName::HeraldOfTheTome,
            vec![],
        ),
        PassiveData::new(
            "Splintered Secrets",
            ClassName::Arcanist,
            SkillLineName::HeraldOfTheTome,
            vec![BonusData::new(
                "Splintered Secrets",
                BonusTrigger::AbilitySlottedCount,
                BonusTarget::PhysicalAndSpellPenetration,
                1240.0,
            )],
        ),
        // === SOLDIER OF APOCRYPHA ===
        PassiveData::new(
            "Aegis of the Unseen",
            ClassName::Arcanist,
            SkillLineName::SoldierOfApocrypha,
            vec![], // Increase Armor 3271
        ),
        PassiveData::new(
            "Wellspring of the Abyss",
            ClassName::Arcanist,
            SkillLineName::SoldierOfApocrypha,
            vec![], // Increase resource recovery 81
        ),
        PassiveData::new(
            "Circumvented Fate",
            ClassName::Arcanist,
            SkillLineName::SoldierOfApocrypha,
            vec![], // Minor Evasion
        ),
        PassiveData::new(
            "Implacable Outcome",
            ClassName::Arcanist,
            SkillLineName::SoldierOfApocrypha,
            vec![], // Ultimate generation 4
        ),
        // === CURATIVE RUNEFORMS ===
        PassiveData::new(
            "Healing Tides",
            ClassName::Arcanist,
            SkillLineName::CurativeRuneforms,
            vec![], // Increase healing 0.4 per crux
        ),
        PassiveData::new(
            // TODO: See how we can track this
            "Hideous Clarity",
            ClassName::Arcanist,
            SkillLineName::CurativeRuneforms,
            vec![], // Restore magicka and stamina on crux generation 225
        ),
        PassiveData::new(
            "Erudition",
            ClassName::Arcanist,
            SkillLineName::CurativeRuneforms,
            vec![], // Increase Magicka and Stamina recovery 0.18
        ),
        PassiveData::new(
            "Intricate Runeforms",
            ClassName::Arcanist,
            SkillLineName::CurativeRuneforms,
            vec![], // Damage shield increase 10%
        ),
    ]
});
