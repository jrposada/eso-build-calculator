use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
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
