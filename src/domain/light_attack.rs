use super::{DamageCoefficients, DamageFlags, WeaponType};

#[derive(Debug, Clone)]
pub struct LightAttackData {
    pub weapon_type: WeaponType,
    pub flags: DamageFlags,
    pub coefficients: DamageCoefficients,
}

impl LightAttackData {
    pub fn new(weapon_type: WeaponType, flags: DamageFlags, coef_a: f64, coef_b: f64) -> Self {
        Self {
            weapon_type,
            flags: flags | DamageFlags::DIRECT | DamageFlags::SINGLE_TARGET,
            coefficients: DamageCoefficients::new(coef_a, coef_b),
        }
    }

    pub fn calculate_damage(
        &self,
        modifier_sum: f64,
        max_stat: f64,
        max_power: f64,
        armor_factor: f64,
        crit_mult: f64,
    ) -> f64 {
        let base = self.coefficients.calculate_base_damage(max_stat, max_power);
        base * (1.0 + modifier_sum) * armor_factor * crit_mult
    }
}
