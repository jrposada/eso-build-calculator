use serde::{Deserialize, Serialize};

/// Hit damage data
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HitDamage {
    pub value: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delay: Option<f64>,
}

impl HitDamage {
    pub fn new(value: f64) -> Self {
        Self { value, delay: None }
    }

    pub fn with_delay(mut self, delay: f64) -> Self {
        self.delay = Some(delay);
        self
    }
}
