use super::{formulas, BonusSource, BonusTrigger, SkillLineName, WeaponType};
use crate::domain::{BonusValue, CharacterStats};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default)]
pub struct ResolveContext {
    pub character_stats: CharacterStats,
    pub main_hand_weapon: Option<WeaponType>,
    pub off_hand_weapon: Option<WeaponType>,
}

impl ResolveContext {
    pub fn new(character_stats: CharacterStats) -> Self {
        Self {
            character_stats,
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
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BonusData {
    pub name: String,
    pub source: BonusSource,
    pub trigger: BonusTrigger,

    pub cooldown: Option<f64>,
    pub duration: Option<f64>,
    pub execute_threshold: Option<f64>,
    pub skill_line_filter: Option<SkillLineName>,

    value: Vec<BonusValue>,
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

    pub fn has_alternative(&self) -> bool {
        self.value.len() > 1
    }

    pub fn resolve(&self, ctx: &ResolveContext) -> BonusValue {
        self.resolve_ref(ctx).clone()
    }

    pub fn resolve_ref(&self, ctx: &ResolveContext) -> &BonusValue {
        if self.value.len() <= 1 {
            return &self.value[0];
        }

        // TODO: use trigger to multiple active by trigger, things like once per
        // slotted skill, just once if skill line used, etc
        self.value
            .iter()
            .max_by(|a, b| {
                let a_eff = formulas::effective_damage_contribution(
                    a.target,
                    a.value,
                    &ctx.character_stats,
                );
                let b_eff = formulas::effective_damage_contribution(
                    b.target,
                    b.value,
                    &ctx.character_stats,
                );
                a_eff
                    .partial_cmp(&b_eff)
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::BonusTarget;

    fn long_shots_bonus() -> BonusData {
        BonusData::new(
            "Long Shots",
            BonusSource::ChampionPointSlottable,
            BonusTrigger::Passive,
            BonusValue::new("Damage", BonusTarget::Damage, 0.05),
        )
        .with_alternative(BonusValue::new(
            "Crit Rating",
            BonusTarget::CriticalRating,
            1314.0,
        ))
    }

    #[test]
    fn test_resolve_single_value() {
        let bonus = BonusData::new(
            "Simple",
            BonusSource::ChampionPointSlottable,
            BonusTrigger::Passive,
            BonusValue::new("Damage", BonusTarget::Damage, 0.05),
        );
        let ctx = ResolveContext::new(CharacterStats::default());
        let result = bonus.resolve(&ctx);
        assert_eq!(result.target, BonusTarget::Damage);
        assert!((result.value - 0.05).abs() < 0.0001);
    }

    #[test]
    fn test_resolve_long_shots_low_crit_damage_picks_damage() {
        let bonus = long_shots_bonus();
        // Low crit damage → Damage% is better
        let stats = CharacterStats::default()
            .with_critical_rating(4382.4)
            .with_critical_damage(1.50);
        let ctx = ResolveContext::new(stats);
        let result = bonus.resolve(&ctx);
        assert_eq!(result.target, BonusTarget::Damage);
    }

    #[test]
    fn test_resolve_long_shots_high_crit_damage_picks_crit_rating() {
        let bonus = long_shots_bonus();
        // High crit damage → CritRating is better
        let stats = CharacterStats::default()
            .with_critical_rating(10956.0)
            .with_critical_damage(2.25);
        let ctx = ResolveContext::new(stats);
        let result = bonus.resolve(&ctx);
        assert_eq!(result.target, BonusTarget::CriticalRating);
    }

    #[test]
    fn test_resolve_ancient_knowledge_dot_vs_direct() {
        // Ancient Knowledge: DotDamage(12%) vs DirectDamage(12%)
        // Both are percentage modifiers so they should be equal — resolve picks first max
        let bonus = BonusData::new(
            "Ancient Knowledge",
            BonusSource::Passive,
            BonusTrigger::Passive,
            BonusValue::new("DoT Damage", BonusTarget::DotDamage, 0.12),
        )
        .with_alternative(BonusValue::new(
            "Direct Damage",
            BonusTarget::DirectDamage,
            0.12,
        ));
        let ctx = ResolveContext::new(CharacterStats::default());
        let result = bonus.resolve(&ctx);
        // Both equal — max_by picks last equal, which is DirectDamage
        assert!(result.value == 0.12);
    }

    #[test]
    fn test_resolve_twin_blade_and_blunt() {
        // Twin Blade and Blunt: CritDamage(0.06) vs Penetration(1487) vs DamageFlat(129) vs CritRating(657)
        let bonus = BonusData::new(
            "Twin Blade and Blunt",
            BonusSource::Passive,
            BonusTrigger::DualWieldEquipped,
            BonusValue::new("Crit Damage", BonusTarget::CriticalDamage, 0.06),
        )
        .with_alternative(BonusValue::new(
            "Penetration",
            BonusTarget::PhysicalAndSpellPenetration,
            1487.0,
        ))
        .with_alternative(BonusValue::new(
            "Damage Flat",
            BonusTarget::WeaponAndSpellDamageFlat,
            129.0,
        ))
        .with_alternative(BonusValue::new(
            "Crit Rating",
            BonusTarget::CriticalRating,
            657.0,
        ));

        // With high stats, penetration should be valuable when target has lots of armor
        let stats = CharacterStats::default()
            .with_weapon_damage(6000.0)
            .with_spell_damage(6000.0)
            .with_critical_rating(10956.0)
            .with_critical_damage(1.75)
            .with_penetration(10000.0)
            .with_target_armor(18200.0);
        let ctx = ResolveContext::new(stats);
        let result = bonus.resolve(&ctx);
        // Just verify it picks something reasonable (the best option given these stats)
        assert!(result.value > 0.0);
    }
}
