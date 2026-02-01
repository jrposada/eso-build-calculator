use crate::data::{BonusTarget, BonusType, ClassName, SkillLineName};
use crate::domain::{BonusData, PassiveData};
use once_cell::sync::Lazy;

pub static ARCANIST_PASSIVES: Lazy<Vec<PassiveData>> = Lazy::new(|| {
    vec![
        // === HERALD OF THE TOME ===
        PassiveData::new(
            "Scion of Apocrypha",
            ClassName::Arcanist,
            SkillLineName::HeraldOfTheTome,
            vec![],
        ),
        PassiveData::new(
            "Meticulous Curation",
            ClassName::Arcanist,
            SkillLineName::HeraldOfTheTome,
            vec![],
        ),
        PassiveData::new(
            "Sage-Sight Aura",
            ClassName::Arcanist,
            SkillLineName::HeraldOfTheTome,
            vec![],
        ),
        PassiveData::new(
            "Tome-Bearer's Inspiration",
            ClassName::Arcanist,
            SkillLineName::HeraldOfTheTome,
            vec![BonusData::new(
                "Tome-Bearer's Inspiration",
                BonusType::AbilitySlottedCount,
                BonusTarget::CriticalChance,
                0.03,
            )],
        ),
        // === SOLDIER OF APOCRYPHA ===
        PassiveData::new(
            "Seeker's Will",
            ClassName::Arcanist,
            SkillLineName::SoldierOfApocrypha,
            vec![],
        ),
        PassiveData::new(
            "Resonating Glyphs",
            ClassName::Arcanist,
            SkillLineName::SoldierOfApocrypha,
            vec![],
        ),
        PassiveData::new(
            "Hidden Knowledge",
            ClassName::Arcanist,
            SkillLineName::SoldierOfApocrypha,
            vec![],
        ),
        PassiveData::new(
            "Cruxweaver Armor",
            ClassName::Arcanist,
            SkillLineName::SoldierOfApocrypha,
            vec![],
        ),
        // === CURATIVE RUNEFORMS ===
        PassiveData::new(
            "Erudition",
            ClassName::Arcanist,
            SkillLineName::CurativeRuneforms,
            vec![],
        ),
        PassiveData::new(
            "Circumscribed Recovery",
            ClassName::Arcanist,
            SkillLineName::CurativeRuneforms,
            vec![],
        ),
        PassiveData::new(
            "Healing Tides",
            ClassName::Arcanist,
            SkillLineName::CurativeRuneforms,
            vec![],
        ),
        PassiveData::new(
            "Curator's Focus",
            ClassName::Arcanist,
            SkillLineName::CurativeRuneforms,
            vec![],
        ),
    ]
});
