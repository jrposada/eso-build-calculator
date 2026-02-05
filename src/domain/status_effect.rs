use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum StatusEffect {
    OffBalance,
    Chilled,
    Burning,
    Poisoned,
    Concussed,
}

impl fmt::Display for StatusEffect {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StatusEffect::OffBalance => write!(f, "Off Balance"),
            StatusEffect::Chilled => write!(f, "Chilled"),
            StatusEffect::Burning => write!(f, "Burning"),
            StatusEffect::Poisoned => write!(f, "Poisoned"),
            StatusEffect::Concussed => write!(f, "Concussed"),
        }
    }
}

// /// Represents a status effect that a skill can apply
// #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
// pub struct StatusEffectApplication {
//     pub effect: StatusEffect,
//     /// Duration of the status effect in seconds (None for permanent until cleansed)
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub duration: Option<f64>,
//     /// Condition under which the effect is applied
//     #[serde(default = "default_condition")]
//     pub condition: StatusEffectCondition,
// }

// fn default_condition() -> StatusEffectCondition {
//     StatusEffectCondition::Always
// }

// impl StatusEffectApplication {
//     pub fn new(effect: StatusEffect) -> Self {
//         Self {
//             effect,
//             duration: None,
//             condition: StatusEffectCondition::Always,
//         }
//     }

//     pub fn with_duration(mut self, duration: f64) -> Self {
//         self.duration = Some(duration);
//         self
//     }

//     pub fn with_condition(mut self, condition: StatusEffectCondition) -> Self {
//         self.condition = condition;
//         self
//     }
// }
