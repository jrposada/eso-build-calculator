use crate::data::{BonusTarget, BonusType};
use crate::domain::BonusData;
use once_cell::sync::Lazy;

pub static CHAMPION_POINTS: Lazy<Vec<BonusData>> = Lazy::new(|| {
    vec![
        BonusData::new(
            "Backstabber",
            BonusType::Passive,
            BonusTarget::CriticalDamage,
            0.02 * 5.0,
        ),
        BonusData::new(
            "Biting Aura",
            BonusType::Passive,
            BonusTarget::AoeDamage,
            0.03 * 2.0,
        ),
        BonusData::new(
            "Deadly Aim",
            BonusType::Passive,
            BonusTarget::SingleDamage,
            0.03 * 2.0,
        ),
        BonusData::new(
            "Master-at-Arms",
            BonusType::Passive,
            BonusTarget::DirectDamage,
            0.03 * 2.0,
        ),
        BonusData::new(
            "Exploiter",
            BonusType::Passive,
            BonusTarget::OffBalanceDamage,
            0.02 * 5.0,
        ),
        BonusData::new(
            "Fighting Finesse",
            BonusType::Passive,
            BonusTarget::CriticalDamage,
            0.04 * 2.0,
        ),
        BonusData::new(
            "Thaumaturge",
            BonusType::Passive,
            BonusTarget::DotDamage,
            0.03 * 2.0,
        ),
    ]
});
