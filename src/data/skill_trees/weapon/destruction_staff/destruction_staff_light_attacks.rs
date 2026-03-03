use crate::domain::{DamageFlags, LightAttackData, WeaponType};
use once_cell::sync::Lazy;

pub static DESTRUCTION_STAFF_LIGHT_ATTACKS: Lazy<Vec<LightAttackData>> = Lazy::new(|| {
    vec![
        LightAttackData::new(
            WeaponType::InfernoStaff,
            DamageFlags::flame_single(),
            0.045,
            0.4725,
        ),
        LightAttackData::new(
            WeaponType::LightningStaff,
            DamageFlags::shock_single(),
            0.045,
            0.4725,
        ),
        LightAttackData::new(
            WeaponType::IceStaff,
            DamageFlags::frost_single(),
            0.045,
            0.4725,
        ),
    ]
});
