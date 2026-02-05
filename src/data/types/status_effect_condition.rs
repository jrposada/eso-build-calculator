use serde::{Deserialize, Serialize};

// TODO: needed?
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum StatusEffectCondition {
    Flanking,
    FromDistance,
    Always,
}
