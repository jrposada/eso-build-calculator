use crate::domain::{BonusData, BonusSource, BonusValue, PassiveData};
use crate::domain::{BonusTarget, BonusTrigger, ClassName, SkillLineName};
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
                BonusSource::Passive,
                BonusTrigger::ArcanistCrux,
                BonusValue::new("Fated Fortune", BonusTarget::CriticalDamage, 0.12),
            )
            .with_duration(7.0)],
        ),
        PassiveData::new(
            "Harnessed Quintessence",
            ClassName::Arcanist,
            SkillLineName::HeraldOfTheTome,
            vec![BonusData::new(
                "Harnessed Quintessence W",
                BonusSource::Passive,
                BonusTrigger::MagickaOrStaminaRestored,
                BonusValue::new(
                    "Harnessed Quintessence W",
                    BonusTarget::WeaponAndSpellDamageFlat,
                    284.0,
                ),
            )
            .with_duration(10.0)],
        ),
        // +15% Status Effect Damage, +55% Status Effect Chance
        // Status effects include Burning, Poisoned, Chilled, etc. - complex to model directly
        PassiveData::new(
            "Psychic Lesion",
            ClassName::Arcanist,
            SkillLineName::HeraldOfTheTome,
            vec![
                BonusData::new(
                    "Psychic Lesion (Damage)",
                    BonusSource::Passive,
                    BonusTrigger::AbilitySlotted,
                    BonusValue::new(
                        "Psychic Lesion (Damage)",
                        BonusTarget::StatusEffectDamage,
                        0.15,
                    ),
                )
                .with_duration(10.0),
                BonusData::new(
                    "Psychic Lesion (Chance)",
                    BonusSource::Passive,
                    BonusTrigger::AbilitySlotted,
                    BonusValue::new(
                        "Psychic Lesion (Chance)",
                        BonusTarget::StatusEffectChance,
                        0.55,
                    ),
                )
                .with_duration(10.0),
            ],
        ),
        PassiveData::new(
            "Splintered Secrets",
            ClassName::Arcanist,
            SkillLineName::HeraldOfTheTome,
            vec![BonusData::new(
                "Splintered Secrets",
                BonusSource::Passive,
                BonusTrigger::AbilitySlottedCount,
                BonusValue::new(
                    "Splintered Secrets",
                    BonusTarget::PhysicalAndSpellPenetration,
                    1240.0,
                ),
            )],
        ),
        // === SOLDIER OF APOCRYPHA ===
        // +3271 Armor - defensive, not tracked in damage calculations
        PassiveData::new(
            "Aegis of the Unseen",
            ClassName::Arcanist,
            SkillLineName::SoldierOfApocrypha,
            vec![],
        ),
        // +81 Magicka, Stamina, and Health Recovery - not tracked in damage calculations
        PassiveData::new(
            "Wellspring of the Abyss",
            ClassName::Arcanist,
            SkillLineName::SoldierOfApocrypha,
            vec![],
        ),
        // Minor Evasion (5% AoE damage reduction) - defensive, not tracked
        PassiveData::new(
            "Circumvented Fate",
            ClassName::Arcanist,
            SkillLineName::SoldierOfApocrypha,
            vec![],
        ),
        // +4 Ultimate when generating Crux - resource generation, not tracked
        PassiveData::new(
            "Implacable Outcome",
            ClassName::Arcanist,
            SkillLineName::SoldierOfApocrypha,
            vec![],
        ),
        // === CURATIVE RUNEFORMS ===
        // +4% Healing done per Crux - healing, not tracked in damage calculations
        PassiveData::new(
            "Healing Tides",
            ClassName::Arcanist,
            SkillLineName::CurativeRuneforms,
            vec![],
        ),
        PassiveData::new(
            "Hideous Clarity",
            ClassName::Arcanist,
            SkillLineName::CurativeRuneforms,
            vec![BonusData::new(
                "Hideous Clarity",
                BonusSource::Passive,
                BonusTrigger::ArcanistCrux, // TODO: should be only on generate
                BonusValue::new(
                    "Hideous Clarity",
                    BonusTarget::RestoreMagickaOrStamina,
                    225.0,
                ),
            )],
        ),
        // +18% Magicka and Stamina Recovery - recovery, not tracked
        PassiveData::new(
            "Erudition",
            ClassName::Arcanist,
            SkillLineName::CurativeRuneforms,
            vec![],
        ),
        // +10% Damage Shield strength - defensive, not tracked
        PassiveData::new(
            "Intricate Runeforms",
            ClassName::Arcanist,
            SkillLineName::CurativeRuneforms,
            vec![],
        ),
    ]
});
