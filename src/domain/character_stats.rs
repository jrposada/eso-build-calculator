use super::formulas;
use serde::{Deserialize, Serialize};

pub const ATTRIBUTE_POINTS_BONUS: f64 = 111.0 * 64.0;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CharacterStats {
    pub max_magicka: f64,
    pub max_stamina: f64,
    pub weapon_damage: f64,
    pub spell_damage: f64,
    pub critical_chance: f64,
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
            critical_chance: 0.10,
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
        critical_chance: f64,
        critical_damage: f64,
        penetration: f64,
        target_armor: f64,
    ) -> Self {
        Self {
            max_magicka,
            max_stamina,
            weapon_damage,
            spell_damage,
            critical_chance,
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

    pub fn with_critical_chance(mut self, value: f64) -> Self {
        self.critical_chance = value;
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

    pub fn clamp_caps(&mut self) {
        self.critical_chance = self.critical_chance.min(formulas::MAX_CRITICAL_CHANCE);
        self.critical_damage = self.critical_damage.min(formulas::MAX_CRITICAL_DAMAGE);
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
            .with_critical_chance(0.70);

        assert_eq!(stats.max_magicka, 45_000.0);
        assert_eq!(stats.spell_damage, 6_000.0);
        assert_eq!(stats.critical_chance, 0.70);
    }

    #[test]
    fn test_clamp_caps_above_cap() {
        let mut stats = CharacterStats::default()
            .with_critical_chance(1.5)
            .with_critical_damage(3.0);
        stats.clamp_caps();
        assert_eq!(stats.critical_chance, 1.0);
        assert_eq!(stats.critical_damage, 2.25);
    }

    #[test]
    fn test_clamp_caps_below_cap() {
        let mut stats = CharacterStats::default()
            .with_critical_chance(0.5)
            .with_critical_damage(1.75);
        stats.clamp_caps();
        assert_eq!(stats.critical_chance, 0.5);
        assert_eq!(stats.critical_damage, 1.75);
    }

    #[test]
    fn test_clamp_caps_at_exact_cap() {
        let mut stats = CharacterStats::default()
            .with_critical_chance(1.0)
            .with_critical_damage(2.25);
        stats.clamp_caps();
        assert_eq!(stats.critical_chance, 1.0);
        assert_eq!(stats.critical_damage, 2.25);
    }
}
