use crate::domain::{DamageFlags, LightAttackData, WeaponType};
use once_cell::sync::Lazy;

pub static DUAL_WIELD_LIGHT_ATTACKS: Lazy<Vec<LightAttackData>> = Lazy::new(|| {
    vec![
        LightAttackData::new(WeaponType::DualWieldSword, DamageFlags::physical_single(), 0.05, 0.525),
        LightAttackData::new(WeaponType::DualWieldAxe, DamageFlags::physical_single(), 0.05, 0.525),
        LightAttackData::new(WeaponType::DualWieldMace, DamageFlags::physical_single(), 0.05, 0.525),
        LightAttackData::new(WeaponType::DualWieldDagger, DamageFlags::physical_single(), 0.05, 0.525),
    ]
});
