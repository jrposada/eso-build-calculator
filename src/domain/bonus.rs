use crate::data::{BonusTarget, BonusTrigger};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BonusData {
    pub name: String,
    pub bonus_trigger: BonusTrigger,
    pub target: BonusTarget,
    pub value: f64,
    pub duration: Option<f64>,
    pub cooldown: Option<f64>,
}

impl BonusData {
    pub fn new(
        name: impl Into<String>,
        bonus_trigger: BonusTrigger,
        target: BonusTarget,
        value: f64,
    ) -> Self {
        BonusData {
            name: name.into(),
            bonus_trigger,
            target,
            value,
            duration: None,
            cooldown: None,
        }
    }

    pub fn with_trigger(mut self, trigger: BonusTrigger) -> Self {
        self.bonus_trigger = trigger;
        self
    }

    pub fn with_duration(mut self, duration: f64) -> Self {
        self.duration = Some(duration);
        self
    }

    pub fn with_cooldown(mut self, cooldown: f64) -> Self {
        self.cooldown = Some(cooldown);
        self
    }
}
