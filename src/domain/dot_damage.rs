use serde::{Deserialize, Serialize};

/// DoT (Damage over Time) damage data
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DotDamage {
    pub value: f64,
    pub duration: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delay: Option<f64>,
    /// Defaults to duration if not specified (total damage over duration)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<f64>,
    /// Percentage increase per tick (e.g., 0.12 for 12%)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub increase_per_tick: Option<f64>,
    /// Flat increase per tick
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flat_increase_per_tick: Option<f64>,
    /// If true, this damage ignores modifiers
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignores_modifier: Option<bool>,
}

impl DotDamage {
    pub fn new(value: f64, duration: f64) -> Self {
        Self {
            value,
            duration,
            delay: None,
            interval: None,
            increase_per_tick: None,
            flat_increase_per_tick: None,
            ignores_modifier: None,
        }
    }

    pub fn with_interval(mut self, interval: f64) -> Self {
        self.interval = Some(interval);
        self
    }

    pub fn with_increase_per_tick(mut self, increase: f64) -> Self {
        self.increase_per_tick = Some(increase);
        self
    }

    pub fn with_flat_increase_per_tick(mut self, increase: f64) -> Self {
        self.flat_increase_per_tick = Some(increase);
        self
    }

    pub fn with_delay(mut self, delay: f64) -> Self {
        self.delay = Some(delay);
        self
    }

    pub fn ignores_modifier(mut self) -> Self {
        self.ignores_modifier = Some(true);
        self
    }
}
