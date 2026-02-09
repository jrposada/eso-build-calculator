use crate::domain::{formulas, BonusTarget};

/// Service for breakpoint calculations
pub struct BreakpointsService;

impl BreakpointsService {
    pub fn calculate_breakpoint(
        primary_target: BonusTarget,
        primary_value: f64,
        alt_target: BonusTarget,
        alt_value: f64,
    ) -> Option<f64> {
        match (primary_target, alt_target) {
            (BonusTarget::Damage, BonusTarget::CriticalChance) => {
                let crit_increase = formulas::crit_rating_to_bonus_chance(alt_value);
                if crit_increase > 0.0 {
                    Some(primary_value / crit_increase)
                } else {
                    None
                }
            }
            (BonusTarget::CriticalChance, BonusTarget::Damage) => {
                let crit_increase = formulas::crit_rating_to_bonus_chance(primary_value);
                if crit_increase > 0.0 {
                    Some(alt_value / crit_increase)
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_breakpoint_flat_vs_crit_rating() {
        let breakpoint = BreakpointsService::calculate_breakpoint(
            BonusTarget::Damage,
            0.05,
            BonusTarget::CriticalChance,
            1314.0,
        );

        assert!(breakpoint.is_some());
        let bp = breakpoint.unwrap();
        let expected = 0.05 / (1314.0 / formulas::MAX_CRIT_VALUE_CP160);
        assert!(
            (bp - expected).abs() < 0.001,
            "Expected breakpoint ~{}, got {}",
            expected,
            bp
        );
    }

    #[test]
    fn test_breakpoint_crit_rating_vs_flat() {
        let breakpoint = BreakpointsService::calculate_breakpoint(
            BonusTarget::CriticalChance,
            1314.0,
            BonusTarget::Damage,
            0.05,
        );

        assert!(breakpoint.is_some());
        let bp = breakpoint.unwrap();
        let expected = 0.05 / (1314.0 / formulas::MAX_CRIT_VALUE_CP160);
        assert!(
            (bp - expected).abs() < 0.001,
            "Expected breakpoint ~{}, got {}",
            expected,
            bp
        );
    }

    #[test]
    fn test_breakpoint_unsupported_types() {
        let breakpoint = BreakpointsService::calculate_breakpoint(
            BonusTarget::CriticalDamage,
            0.10,
            BonusTarget::Damage,
            0.05,
        );

        assert!(breakpoint.is_none());
    }
}
