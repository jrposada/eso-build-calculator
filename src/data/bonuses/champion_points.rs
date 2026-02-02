use crate::data::{BonusTarget, BonusTrigger};
use crate::domain::BonusData;
use once_cell::sync::Lazy;

pub static CHAMPION_POINTS: Lazy<Vec<BonusData>> = Lazy::new(|| {
    vec![
        BonusData::new(
            "Backstabber",
            BonusTrigger::Passive,
            BonusTarget::CriticalDamage,
            0.02 * 5.0,
        ),
        BonusData::new(
            "Biting Aura",
            BonusTrigger::Passive,
            BonusTarget::AoeDamage,
            0.03 * 2.0,
        ),
        BonusData::new(
            "Deadly Aim",
            BonusTrigger::Passive,
            BonusTarget::SingleDamage,
            0.03 * 2.0,
        ),
        BonusData::new(
            "Master-at-Arms",
            BonusTrigger::Passive,
            BonusTarget::DirectDamage,
            0.03 * 2.0,
        ),
        BonusData::new(
            "Exploiter",
            BonusTrigger::Passive,
            BonusTarget::OffBalanceDamage,
            0.02 * 5.0,
        ),
        BonusData::new(
            "Fighting Finesse",
            BonusTrigger::Passive,
            BonusTarget::CriticalDamage,
            0.04 * 2.0,
        ),
        BonusData::new(
            "Thaumaturge",
            BonusTrigger::Passive,
            BonusTarget::DotDamage,
            0.03 * 2.0,
        ),
    ]
});
