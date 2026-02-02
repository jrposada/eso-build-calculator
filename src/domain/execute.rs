use serde::{Deserialize, Serialize};

/// How execute damage scales with enemy health
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ExecuteScaling {
    /// Full bonus when below threshold
    Flat,
    /// Scales linearly with missing health
    Linear,
}

/// Execute damage data for skills that deal bonus damage at low enemy health
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExecuteData {
    /// Damage multiplier (e.g., 3.0 for 300% bonus)
    pub multiplier: f64,
    /// Health threshold (e.g., 0.25 for 25% HP)
    pub threshold: f64,
    /// How the bonus scales
    pub scaling: ExecuteScaling,
}

impl ExecuteData {
    pub fn new(multiplier: f64, threshold: f64, scaling: ExecuteScaling) -> Self {
        Self {
            multiplier,
            threshold,
            scaling,
        }
    }

    /// Calculate the damage multiplier based on enemy health percentage
    ///
    /// Returns 1.0 if enemy is above threshold, otherwise returns the execute multiplier
    pub fn calculate_multiplier(&self, enemy_health_percent: f64) -> f64 {
        if enemy_health_percent >= self.threshold {
            return 1.0;
        }
        match self.scaling {
            ExecuteScaling::Flat => 1.0 + self.multiplier,
            ExecuteScaling::Linear => {
                let progress = 1.0 - (enemy_health_percent / self.threshold);
                1.0 + (self.multiplier * progress)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flat_execute_above_threshold() {
        let execute = ExecuteData::new(3.0, 0.25, ExecuteScaling::Flat);
        assert_eq!(execute.calculate_multiplier(0.50), 1.0);
        assert_eq!(execute.calculate_multiplier(0.25), 1.0);
    }

    #[test]
    fn test_flat_execute_below_threshold() {
        let execute = ExecuteData::new(3.0, 0.25, ExecuteScaling::Flat);
        assert_eq!(execute.calculate_multiplier(0.20), 4.0);
        assert_eq!(execute.calculate_multiplier(0.10), 4.0);
        assert_eq!(execute.calculate_multiplier(0.0), 4.0);
    }

    #[test]
    fn test_linear_execute_above_threshold() {
        let execute = ExecuteData::new(4.0, 0.50, ExecuteScaling::Linear);
        assert_eq!(execute.calculate_multiplier(0.75), 1.0);
        assert_eq!(execute.calculate_multiplier(0.50), 1.0);
    }

    #[test]
    fn test_linear_execute_at_thresholds() {
        let execute = ExecuteData::new(4.0, 0.50, ExecuteScaling::Linear);
        // At 25% HP: progress = 1 - (0.25/0.50) = 0.5, multiplier = 1 + 4.0*0.5 = 3.0
        assert_eq!(execute.calculate_multiplier(0.25), 3.0);
        // At 0% HP: progress = 1 - (0/0.50) = 1.0, multiplier = 1 + 4.0*1.0 = 5.0
        assert_eq!(execute.calculate_multiplier(0.0), 5.0);
    }
}
