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
    // Necromancer
    GraveLord,
    BoneTyrant,
    LivingDeath,
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
    // Guild
    FightersGuild,
    MagesGuild,
    Undaunted,
    PsijicOrder,
}

impl fmt::Display for SkillLineName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SkillLineName::CurativeRuneforms => write!(f, "Curative Runeforms"),
            SkillLineName::SoldierOfApocrypha => write!(f, "Soldier of Apocrypha"),
            SkillLineName::HeraldOfTheTome => write!(f, "Herald of The Tome"),
            SkillLineName::ArdentFlame => write!(f, "Ardent Flame"),
            SkillLineName::DraconicPower => write!(f, "Draconic Power"),
            SkillLineName::EarthenHeart => write!(f, "Earthen Heart"),
            SkillLineName::GraveLord => write!(f, "Grave Lord"),
            SkillLineName::BoneTyrant => write!(f, "Bone Tyrant"),
            SkillLineName::LivingDeath => write!(f, "Living Death"),
            SkillLineName::Assassination => write!(f, "Assassination"),
            SkillLineName::Shadow => write!(f, "Shadow"),
            SkillLineName::Siphoning => write!(f, "Siphoning"),
            SkillLineName::DarkMagic => write!(f, "Dark Magic"),
            SkillLineName::DaedricSummoning => write!(f, "Daedric Summoning"),
            SkillLineName::StormCalling => write!(f, "Storm Calling"),
            SkillLineName::AedricSpear => write!(f, "Aedric Spear"),
            SkillLineName::DawnsWrath => write!(f, "Dawn's Wrath"),
            SkillLineName::RestoringLight => write!(f, "Restoring Light"),
            SkillLineName::AnimalCompanions => write!(f, "Animal Companions"),
            SkillLineName::GreenBalance => write!(f, "Green Balance"),
            SkillLineName::WintersEmbrace => write!(f, "Winters Embrace"),
            SkillLineName::Bow => write!(f, "Bow"),
            SkillLineName::TwoHanded => write!(f, "Two-Handed"),
            SkillLineName::DestructionStaff => write!(f, "Destruction Staff"),
            SkillLineName::DualWield => write!(f, "Dual Wield"),
            SkillLineName::FightersGuild => write!(f, "Fighters Guild"),
            SkillLineName::MagesGuild => write!(f, "Mages Guild"),
            SkillLineName::Undaunted => write!(f, "Undaunted"),
            SkillLineName::PsijicOrder => write!(f, "Psijic Order"),
        }
    }
}

impl SkillLineName {
    pub const ALL: [SkillLineName; 29] = [
        SkillLineName::CurativeRuneforms,
        SkillLineName::SoldierOfApocrypha,
        SkillLineName::HeraldOfTheTome,
        SkillLineName::ArdentFlame,
        SkillLineName::DraconicPower,
        SkillLineName::EarthenHeart,
        SkillLineName::GraveLord,
        SkillLineName::BoneTyrant,
        SkillLineName::LivingDeath,
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
        SkillLineName::FightersGuild,
        SkillLineName::MagesGuild,
        SkillLineName::Undaunted,
        SkillLineName::PsijicOrder,
    ];

    pub const WEAPON: [SkillLineName; 4] = [
        SkillLineName::Bow,
        SkillLineName::TwoHanded,
        SkillLineName::DestructionStaff,
        SkillLineName::DualWield,
    ];

    pub const GUILD: [SkillLineName; 4] = [
        SkillLineName::FightersGuild,
        SkillLineName::MagesGuild,
        SkillLineName::Undaunted,
        SkillLineName::PsijicOrder,
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

            SkillLineName::GraveLord
            | SkillLineName::BoneTyrant
            | SkillLineName::LivingDeath => ClassName::Necromancer,

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

            SkillLineName::FightersGuild
            | SkillLineName::MagesGuild
            | SkillLineName::Undaunted
            | SkillLineName::PsijicOrder => ClassName::Guild,
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

    /// Check if this skill line is a guild skill line
    pub fn is_guild(&self) -> bool {
        matches!(
            self,
            SkillLineName::FightersGuild
                | SkillLineName::MagesGuild
                | SkillLineName::Undaunted
                | SkillLineName::PsijicOrder
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
            ClassName::Necromancer => vec![
                SkillLineName::GraveLord,
                SkillLineName::BoneTyrant,
                SkillLineName::LivingDeath,
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
            ClassName::Guild => vec![
                SkillLineName::FightersGuild,
                SkillLineName::MagesGuild,
                SkillLineName::Undaunted,
                SkillLineName::PsijicOrder,
            ],
        }
    }
}
