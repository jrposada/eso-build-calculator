use crate::data::{BonusTarget, BonusType};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BonusData {
    pub name: String,
    pub bonus_type: BonusType,
    pub target: BonusTarget,
    pub value: f64,
}

impl BonusData {
    pub fn new(
        name: impl Into<String>,
        bonus_type: BonusType,
        target: BonusTarget,
        value: f64,
    ) -> Self {
        BonusData {
            name: name.into(),
            bonus_type,
            target,
            value,
        }
    }
}
