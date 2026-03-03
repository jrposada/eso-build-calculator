use std::fmt;

use super::CharacterStats;
pub use crate::data::skill_trees::race::Race;

/// All races for gear optimization.
pub const DPS_RACES: &[Race] = &[
    Race::Altmer,
    Race::Argonian,
    Race::Bosmer,
    Race::Breton,
    Race::Dunmer,
    Race::Imperial,
    Race::Khajiit,
    Race::Nord,
    Race::Orc,
    Race::Redguard,
];

impl Race {
    /// Apply racial passive bonuses to character stats.
    pub fn apply(&self, stats: &mut CharacterStats) {
        match self {
            Race::Altmer => {
                stats.max_magicka += 2_000.0;
                stats.weapon_damage += 258.0;
                stats.spell_damage += 258.0;
            }
            Race::Argonian => {
                stats.max_magicka += 1_000.0;
                // +1,000 health not modeled (no max_health field)
            }
            Race::Bosmer => {
                stats.max_stamina += 2_000.0;
                stats.penetration += 950.0;
            }
            Race::Breton => {
                stats.max_magicka += 2_000.0;
            }
            Race::Dunmer => {
                stats.max_stamina += 1_910.0;
                stats.max_magicka += 1_910.0;
                stats.weapon_damage += 258.0;
                stats.spell_damage += 258.0;
            }
            Race::Imperial => {
                stats.max_stamina += 2_000.0;
                // +2,000 health not modeled
            }
            Race::Khajiit => {
                stats.max_stamina += 915.0;
                stats.max_magicka += 915.0;
                stats.critical_damage += 0.12;
                // +915 health not modeled
            }
            Race::Nord => {
                stats.max_stamina += 1_500.0;
                // +1,000 health not modeled
            }
            Race::Orc => {
                stats.max_stamina += 1_000.0;
                stats.weapon_damage += 258.0;
                // +1,000 health not modeled
            }
            Race::Redguard => {
                stats.max_stamina += 2_000.0;
            }
        }
    }

    pub fn parse(s: &str) -> Result<Race, String> {
        match s.to_lowercase().replace(' ', "-").as_str() {
            "altmer" | "high-elf" => Ok(Race::Altmer),
            "argonian" => Ok(Race::Argonian),
            "bosmer" | "wood-elf" => Ok(Race::Bosmer),
            "breton" => Ok(Race::Breton),
            "dunmer" | "dark-elf" => Ok(Race::Dunmer),
            "imperial" => Ok(Race::Imperial),
            "khajiit" => Ok(Race::Khajiit),
            "nord" => Ok(Race::Nord),
            "orc" | "orsimer" => Ok(Race::Orc),
            "redguard" => Ok(Race::Redguard),
            _ => Err(format!(
                "Unknown race '{}'. Valid: altmer/high-elf, argonian, bosmer/wood-elf, breton, \
                 dunmer/dark-elf, imperial, khajiit, nord, orc/orsimer, redguard",
                s
            )),
        }
    }
}

impl fmt::Display for Race {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Race::Altmer => write!(f, "Altmer"),
            Race::Argonian => write!(f, "Argonian"),
            Race::Bosmer => write!(f, "Bosmer"),
            Race::Breton => write!(f, "Breton"),
            Race::Dunmer => write!(f, "Dunmer"),
            Race::Imperial => write!(f, "Imperial"),
            Race::Khajiit => write!(f, "Khajiit"),
            Race::Nord => write!(f, "Nord"),
            Race::Orc => write!(f, "Orc"),
            Race::Redguard => write!(f, "Redguard"),
        }
    }
}
