use serde::{Deserialize, Serialize};
use std::fmt;

/// Class names for ESO characters
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ClassName {
    Dragonknight,
    Sorcerer,
    Nightblade,
    Warden,
    Templar,
    Arcanist,
    Weapon,
}

impl fmt::Display for ClassName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ClassName::Dragonknight => write!(f, "Dragonknight"),
            ClassName::Sorcerer => write!(f, "Sorcerer"),
            ClassName::Nightblade => write!(f, "Nightblade"),
            ClassName::Warden => write!(f, "Warden"),
            ClassName::Templar => write!(f, "Templar"),
            ClassName::Arcanist => write!(f, "Arcanist"),
            ClassName::Weapon => write!(f, "Weapon"),
        }
    }
}

impl ClassName {
    pub const ALL: [ClassName; 7] = [
        ClassName::Dragonknight,
        ClassName::Sorcerer,
        ClassName::Nightblade,
        ClassName::Warden,
        ClassName::Templar,
        ClassName::Arcanist,
        ClassName::Weapon,
    ];

    pub const CLASS_ONLY: [ClassName; 6] = [
        ClassName::Dragonknight,
        ClassName::Sorcerer,
        ClassName::Nightblade,
        ClassName::Warden,
        ClassName::Templar,
        ClassName::Arcanist,
    ];
}

/// Resource types for skills
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Resource {
    Magicka,
    Stamina,
    Health,
    Ultimate,
}

impl fmt::Display for Resource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Resource::Magicka => write!(f, "magicka"),
            Resource::Stamina => write!(f, "stamina"),
            Resource::Health => write!(f, "health"),
            Resource::Ultimate => write!(f, "ultimate"),
        }
    }
}

/// Damage types in ESO
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DamageType {
    Magic,
    Physical,
    Disease,
    Flame,
    Poison,
    Bleed,
    Frost,
    Shock,
}

impl fmt::Display for DamageType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DamageType::Magic => write!(f, "magic"),
            DamageType::Physical => write!(f, "physical"),
            DamageType::Disease => write!(f, "disease"),
            DamageType::Flame => write!(f, "flame"),
            DamageType::Poison => write!(f, "poison"),
            DamageType::Bleed => write!(f, "bleed"),
            DamageType::Frost => write!(f, "frost"),
            DamageType::Shock => write!(f, "shock"),
        }
    }
}

/// Target types for skills
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TargetType {
    Single,
    Aoe,
}

impl fmt::Display for TargetType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TargetType::Single => write!(f, "single"),
            TargetType::Aoe => write!(f, "aoe"),
        }
    }
}

/// Skill mechanic types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SkillMechanic {
    Dot,
    Instant,
    Channeled,
    Unknown,
}

impl fmt::Display for SkillMechanic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SkillMechanic::Dot => write!(f, "dot"),
            SkillMechanic::Instant => write!(f, "instant"),
            SkillMechanic::Channeled => write!(f, "channeled"),
            SkillMechanic::Unknown => write!(f, "unknown"),
        }
    }
}

/// Skill type - class or weapon
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SkillType {
    Class,
    Weapon,
}

/// How a bonus is applied
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum BonusClassName {
    /// Always applied
    Passive,
    /// Always applied for a duration
    Duration,
    /// Applied once if skill line is part of build
    SkillLine,
    /// Applied once if at least 1 skill from skill line is equipped on build
    AbilitySlotted,
    /// Applied once per skill of related skill line equipped on build
    AbilitySlottedCount,
}

/// What stat the bonus affects
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum BonusType {
    AoeDamage,
    CriticalChance,
    CriticalDamage,
    DirectDamage,
    DotDamage,
    Duration,
    MaxMagicka,
    MaxStamina,
    OffBalanceDamage,
    SingleDamage,
    SpellCriticalChance,
    SpellDamage,
    WeaponCriticalChance,
}

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
