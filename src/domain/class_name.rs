use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SkillTree {
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

impl fmt::Display for SkillTree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SkillTree::Dragonknight => write!(f, "Dragonknight"),
            SkillTree::Necromancer => write!(f, "Necromancer"),
            SkillTree::Sorcerer => write!(f, "Sorcerer"),
            SkillTree::Nightblade => write!(f, "Nightblade"),
            SkillTree::Warden => write!(f, "Warden"),
            SkillTree::Templar => write!(f, "Templar"),
            SkillTree::Arcanist => write!(f, "Arcanist"),
            SkillTree::Weapon => write!(f, "Weapon"),
            SkillTree::Guild => write!(f, "Guild"),
        }
    }
}

impl SkillTree {
    pub const ALL: [SkillTree; 9] = [
        SkillTree::Dragonknight,
        SkillTree::Necromancer,
        SkillTree::Sorcerer,
        SkillTree::Nightblade,
        SkillTree::Warden,
        SkillTree::Templar,
        SkillTree::Arcanist,
        SkillTree::Weapon,
        SkillTree::Guild,
    ];

    pub const CLASS_ONLY: [SkillTree; 7] = [
        SkillTree::Dragonknight,
        SkillTree::Necromancer,
        SkillTree::Sorcerer,
        SkillTree::Nightblade,
        SkillTree::Warden,
        SkillTree::Templar,
        SkillTree::Arcanist,
    ];

    pub fn parse(s: &str) -> Result<SkillTree, String> {
        match s.trim().to_lowercase().as_str() {
            "arcanist" => Ok(SkillTree::Arcanist),
            "dragonknight" => Ok(SkillTree::Dragonknight),
            "necromancer" => Ok(SkillTree::Necromancer),
            "nightblade" => Ok(SkillTree::Nightblade),
            "sorcerer" => Ok(SkillTree::Sorcerer),
            "templar" => Ok(SkillTree::Templar),
            "warden" => Ok(SkillTree::Warden),
            _ => Err(format!(
                "Invalid class '{}'. Valid options: arcanist, dragonknight, necromancer, nightblade, sorcerer, templar, warden",
                s
            )),
        }
    }
}
