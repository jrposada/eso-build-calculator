use serde::{Deserialize, Serialize};

use crate::domain::BonusTarget;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BonusValue {
    pub name: String,
    pub target: BonusTarget,
    pub value: f64,
}

impl BonusValue {
    pub fn new(name: impl Into<String>, target: BonusTarget, value: f64) -> Self {
        Self {
            name: name.into(),
            target,
            value,
        }
    }
}
