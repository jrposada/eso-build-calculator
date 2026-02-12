use crate::domain::{BonusData, BonusSource, BonusValue, PassiveData};
use crate::domain::{BonusTarget, BonusTrigger, ClassName, SkillLineName};
use once_cell::sync::Lazy;

pub static NECROMANCER_PASSIVES: Lazy<Vec<PassiveData>> = Lazy::new(|| {
    vec![
        // === GRAVE LORD ===
        // Death Knell: +20% Critical Strike Chance against enemies under 33% Health
        // 20% crit chance ≈ 4382 crit rating (21912 rating = 100% crit)
        PassiveData::new(
            "Death Knell",
            ClassName::Necromancer,
            SkillLineName::GraveLord,
            vec![BonusData::new(
                "Death Knell",
                BonusSource::Passive,
                BonusTrigger::Passive,
                BonusValue::new("Death Knell", BonusTarget::CriticalRating, 4382.0),
            )
            .with_execute_threshold(0.33)],
        ),
        // Dismember: +3271 Physical and Spell Penetration while a Grave Lord ability is active
        PassiveData::new(
            "Dismember",
            ClassName::Necromancer,
            SkillLineName::GraveLord,
            vec![BonusData::new(
                "Dismember",
                BonusSource::Passive,
                BonusTrigger::SkillLineSlotted,
                BonusValue::new(
                    "Dismember",
                    BonusTarget::PhysicalAndSpellPenetration,
                    3271.0,
                ),
            )],
        ),
        // Rapid Rot: +10% damage done with damage over time effects
        PassiveData::new(
            "Rapid Rot",
            ClassName::Necromancer,
            SkillLineName::GraveLord,
            vec![BonusData::new(
                "Rapid Rot",
                BonusSource::Passive,
                BonusTrigger::Passive,
                BonusValue::new("Rapid Rot", BonusTarget::DotDamage, 0.10),
            )],
        ),
        // Reusable Parts: Cost reduction when pet dies - not tracked in damage calculations
        PassiveData::new(
            "Reusable Parts",
            ClassName::Necromancer,
            SkillLineName::GraveLord,
            vec![],
        ),
        // === BONE TYRANT ===
        // Death Gleaning: Restore 666 Magicka and Stamina when enemy dies - not tracked
        PassiveData::new(
            "Death Gleaning",
            ClassName::Necromancer,
            SkillLineName::BoneTyrant,
            vec![],
        ),
        // Disdain Harm: -15% DoT damage taken while Bone Tyrant ability active - defensive, not tracked
        PassiveData::new(
            "Disdain Harm",
            ClassName::Necromancer,
            SkillLineName::BoneTyrant,
            vec![],
        ),
        // Health Avarice: +3% Healing Received per Bone Tyrant ability slotted - not tracked
        PassiveData::new(
            "Health Avarice",
            ClassName::Necromancer,
            SkillLineName::BoneTyrant,
            vec![],
        ),
        // Last Gasp: +2412 Max Health - not tracked in damage calculations
        PassiveData::new(
            "Last Gasp",
            ClassName::Necromancer,
            SkillLineName::BoneTyrant,
            vec![],
        ),
        // === LIVING DEATH ===
        // Corpse Consumption: +10 Ultimate when consuming a corpse (16s cooldown) - not tracked
        PassiveData::new(
            "Corpse Consumption",
            ClassName::Necromancer,
            SkillLineName::LivingDeath,
            vec![],
        ),
        // Curative Curse: +12% healing done while you have a negative effect - not tracked
        PassiveData::new(
            "Curative Curse",
            ClassName::Necromancer,
            SkillLineName::LivingDeath,
            vec![],
        ),
        // Undead Confederate: +155 Health, Magicka, and Stamina Recovery with pet active - not tracked
        PassiveData::new(
            "Undead Confederate",
            ClassName::Necromancer,
            SkillLineName::LivingDeath,
            vec![],
        ),
    ]
});
