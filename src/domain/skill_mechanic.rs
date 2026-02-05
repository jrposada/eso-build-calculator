use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SkillMechanic {
    Dot,
    Instant,
    Channeled,
}

impl fmt::Display for SkillMechanic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SkillMechanic::Dot => write!(f, "dot"),
            SkillMechanic::Instant => write!(f, "instant"),
            SkillMechanic::Channeled => write!(f, "channeled"),
        }
    }
}
