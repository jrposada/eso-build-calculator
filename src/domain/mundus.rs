use serde::{Deserialize, Serialize};
use std::fmt;

use super::CharacterStats;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MundusStone {
    Thief,
    Shadow,
    Warrior,
    Apprentice,
    Lover,
    Tower,
    Mage,
    Lord,
    Steed,
    Atronach,
    Serpent,
    Ritual,
}

/// DPS-relevant mundus stones for gear optimization.
pub const DPS_MUNDUS_STONES: &[MundusStone] = &[
    MundusStone::Thief,
    MundusStone::Shadow,
    MundusStone::Warrior,
    MundusStone::Apprentice,
    MundusStone::Lover,
    MundusStone::Tower,
    MundusStone::Mage,
];

impl MundusStone {
    /// Apply mundus stone bonus to stats, amplified by Divines trait.
    /// `divines_count` is the number of armor pieces with the Divines trait (0-7).
    pub fn apply(&self, stats: &mut CharacterStats, divines_count: u8) {
        let amp = 1.0 + 0.075 * divines_count as f64;
        match self {
            MundusStone::Thief => stats.critical_rating += 1_333.0 * amp,
            MundusStone::Shadow => stats.critical_damage += 0.11 * amp,
            MundusStone::Warrior => stats.weapon_damage += 238.0 * amp,
            MundusStone::Apprentice => stats.spell_damage += 238.0 * amp,
            MundusStone::Lover => stats.penetration += 2_744.0 * amp,
            MundusStone::Tower => stats.max_stamina += 2_023.0 * amp,
            MundusStone::Mage => stats.max_magicka += 2_023.0 * amp,
            MundusStone::Lord => {
                // +2,225 max health — not modeled (no max_health field)
            }
            MundusStone::Steed => {
                // +movement speed — not modeled
            }
            MundusStone::Atronach => {
                // +310 mag recovery — not modeled for DPS
            }
            MundusStone::Serpent => {
                // +310 stam recovery — not modeled for DPS
            }
            MundusStone::Ritual => {
                // +healing done — not modeled for DPS
            }
        }
    }

    pub fn parse(s: &str) -> Result<MundusStone, String> {
        match s.to_lowercase().replace(' ', "-").as_str() {
            "thief" => Ok(MundusStone::Thief),
            "shadow" => Ok(MundusStone::Shadow),
            "warrior" => Ok(MundusStone::Warrior),
            "apprentice" => Ok(MundusStone::Apprentice),
            "lover" => Ok(MundusStone::Lover),
            "tower" => Ok(MundusStone::Tower),
            "mage" => Ok(MundusStone::Mage),
            "lord" => Ok(MundusStone::Lord),
            "steed" => Ok(MundusStone::Steed),
            "atronach" => Ok(MundusStone::Atronach),
            "serpent" => Ok(MundusStone::Serpent),
            "ritual" => Ok(MundusStone::Ritual),
            _ => Err(format!(
                "Unknown mundus stone '{}'. Valid: thief, shadow, warrior, apprentice, \
                 lover, tower, mage, lord, steed, atronach, serpent, ritual",
                s
            )),
        }
    }
}

impl fmt::Display for MundusStone {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MundusStone::Thief => write!(f, "The Thief"),
            MundusStone::Shadow => write!(f, "The Shadow"),
            MundusStone::Warrior => write!(f, "The Warrior"),
            MundusStone::Apprentice => write!(f, "The Apprentice"),
            MundusStone::Lover => write!(f, "The Lover"),
            MundusStone::Tower => write!(f, "The Tower"),
            MundusStone::Mage => write!(f, "The Mage"),
            MundusStone::Lord => write!(f, "The Lord"),
            MundusStone::Steed => write!(f, "The Steed"),
            MundusStone::Atronach => write!(f, "The Atronach"),
            MundusStone::Serpent => write!(f, "The Serpent"),
            MundusStone::Ritual => write!(f, "The Ritual"),
        }
    }
}
