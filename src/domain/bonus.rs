use crate::data::{BonusTarget, BonusTrigger};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default)]
pub struct ResolveContext {
    pub crit_damage: f64,
}

impl ResolveContext {
    pub fn new(crit_damage: f64) -> Self {
        Self { crit_damage }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BonusAlternative {
    pub target: BonusTarget,
    pub value: f64,
    pub crit_damage_breakpoint: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BonusData {
    pub name: String,
    pub bonus_trigger: BonusTrigger,
    pub target: BonusTarget,
    pub value: f64,
    pub duration: Option<f64>,
    pub cooldown: Option<f64>,
    #[serde(default)]
    alternative: Option<BonusAlternative>,
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
            alternative: None,
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

    /// Add an alternative bonus option with a pre-calculated crit damage breakpoint.
    /// When crit damage is above the breakpoint, the alternative is used instead.
    pub fn with_alternative(
        mut self,
        target: BonusTarget,
        value: f64,
        crit_damage_breakpoint: f64,
    ) -> Self {
        self.alternative = Some(BonusAlternative {
            target,
            value,
            crit_damage_breakpoint,
        });
        self
    }

    pub fn is_conditional(&self) -> bool {
        self.alternative.is_some()
    }

    /// Resolve the bonus based on build context.
    /// For non-conditional bonuses, returns primary target/value immediately.
    /// For conditional bonuses, selects based on context (e.g., crit damage threshold).
    pub fn resolve(&self, ctx: &ResolveContext) -> (BonusTarget, f64) {
        if let Some(alt) = &self.alternative {
            if ctx.crit_damage > alt.crit_damage_breakpoint {
                return (alt.target, alt.value);
            }
        }
        (self.target, self.value)
    }

    /// Returns selection info for conditional bonuses, None for non-conditional.
    /// Returns (used_alternative, crit_damage_breakpoint).
    pub fn selection_info(&self, ctx: &ResolveContext) -> Option<(bool, f64)> {
        self.alternative.as_ref().map(|alt| {
            let used_alternative = ctx.crit_damage > alt.crit_damage_breakpoint;
            (used_alternative, alt.crit_damage_breakpoint)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const LONG_SHOTS_BREAKPOINT: f64 = 0.8338;

    fn create_long_shots_bonus() -> BonusData {
        BonusData::new(
            "Long Shots",
            BonusTrigger::SkillLineSlotted,
            BonusTarget::Damage,
            0.05,
        )
        .with_alternative(BonusTarget::CriticalChance, 1314.0, LONG_SHOTS_BREAKPOINT)
    }

    #[test]
    fn test_is_conditional() {
        let bonus = create_long_shots_bonus();
        assert!(bonus.is_conditional());

        let simple_bonus =
            BonusData::new("Simple", BonusTrigger::Passive, BonusTarget::Damage, 0.10);
        assert!(!simple_bonus.is_conditional());
    }

    #[test]
    fn test_resolve_uses_primary_below_breakpoint() {
        let bonus = create_long_shots_bonus();
        let ctx = ResolveContext::new(0.80);

        // Below breakpoint (80% crit damage), use primary (flat damage)
        let (target, value) = bonus.resolve(&ctx);
        assert_eq!(target, BonusTarget::Damage);
        assert!((value - 0.05).abs() < 0.0001);
    }

    #[test]
    fn test_resolve_uses_alternative_above_breakpoint() {
        let bonus = create_long_shots_bonus();
        let ctx = ResolveContext::new(0.90);

        // Above breakpoint (90% crit damage), use alternative (crit rating)
        let (target, value) = bonus.resolve(&ctx);
        assert_eq!(target, BonusTarget::CriticalChance);
        assert!((value - 1314.0).abs() < 0.0001);
    }

    #[test]
    fn test_resolve_at_breakpoint_uses_primary() {
        let bonus = create_long_shots_bonus();
        let ctx = ResolveContext::new(LONG_SHOTS_BREAKPOINT);

        // At exactly breakpoint, use primary (not strictly above)
        let (target, value) = bonus.resolve(&ctx);
        assert_eq!(target, BonusTarget::Damage);
        assert!((value - 0.05).abs() < 0.0001);
    }

    #[test]
    fn test_resolve_non_conditional_always_returns_primary() {
        let bonus = BonusData::new(
            "Simple",
            BonusTrigger::Passive,
            BonusTarget::CriticalDamage,
            0.10,
        );

        let ctx_low = ResolveContext::new(0.50);
        let (target, value) = bonus.resolve(&ctx_low);
        assert_eq!(target, BonusTarget::CriticalDamage);
        assert!((value - 0.10).abs() < 0.0001);

        let ctx_high = ResolveContext::new(0.99);
        let (target, value) = bonus.resolve(&ctx_high);
        assert_eq!(target, BonusTarget::CriticalDamage);
        assert!((value - 0.10).abs() < 0.0001);
    }

    #[test]
    fn test_selection_info_returns_none_for_non_conditional() {
        let bonus = BonusData::new("Simple", BonusTrigger::Passive, BonusTarget::Damage, 0.10);
        let ctx = ResolveContext::new(0.80);

        assert!(bonus.selection_info(&ctx).is_none());
    }

    #[test]
    fn test_selection_info_returns_primary_below_breakpoint() {
        let bonus = create_long_shots_bonus();
        let ctx = ResolveContext::new(0.80);

        let (used_alt, breakpoint) = bonus.selection_info(&ctx).unwrap();
        assert!(!used_alt);
        assert!((breakpoint - LONG_SHOTS_BREAKPOINT).abs() < 0.0001);
    }

    #[test]
    fn test_selection_info_returns_alternative_above_breakpoint() {
        let bonus = create_long_shots_bonus();
        let ctx = ResolveContext::new(0.90);

        let (used_alt, breakpoint) = bonus.selection_info(&ctx).unwrap();
        assert!(used_alt);
        assert!((breakpoint - LONG_SHOTS_BREAKPOINT).abs() < 0.0001);
    }
}
