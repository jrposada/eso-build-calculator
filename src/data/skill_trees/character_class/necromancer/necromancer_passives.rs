use crate::domain::{BonusData, BonusSource, BonusValue, PassiveData};
use crate::domain::{BonusTarget, BonusTrigger, ClassName, SkillLineName};
use once_cell::sync::Lazy;

pub static NECROMANCER_PASSIVES: Lazy<Vec<PassiveData>> = Lazy::new(|| {
    vec![
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
        )
            .with_skill_id(116198),
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
        )
            .with_skill_id(116194),
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
        )
            .with_skill_id(116201),
        PassiveData::new(
            "Reusable Parts",
            ClassName::Necromancer,
            SkillLineName::GraveLord,
            vec![],
        )
            .with_skill_id(116188),
        PassiveData::new(
            "Death Gleaning",
            ClassName::Necromancer,
            SkillLineName::BoneTyrant,
            vec![],
        )
            .with_skill_id(116235),
        PassiveData::new(
            "Disdain Harm",
            ClassName::Necromancer,
            SkillLineName::BoneTyrant,
            vec![],
        )
            .with_skill_id(116240),
        PassiveData::new(
            "Health Avarice",
            ClassName::Necromancer,
            SkillLineName::BoneTyrant,
            vec![],
        )
            .with_skill_id(116270),
        PassiveData::new(
            "Last Gasp",
            ClassName::Necromancer,
            SkillLineName::BoneTyrant,
            vec![],
        )
            .with_skill_id(116272),
        PassiveData::new(
            "Corpse Consumption",
            ClassName::Necromancer,
            SkillLineName::LivingDeath,
            vec![],
        )
            .with_skill_id(116285),
        PassiveData::new(
            "Curative Curse",
            ClassName::Necromancer,
            SkillLineName::LivingDeath,
            vec![],
        )
            .with_skill_id(116287),
        PassiveData::new(
            "Undead Confederate",
            ClassName::Necromancer,
            SkillLineName::LivingDeath,
            vec![],
        )
            .with_skill_id(116283),
    ]
});

