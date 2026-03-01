use crate::domain::{DamageFlags, LightAttackData, WeaponType};
use std::sync::LazyLock;

/// Light attack data per weapon type.
/// Coefficients from UESP ESO Skill Coefficients datamine.
/// Melee weapons (Two-Handed, Dual Wield): coef_a=0.05, coef_b=0.525
/// Ranged/Staff weapons (Bow, Destruction Staves): coef_a=0.045, coef_b=0.4725
static LIGHT_ATTACKS: LazyLock<Vec<LightAttackData>> = LazyLock::new(|| {
    vec![
        // Two-Handed (physical)
        LightAttackData::new(WeaponType::TwoHandedSword, DamageFlags::physical_single(), 0.05, 0.525),
        LightAttackData::new(WeaponType::TwoHandedAxe, DamageFlags::physical_single(), 0.05, 0.525),
        LightAttackData::new(WeaponType::TwoHandedMace, DamageFlags::physical_single(), 0.05, 0.525),
        // Dual Wield (physical)
        LightAttackData::new(WeaponType::DualWieldSword, DamageFlags::physical_single(), 0.05, 0.525),
        LightAttackData::new(WeaponType::DualWieldAxe, DamageFlags::physical_single(), 0.05, 0.525),
        LightAttackData::new(WeaponType::DualWieldMace, DamageFlags::physical_single(), 0.05, 0.525),
        LightAttackData::new(WeaponType::DualWieldDagger, DamageFlags::physical_single(), 0.05, 0.525),
        // Bow (physical)
        LightAttackData::new(WeaponType::Bow, DamageFlags::physical_single(), 0.045, 0.4725),
        // Destruction Staves (elemental)
        LightAttackData::new(WeaponType::InfernoStaff, DamageFlags::flame_single(), 0.045, 0.4725),
        LightAttackData::new(WeaponType::LightningStaff, DamageFlags::shock_single(), 0.045, 0.4725),
        LightAttackData::new(WeaponType::IceStaff, DamageFlags::frost_single(), 0.045, 0.4725),
    ]
});

pub fn light_attack_for_weapon(weapon_type: WeaponType) -> &'static LightAttackData {
    LIGHT_ATTACKS
        .iter()
        .find(|la| la.weapon_type == weapon_type)
        .expect("Light attack data should exist for all weapon types")
}
