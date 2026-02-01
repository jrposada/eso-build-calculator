use crate::data::{BonusTarget, BonusType, ClassName, SkillLineName};
use crate::domain::{BonusData, PassiveData};
use once_cell::sync::Lazy;

pub static DRAGONKNIGHT_PASSIVES: Lazy<Vec<PassiveData>> = Lazy::new(|| {
    vec![
        // === ARDENT FLAME ===
        PassiveData::new(
            "Combustion",
            ClassName::Dragonknight,
            SkillLineName::ArdentFlame,
            vec![],
        ),
        PassiveData::new(
            "Warmth",
            ClassName::Dragonknight,
            SkillLineName::ArdentFlame,
            vec![],
        ),
        PassiveData::new(
            "Searing Heat",
            ClassName::Dragonknight,
            SkillLineName::ArdentFlame,
            vec![BonusData::new(
                "Searing Heat",
                BonusType::SkillLine,
                BonusTarget::Duration,
                2.0,
            )],
        ),
        PassiveData::new(
            "World in Ruin",
            ClassName::Dragonknight,
            SkillLineName::ArdentFlame,
            vec![],
        ),
        // === DRACONIC POWER ===
        PassiveData::new(
            "Iron Skin",
            ClassName::Dragonknight,
            SkillLineName::DraconicPower,
            vec![],
        ),
        PassiveData::new(
            "Burning Heart",
            ClassName::Dragonknight,
            SkillLineName::DraconicPower,
            vec![],
        ),
        PassiveData::new(
            "Elder Dragon",
            ClassName::Dragonknight,
            SkillLineName::DraconicPower,
            vec![],
        ),
        PassiveData::new(
            "Scaled Armor",
            ClassName::Dragonknight,
            SkillLineName::DraconicPower,
            vec![],
        ),
        // === EARTHEN HEART ===
        PassiveData::new(
            "Eternal Mountain",
            ClassName::Dragonknight,
            SkillLineName::EarthenHeart,
            vec![],
        ),
        PassiveData::new(
            "Battle Roar",
            ClassName::Dragonknight,
            SkillLineName::EarthenHeart,
            vec![],
        ),
        PassiveData::new(
            "Mountain's Blessing",
            ClassName::Dragonknight,
            SkillLineName::EarthenHeart,
            vec![],
        ),
        PassiveData::new(
            "Helping Hands",
            ClassName::Dragonknight,
            SkillLineName::EarthenHeart,
            vec![],
        ),
    ]
});
