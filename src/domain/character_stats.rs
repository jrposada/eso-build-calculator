use serde::{Deserialize, Serialize};

/// Character stats used for coefficient-based damage calculation.
///
/// These represent the offensive stats that affect damage output in ESO.
/// Since Update 33+, ESO uses dynamic scaling where skills use the higher
/// of magicka/stamina and weapon/spell damage.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CharacterStats {
    /// Maximum magicka pool
    pub max_magicka: f64,
    /// Maximum stamina pool
    pub max_stamina: f64,
    /// Weapon damage rating
    pub weapon_damage: f64,
    /// Spell damage rating
    pub spell_damage: f64,
    /// Critical strike chance (0.0 - 1.0, e.g., 0.60 for 60%)
    pub critical_chance: f64,
    /// Critical damage multiplier (e.g., 1.75 for 75% bonus damage)
    pub critical_damage: f64,
    /// Armor penetration rating
    pub penetration: f64,
    /// Target's armor value
    pub target_armor: f64,
}

impl Default for CharacterStats {
    /// Default values representing typical endgame stats
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
    /// Create new CharacterStats with all values specified
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

    /// Get the higher of max_magicka and max_stamina (ESO Update 33+ scaling)
    pub fn max_stat(&self) -> f64 {
        self.max_magicka.max(self.max_stamina)
    }

    /// Get the higher of weapon_damage and spell_damage (ESO Update 33+ scaling)
    pub fn max_power(&self) -> f64 {
        self.weapon_damage.max(self.spell_damage)
    }

    // Builder pattern methods

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
}
