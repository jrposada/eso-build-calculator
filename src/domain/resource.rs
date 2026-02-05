use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
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
            Resource::Stamina => write!(f, "Stamina"),
            Resource::Health => write!(f, "Health"),
            Resource::Ultimate => write!(f, "Ultimate"),
        }
    }
}
