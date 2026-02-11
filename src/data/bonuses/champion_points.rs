use crate::domain::{BonusData, BonusValue};
use crate::domain::{BonusSource, BonusTarget, BonusTrigger};
use once_cell::sync::Lazy;

pub static CHAMPION_POINTS: Lazy<Vec<BonusData>> = Lazy::new(|| {
    vec![
        BonusData::new(
            "Backstabber",
            BonusSource::ChampionPointSlottable,
            BonusTrigger::Passive,
            BonusValue::new("Backstabber", BonusTarget::CriticalDamage, 0.02 * 5.0),
        ),
        BonusData::new(
            "Biting Aura",
            BonusSource::ChampionPointSlottable,
            BonusTrigger::Passive,
            BonusValue::new("Biting Aura", BonusTarget::AoeDamage, 0.03 * 2.0),
        ),
        BonusData::new(
            "Deadly Aim",
            BonusSource::ChampionPointSlottable,
            BonusTrigger::Passive,
            BonusValue::new("Deadly Aim", BonusTarget::SingleDamage, 0.03 * 2.0),
        ),
        BonusData::new(
            "Master-at-Arms",
            BonusSource::ChampionPointSlottable,
            BonusTrigger::Passive,
            BonusValue::new("Master-at-Arms", BonusTarget::DirectDamage, 0.03 * 2.0),
        ),
        BonusData::new(
            "Exploiter",
            BonusSource::ChampionPointSlottable,
            BonusTrigger::Passive,
            BonusValue::new("Exploiter", BonusTarget::OffBalanceDamage, 0.02 * 5.0),
        ),
        BonusData::new(
            "Fighting Finesse",
            BonusSource::ChampionPointSlottable,
            BonusTrigger::Passive,
            BonusValue::new("Fighting Finesse", BonusTarget::CriticalDamage, 0.04 * 2.0),
        ),
        BonusData::new(
            "Thaumaturge",
            BonusSource::ChampionPointSlottable,
            BonusTrigger::Passive,
            BonusValue::new("Thaumaturge", BonusTarget::DotDamage, 0.03 * 2.0),
        ),
    ]
});
