use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ClassName {
    Dragonknight,
    Necromancer,
    Sorcerer,
    Nightblade,
    Warden,
    Templar,
    Arcanist,
    Weapon,
    Guild,
}

impl fmt::Display for ClassName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ClassName::Dragonknight => write!(f, "Dragonknight"),
            ClassName::Necromancer => write!(f, "Necromancer"),
            ClassName::Sorcerer => write!(f, "Sorcerer"),
            ClassName::Nightblade => write!(f, "Nightblade"),
            ClassName::Warden => write!(f, "Warden"),
            ClassName::Templar => write!(f, "Templar"),
            ClassName::Arcanist => write!(f, "Arcanist"),
            ClassName::Weapon => write!(f, "Weapon"),
            ClassName::Guild => write!(f, "Guild"),
        }
    }
}

impl ClassName {
    pub const ALL: [ClassName; 9] = [
        ClassName::Dragonknight,
        ClassName::Necromancer,
        ClassName::Sorcerer,
        ClassName::Nightblade,
        ClassName::Warden,
        ClassName::Templar,
        ClassName::Arcanist,
        ClassName::Weapon,
        ClassName::Guild,
    ];

    pub const CLASS_ONLY: [ClassName; 7] = [
        ClassName::Dragonknight,
        ClassName::Necromancer,
        ClassName::Sorcerer,
        ClassName::Nightblade,
        ClassName::Warden,
        ClassName::Templar,
        ClassName::Arcanist,
    ];
}
