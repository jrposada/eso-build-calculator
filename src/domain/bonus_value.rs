use serde::{Deserialize, Serialize};

use crate::domain::{BonusTarget, SkillLineName};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BonusValue {
    #[serde(skip)]
    pub name: &'static str,
    pub target: BonusTarget,
    pub value: f64,
}

impl BonusValue {
    pub fn new(name: &'static str, target: BonusTarget, value: f64) -> Self {
        Self {
            name,
            target,
            value,
        }
    }
}

/// Lightweight, fully Copy bonus representation for the optimizer fast path.
/// Avoids heap allocations (no String fields) while carrying all data needed
/// for damage calculation.
#[derive(Debug, Clone, Copy)]
pub struct ResolvedBonus {
    pub target: BonusTarget,
    pub value: f64,
    pub skill_line_filter: Option<SkillLineName>,
    pub execute_threshold: Option<f64>,
}
