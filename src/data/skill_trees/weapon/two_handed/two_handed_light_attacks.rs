use crate::domain::{DamageFlags, LightAttackData, WeaponType};
use once_cell::sync::Lazy;

pub static TWO_HANDED_LIGHT_ATTACKS: Lazy<Vec<LightAttackData>> = Lazy::new(|| {
    vec![
        LightAttackData::new(
            WeaponType::TwoHandedSword,
            DamageFlags::physical_single(),
            0.05,
            0.525,
        ),
        LightAttackData::new(
            WeaponType::TwoHandedAxe,
            DamageFlags::physical_single(),
            0.05,
            0.525,
        ),
        LightAttackData::new(
            WeaponType::TwoHandedMace,
            DamageFlags::physical_single(),
            0.05,
            0.525,
        ),
    ]
});
