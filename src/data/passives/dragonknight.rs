use crate::data::bonuses::MINOR_BRUTALITY;
use crate::domain::{BonusData, BonusSource, BonusValue, PassiveData};
use crate::domain::{BonusTarget, BonusTrigger, ClassName, SkillLineName};
use once_cell::sync::Lazy;

pub static DRAGONKNIGHT_PASSIVES: Lazy<Vec<PassiveData>> = Lazy::new(|| {
    vec![
        // === ARDENT FLAME ===
        PassiveData::new(
            "Combustion",
            ClassName::Dragonknight,
            SkillLineName::ArdentFlame,
            vec![
                BonusData::new(
                    "Combustion (Damage)",
                    BonusSource::Passive,
                    BonusTrigger::Passive,
                    BonusValue::new(
                        "Combustion (Damage)",
                        BonusTarget::BurningAndPoisonDamage,
                        0.33,
                    ),
                ),
                BonusData::new(
                    "Combustion (Restore)",
                    BonusSource::Passive,
                    BonusTrigger::BurningOrPoisonDamageDealt,
                    BonusValue::new(
                        "Combustion (Restore)",
                        BonusTarget::RestoreMagickaOrStamina,
                        423.0,
                    ),
                )
                .with_cooldown(3.0),
            ],
        ),
        PassiveData::new(
            "Warmth",
            ClassName::Dragonknight,
            SkillLineName::ArdentFlame,
            vec![BonusData::new(
                "Warmth",
                BonusSource::Passive,
                BonusTrigger::SkillLineSkillCast,
                BonusValue::new("Warmth", BonusTarget::AoeDamage, 0.06),
            )
            .with_duration(3.0)],
        ),
        PassiveData::new(
            "Searing Heat",
            ClassName::Dragonknight,
            SkillLineName::ArdentFlame,
            vec![], // TODO: Passive effect to specific skills.
        ),
        PassiveData::new(
            "World in Ruin",
            ClassName::Dragonknight,
            SkillLineName::ArdentFlame,
            vec![BonusData::new(
                "World in Ruin",
                BonusSource::Passive,
                BonusTrigger::Passive,
                BonusValue::new("World in Ruin", BonusTarget::BurningAndPoisonDamage, 0.05),
            )],
        ),
        // === DRACONIC POWER ===
        PassiveData::new(
            "Iron Skin",
            ClassName::Dragonknight,
            SkillLineName::DraconicPower,
            vec![], // Increase blocked damage 10%
        ),
        PassiveData::new(
            "Burning Heart",
            ClassName::Dragonknight,
            SkillLineName::DraconicPower,
            vec![], // Increase healing received 9%
        ),
        PassiveData::new(
            "Elder Dragon",
            ClassName::Dragonknight,
            SkillLineName::DraconicPower,
            vec![], // Increase health recovery
        ),
        PassiveData::new(
            "Scaled Armor",
            ClassName::Dragonknight,
            SkillLineName::DraconicPower,
            vec![], // Increase Physical and Spell Resistance 2974
        ),
        // === EARTHEN HEART ===
        PassiveData::new(
            "Eternal Mountain",
            ClassName::Dragonknight,
            SkillLineName::EarthenHeart,
            vec![BonusData::new(
                "Eternal Mountain",
                BonusSource::Passive,
                BonusTrigger::Passive,
                BonusValue::new(
                    "Eternal Mountain",
                    BonusTarget::DurationSkillLineMultiplier,
                    0.2,
                ),
            )],
        ),
        PassiveData::new(
            "Battle Roar",
            ClassName::Dragonknight,
            SkillLineName::EarthenHeart,
            vec![], // TODO: To complex, restore resources on ult cast
        ),
        PassiveData::new(
            "Mountain's Blessing",
            ClassName::Dragonknight,
            SkillLineName::EarthenHeart,
            vec![
                MINOR_BRUTALITY
                    .clone()
                    .with_trigger(BonusTrigger::SkillLineSkillCast)
                    .with_cooldown(6.0),
                // TODO: Generates 3 ultimate
            ],
        ),
        PassiveData::new(
            "Helping Hands",
            ClassName::Dragonknight,
            SkillLineName::EarthenHeart,
            vec![], // TODO: To complex, restore stamina on skill use with multiple conditions
        ),
    ]
});
