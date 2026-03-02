use serde::{Deserialize, Serialize};
use std::fmt;

use super::CharacterStats;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Food {
    LavaFoot,
    GhastlyEyeBowl,
    BewitchedSugarSkulls,
}

/// DPS-relevant foods for gear optimization.
pub const DPS_FOODS: &[Food] = &[
    Food::LavaFoot,
    Food::GhastlyEyeBowl,
    Food::BewitchedSugarSkulls,
];

impl Food {
    /// Apply food buff to character stats.
    pub fn apply(&self, stats: &mut CharacterStats) {
        match self {
            Food::LavaFoot => {
                stats.max_stamina += 4_936.0;
            }
            Food::GhastlyEyeBowl => {
                stats.max_magicka += 4_936.0;
            }
            Food::BewitchedSugarSkulls => {
                stats.max_stamina += 4_250.0;
                stats.max_magicka += 4_250.0;
                // +4,620 health not modeled
            }
        }
    }

    pub fn parse(s: &str) -> Result<Food, String> {
        match s.to_lowercase().replace(' ', "-").as_str() {
            "lava-foot" | "lava-foot-soup" | "lava-foot-soup-and-saltrice" => Ok(Food::LavaFoot),
            "ghastly-eye" | "ghastly-eye-bowl" => Ok(Food::GhastlyEyeBowl),
            "sugar-skulls" | "bewitched-sugar-skulls" => Ok(Food::BewitchedSugarSkulls),
            _ => Err(format!(
                "Unknown food '{}'. Valid: lava-foot, ghastly-eye, sugar-skulls",
                s
            )),
        }
    }
}

impl fmt::Display for Food {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Food::LavaFoot => write!(f, "Lava Foot Soup-and-Saltrice"),
            Food::GhastlyEyeBowl => write!(f, "Ghastly Eye Bowl"),
            Food::BewitchedSugarSkulls => write!(f, "Bewitched Sugar Skulls"),
        }
    }
}
