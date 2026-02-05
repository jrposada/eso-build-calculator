use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum StatusEffect {
    OffBalance,
    Chilled,
    Burning,
    Poisoned,
    Concussed,
}
