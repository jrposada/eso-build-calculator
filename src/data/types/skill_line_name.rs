use serde::{Deserialize, Serialize};
use std::fmt;

use super::class_name::ClassName;

/// Skill line names for all classes and weapons
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SkillLineName {
    // Arcanist
    CurativeRuneforms,
    SoldierOfApocrypha,
    HeraldOfTheTome,
    // Dragonknight
    ArdentFlame,
    DraconicPower,
    EarthenHeart,
    // Nightblade
    Assassination,
    Shadow,
    Siphoning,
    // Sorcerer
    DarkMagic,
    DaedricSummoning,
    StormCalling,
    // Templar
    AedricSpear,
    DawnsWrath,
    RestoringLight,
    // Warden
    AnimalCompanions,
    GreenBalance,
    WintersEmbrace,
    // Weapons
    Bow,
    TwoHanded,
    DestructionStaff,
    DualWield,
}

impl fmt::Display for SkillLineName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SkillLineName::CurativeRuneforms => write!(f, "CurativeRuneforms"),
            SkillLineName::SoldierOfApocrypha => write!(f, "SoldierOfApocrypha"),
            SkillLineName::HeraldOfTheTome => write!(f, "HeraldOfTheTome"),
            SkillLineName::ArdentFlame => write!(f, "ArdentFlame"),
            SkillLineName::DraconicPower => write!(f, "DraconicPower"),
            SkillLineName::EarthenHeart => write!(f, "EarthenHeart"),
            SkillLineName::Assassination => write!(f, "Assassination"),
            SkillLineName::Shadow => write!(f, "Shadow"),
            SkillLineName::Siphoning => write!(f, "Siphoning"),
            SkillLineName::DarkMagic => write!(f, "DarkMagic"),
            SkillLineName::DaedricSummoning => write!(f, "DaedricSummoning"),
            SkillLineName::StormCalling => write!(f, "StormCalling"),
            SkillLineName::AedricSpear => write!(f, "AedricSpear"),
            SkillLineName::DawnsWrath => write!(f, "DawnsWrath"),
            SkillLineName::RestoringLight => write!(f, "RestoringLight"),
            SkillLineName::AnimalCompanions => write!(f, "AnimalCompanions"),
            SkillLineName::GreenBalance => write!(f, "GreenBalance"),
            SkillLineName::WintersEmbrace => write!(f, "WintersEmbrace"),
            SkillLineName::Bow => write!(f, "Bow"),
            SkillLineName::TwoHanded => write!(f, "TwoHanded"),
            SkillLineName::DestructionStaff => write!(f, "DestructionStaff"),
            SkillLineName::DualWield => write!(f, "DualWield"),
        }
    }
}

impl SkillLineName {
    pub const ALL: [SkillLineName; 22] = [
        SkillLineName::CurativeRuneforms,
        SkillLineName::SoldierOfApocrypha,
        SkillLineName::HeraldOfTheTome,
        SkillLineName::ArdentFlame,
        SkillLineName::DraconicPower,
        SkillLineName::EarthenHeart,
        SkillLineName::Assassination,
        SkillLineName::Shadow,
        SkillLineName::Siphoning,
        SkillLineName::DarkMagic,
        SkillLineName::DaedricSummoning,
        SkillLineName::StormCalling,
        SkillLineName::AedricSpear,
        SkillLineName::DawnsWrath,
        SkillLineName::RestoringLight,
        SkillLineName::AnimalCompanions,
        SkillLineName::GreenBalance,
        SkillLineName::WintersEmbrace,
        SkillLineName::Bow,
        SkillLineName::TwoHanded,
        SkillLineName::DestructionStaff,
        SkillLineName::DualWield,
    ];

    pub const WEAPON: [SkillLineName; 4] = [
        SkillLineName::Bow,
        SkillLineName::TwoHanded,
        SkillLineName::DestructionStaff,
        SkillLineName::DualWield,
    ];

    /// Get the class that this skill line belongs to
    pub fn get_class(&self) -> ClassName {
        match self {
            SkillLineName::CurativeRuneforms
            | SkillLineName::SoldierOfApocrypha
            | SkillLineName::HeraldOfTheTome => ClassName::Arcanist,

            SkillLineName::ArdentFlame
            | SkillLineName::DraconicPower
            | SkillLineName::EarthenHeart => ClassName::Dragonknight,

            SkillLineName::Assassination | SkillLineName::Shadow | SkillLineName::Siphoning => {
                ClassName::Nightblade
            }

            SkillLineName::DarkMagic
            | SkillLineName::DaedricSummoning
            | SkillLineName::StormCalling => ClassName::Sorcerer,

            SkillLineName::AedricSpear
            | SkillLineName::DawnsWrath
            | SkillLineName::RestoringLight => ClassName::Templar,

            SkillLineName::AnimalCompanions
            | SkillLineName::GreenBalance
            | SkillLineName::WintersEmbrace => ClassName::Warden,

            SkillLineName::Bow
            | SkillLineName::TwoHanded
            | SkillLineName::DestructionStaff
            | SkillLineName::DualWield => ClassName::Weapon,
        }
    }

    /// Check if this skill line is a weapon skill line
    pub fn is_weapon(&self) -> bool {
        matches!(
            self,
            SkillLineName::Bow
                | SkillLineName::TwoHanded
                | SkillLineName::DestructionStaff
                | SkillLineName::DualWield
        )
    }

    /// Get all skill lines for a given class
    pub fn for_class(class: ClassName) -> Vec<SkillLineName> {
        match class {
            ClassName::Arcanist => vec![
                SkillLineName::CurativeRuneforms,
                SkillLineName::SoldierOfApocrypha,
                SkillLineName::HeraldOfTheTome,
            ],
            ClassName::Dragonknight => vec![
                SkillLineName::ArdentFlame,
                SkillLineName::DraconicPower,
                SkillLineName::EarthenHeart,
            ],
            ClassName::Nightblade => vec![
                SkillLineName::Assassination,
                SkillLineName::Shadow,
                SkillLineName::Siphoning,
            ],
            ClassName::Sorcerer => vec![
                SkillLineName::DarkMagic,
                SkillLineName::DaedricSummoning,
                SkillLineName::StormCalling,
            ],
            ClassName::Templar => vec![
                SkillLineName::AedricSpear,
                SkillLineName::DawnsWrath,
                SkillLineName::RestoringLight,
            ],
            ClassName::Warden => vec![
                SkillLineName::AnimalCompanions,
                SkillLineName::GreenBalance,
                SkillLineName::WintersEmbrace,
            ],
            ClassName::Weapon => vec![
                SkillLineName::Bow,
                SkillLineName::TwoHanded,
                SkillLineName::DestructionStaff,
                SkillLineName::DualWield,
            ],
        }
    }
}
