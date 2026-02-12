use serde::{Deserialize, Serialize};

use super::formulas;

pub const ATTRIBUTE_POINTS_BONUS: f64 = 111.0 * 64.0;
pub const MAX_CRITICAL_CHANCE: f64 = 1.0;
pub const MAX_CRITICAL_DAMAGE: f64 = 2.25;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CharacterStats {
    pub max_magicka: f64,
    pub max_stamina: f64,
    pub weapon_damage: f64,
    pub spell_damage: f64,
    pub critical_rating: f64,
    pub critical_damage: f64,
    pub penetration: f64,
    pub target_armor: f64,
}

impl Default for CharacterStats {
    fn default() -> Self {
        Self {
            max_magicka: 12_000.0,
            max_stamina: 12_000.0,
            weapon_damage: 1_000.0,
            spell_damage: 1_000.0,
            critical_rating: 0.0, // FIXME: @javi
            critical_damage: 1.50,
            penetration: 0.0,
            target_armor: 18_200.0,
        }
    }
}

impl CharacterStats {
    pub fn new(
        max_magicka: f64,
        max_stamina: f64,
        weapon_damage: f64,
        spell_damage: f64,
        critical_rating: f64,
        critical_damage: f64,
        penetration: f64,
        target_armor: f64,
    ) -> Self {
        Self {
            max_magicka,
            max_stamina,
            weapon_damage,
            spell_damage,
            critical_rating,
            critical_damage,
            penetration,
            target_armor,
        }
    }

    pub fn with_max_magicka(mut self, value: f64) -> Self {
        self.max_magicka = value;
        self
    }

    pub fn with_max_stamina(mut self, value: f64) -> Self {
        self.max_stamina = value;
        self
    }

    pub fn with_weapon_damage(mut self, value: f64) -> Self {
        self.weapon_damage = value;
        self
    }

    pub fn with_spell_damage(mut self, value: f64) -> Self {
        self.spell_damage = value;
        self
    }

    pub fn with_critical_rating(mut self, value: f64) -> Self {
        self.critical_rating = value;
        self
    }

    pub fn with_critical_damage(mut self, value: f64) -> Self {
        self.critical_damage = value;
        self
    }

    pub fn with_penetration(mut self, value: f64) -> Self {
        self.penetration = value;
        self
    }

    pub fn with_target_armor(mut self, value: f64) -> Self {
        self.target_armor = value;
        self
    }

    pub fn max_stat(&self) -> f64 {
        self.max_magicka.max(self.max_stamina)
    }

    pub fn max_power(&self) -> f64 {
        self.weapon_damage.max(self.spell_damage)
    }

    pub fn critical_chance(&self) -> f64 {
        formulas::crit_rating_to_chance(self.critical_rating)
    }

    pub fn clamp_caps(&mut self) {
        self.critical_damage = self.critical_damage.min(MAX_CRITICAL_DAMAGE);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_stat_magicka_higher() {
        let stats = CharacterStats::default().with_max_magicka(50_000.0);
        assert_eq!(stats.max_stat(), 50_000.0);
    }

    #[test]
    fn test_max_stat_stamina_higher() {
        let stats = CharacterStats::default().with_max_stamina(50_000.0);
        assert_eq!(stats.max_stat(), 50_000.0);
    }

    #[test]
    fn test_max_power_weapon_higher() {
        let stats = CharacterStats::default().with_weapon_damage(6_000.0);
        assert_eq!(stats.max_power(), 6_000.0);
    }

    #[test]
    fn test_max_power_spell_higher() {
        let stats = CharacterStats::default().with_spell_damage(6_000.0);
        assert_eq!(stats.max_power(), 6_000.0);
    }

    #[test]
    fn test_builder_chain() {
        let stats = CharacterStats::default()
            .with_max_magicka(45_000.0)
            .with_spell_damage(6_000.0)
            .with_critical_rating(3000.0);

        assert_eq!(stats.max_magicka, 45_000.0);
        assert_eq!(stats.spell_damage, 6_000.0);
        assert_eq!(stats.critical_rating, 3000.0);
    }

    #[test]
    fn test_critical_chance_from_rating() {
        let stats = CharacterStats::default().with_critical_rating(3000.0);
        // crit_rating_to_chance(3000) ≈ 0.237 (10% base + 13.7% from rating)
        assert!((stats.critical_chance() - 0.237).abs() < 0.001);
    }

    #[test]
    fn test_critical_chance_caps_at_100_percent() {
        let stats = CharacterStats::default().with_critical_rating(30000.0);
        assert!((stats.critical_chance() - 1.0).abs() < 0.0001);
    }

    #[test]
    fn test_critical_chance_base_with_zero_rating() {
        let stats = CharacterStats::default();
        assert!((stats.critical_chance() - 0.10).abs() < 0.0001);
    }

    #[test]
    fn test_clamp_caps_above_cap() {
        let mut stats = CharacterStats::default().with_critical_damage(3.0);
        stats.clamp_caps();
        assert_eq!(stats.critical_damage, 2.25);
    }

    #[test]
    fn test_clamp_caps_below_cap() {
        let mut stats = CharacterStats::default().with_critical_damage(1.75);
        stats.clamp_caps();
        assert_eq!(stats.critical_damage, 1.75);
    }

    #[test]
    fn test_clamp_caps_at_exact_cap() {
        let mut stats = CharacterStats::default().with_critical_damage(2.25);
        stats.clamp_caps();
        assert_eq!(stats.critical_damage, 2.25);
    }
}
