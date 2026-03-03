use crate::domain::{DamageFlags, LightAttackData, WeaponType};
use once_cell::sync::Lazy;

pub static BOW_LIGHT_ATTACKS: Lazy<Vec<LightAttackData>> = Lazy::new(|| {
    vec![
        LightAttackData::new(WeaponType::Bow, DamageFlags::physical_single(), 0.045, 0.4725),
    ]
});
