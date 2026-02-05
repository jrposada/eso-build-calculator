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
