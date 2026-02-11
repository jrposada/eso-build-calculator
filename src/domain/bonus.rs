use crate::domain::bonus_value::BonusValue;

use super::{BonusSource, BonusTarget, BonusTrigger, SkillLineName, WeaponType};
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Default)]
pub struct ResolveContext {
    pub crit_damage: f64,
    pub main_hand_weapon: Option<WeaponType>,
    pub off_hand_weapon: Option<WeaponType>,
}

impl ResolveContext {
    pub fn new(crit_damage: f64) -> Self {
        Self {
            crit_damage,
            main_hand_weapon: None,
            off_hand_weapon: None,
        }
    }

    pub fn with_weapons(
        mut self,
        main_hand: Option<WeaponType>,
        off_hand: Option<WeaponType>,
    ) -> Self {
        self.main_hand_weapon = main_hand;
        self.off_hand_weapon = off_hand;
        self
    }

    /// Check if a trigger is active based on the current context
    pub fn is_trigger_active(&self, trigger: BonusTrigger) -> bool {
        match trigger {
            // General weapon triggers
            BonusTrigger::TwoHandedEquipped => {
                self.main_hand_weapon.map_or(false, |w| w.is_two_handed())
            }
            BonusTrigger::DualWieldEquipped => {
                self.main_hand_weapon.map_or(false, |w| w.is_dual_wield())
            }
            BonusTrigger::BowEquipped => self.main_hand_weapon.map_or(false, |w| w.is_bow()),
            BonusTrigger::DestructionStuffEquipped => self
                .main_hand_weapon
                .map_or(false, |w| w.is_destruction_staff()),

            // Specific two-handed weapon triggers
            BonusTrigger::TwoHandedSwordEquipped => self
                .main_hand_weapon
                .map_or(false, |w| w == WeaponType::TwoHandedSword),
            BonusTrigger::TwoHandedAxeEquipped => self
                .main_hand_weapon
                .map_or(false, |w| w == WeaponType::TwoHandedAxe),
            BonusTrigger::TwoHandedMaceEquipped => self
                .main_hand_weapon
                .map_or(false, |w| w == WeaponType::TwoHandedMace),

            // Specific dual wield weapon triggers (check both hands)
            BonusTrigger::DualWieldSwordEquipped => {
                self.main_hand_weapon
                    .map_or(false, |w| w == WeaponType::DualWieldSword)
                    || self
                        .off_hand_weapon
                        .map_or(false, |w| w == WeaponType::DualWieldSword)
            }
            BonusTrigger::DualWieldAxeEquipped => {
                self.main_hand_weapon
                    .map_or(false, |w| w == WeaponType::DualWieldAxe)
                    || self
                        .off_hand_weapon
                        .map_or(false, |w| w == WeaponType::DualWieldAxe)
            }
            BonusTrigger::DualWieldMaceEquipped => {
                self.main_hand_weapon
                    .map_or(false, |w| w == WeaponType::DualWieldMace)
                    || self
                        .off_hand_weapon
                        .map_or(false, |w| w == WeaponType::DualWieldMace)
            }
            BonusTrigger::DualWieldDaggerEquipped => {
                self.main_hand_weapon
                    .map_or(false, |w| w == WeaponType::DualWieldDagger)
                    || self
                        .off_hand_weapon
                        .map_or(false, |w| w == WeaponType::DualWieldDagger)
            }

            // Specific destruction staff triggers
            BonusTrigger::InfernoStaffEquipped => self
                .main_hand_weapon
                .map_or(false, |w| w == WeaponType::InfernoStaff),
            BonusTrigger::LightningStaffEquipped => self
                .main_hand_weapon
                .map_or(false, |w| w == WeaponType::LightningStaff),
            BonusTrigger::IceStaffEquipped => self
                .main_hand_weapon
                .map_or(false, |w| w == WeaponType::IceStaff),

            // Other triggers are always considered active (they depend on other conditions)
            _ => true,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BonusData {
    pub name: String,
    pub source: BonusSource,
    pub trigger: BonusTrigger,

    pub value: Vec<BonusValue>,

    pub cooldown: Option<f64>,
    pub duration: Option<f64>,
    pub execute_threshold: Option<f64>,
    pub skill_line_filter: Option<SkillLineName>,
}

impl BonusData {
    pub fn new(
        name: impl Into<String>,
        source: BonusSource,
        trigger: BonusTrigger,
        value: BonusValue,
    ) -> Self {
        BonusData {
            name: name.into(),
            source,
            trigger,
            value: vec![value],
            cooldown: None,
            duration: None,
            execute_threshold: None,
            skill_line_filter: None,
        }
    }

    pub fn with_trigger(mut self, trigger: BonusTrigger) -> Self {
        self.trigger = trigger;
        self
    }

    pub fn with_cooldown(mut self, cooldown: f64) -> Self {
        self.cooldown = Some(cooldown);
        self
    }

    pub fn with_duration(mut self, duration: f64) -> Self {
        self.duration = Some(duration);
        self
    }

    pub fn with_execute_threshold(mut self, threshold: f64) -> Self {
        self.execute_threshold = Some(threshold);
        self
    }

    pub fn with_skill_line_filter(mut self, skill_line: SkillLineName) -> Self {
        self.skill_line_filter = Some(skill_line);
        self
    }

    pub fn with_alternative(mut self, value: BonusValue) -> Self {
        self.value.push(value);
        self
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
}

#[cfg(test)]
mod tests {
    use super::*;

    const LONG_SHOTS_BREAKPOINT: f64 = 0.8338;

    fn create_long_shots_bonus() -> BonusData {
        BonusData::new(
            "Long Shots",
            BonusSource::Passive,
            BonusTrigger::SkillLineSlotted,
            BonusTarget::Damage,
            0.05,
        )
        .with_alternative(BonusTarget::CriticalRating, 1314.0, LONG_SHOTS_BREAKPOINT)
    }

    #[test]
    fn test_is_conditional() {
        let bonus = create_long_shots_bonus();
        assert!(bonus.is_conditional());

        let simple_bonus = BonusData::new(
            "Simple",
            BonusSource::Passive,
            BonusTrigger::Passive,
            BonusTarget::Damage,
            0.10,
        );
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
        assert_eq!(target, BonusTarget::CriticalRating);
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
            BonusSource::Passive,
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
        let bonus = BonusData::new(
            "Simple",
            BonusSource::Passive,
            BonusTrigger::Passive,
            BonusTarget::Damage,
            0.10,
        );
        let ctx = ResolveContext::new(0.80);

        assert!(bonus.selection_info(&ctx).is_none());
    }

    #[test]
    fn test_selection_info_returns_primary_below_breakpoint() {
        let bonus = create_long_shots_bonus();
        let ctx = ResolveContext::new(0.80);

        let info = bonus.selection_info(&ctx).unwrap();
        assert!(!info.used_alternative);
        assert!((info.crit_damage_breakpoint - LONG_SHOTS_BREAKPOINT).abs() < 0.0001);
    }

    #[test]
    fn test_selection_info_returns_alternative_above_breakpoint() {
        let bonus = create_long_shots_bonus();
        let ctx = ResolveContext::new(0.90);

        let info = bonus.selection_info(&ctx).unwrap();
        assert!(info.used_alternative);
        assert!((info.crit_damage_breakpoint - LONG_SHOTS_BREAKPOINT).abs() < 0.0001);
    }
}
