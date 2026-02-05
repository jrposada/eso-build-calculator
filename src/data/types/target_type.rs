use serde::{Deserialize, Serialize};
use std::fmt;

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
