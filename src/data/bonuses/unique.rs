use crate::domain::{BonusData, BonusValue};
use crate::domain::{BonusSource, BonusTarget, BonusTrigger};
use once_cell::sync::Lazy;

pub static MINOR_SAVAGERY: Lazy<BonusData> = Lazy::new(|| {
    BonusData::new(
        "Minor Savagery",
        BonusSource::Buff,
        BonusTrigger::Cast,
        BonusValue::new("Minor Savagery", BonusTarget::WeaponCriticalRating, 1314.0),
    )
    .with_duration(20.0)
});

pub static MINOR_BERSERK: Lazy<BonusData> = Lazy::new(|| {
    BonusData::new(
        "Minor Berserk",
        BonusSource::Buff,
        BonusTrigger::Cast,
        BonusValue::new("Minor Berserk", BonusTarget::Damage, 0.5),
    )
    .with_duration(20.0)
});

pub static MINOR_PROPHECY: Lazy<BonusData> = Lazy::new(|| {
    BonusData::new(
        "Minor Prophecy",
        BonusSource::Buff,
        BonusTrigger::Cast,
        BonusValue::new("Minor Prophecy", BonusTarget::SpellCriticalRating, 1314.0),
    )
    .with_duration(20.0)
});

pub static MINOR_BRUTALITY: Lazy<BonusData> = Lazy::new(|| {
    BonusData::new(
        "Minor Brutality",
        BonusSource::Buff,
        BonusTrigger::Cast,
        BonusValue::new("Minor Brutality", BonusTarget::WeaponDamage, 0.1),
    )
    .with_duration(20.0)
});

pub static MINOR_SORCERY: Lazy<BonusData> = Lazy::new(|| {
    BonusData::new(
        "Minor Sorcery",
        BonusSource::Buff,
        BonusTrigger::Cast,
        BonusValue::new("Minor Sorcery", BonusTarget::SpellDamage, 0.1),
    )
    .with_duration(20.0)
});

pub static MAJOR_BERSERK: Lazy<BonusData> = Lazy::new(|| {
    BonusData::new(
        "Major Berserk",
        BonusSource::Buff,
        BonusTrigger::Cast,
        BonusValue::new("Major Berserk", BonusTarget::Damage, 0.10),
    )
    .with_duration(20.0)
});

pub static MAJOR_BRUTALITY: Lazy<BonusData> = Lazy::new(|| {
    BonusData::new(
        "Major Brutality",
        BonusSource::Buff,
        BonusTrigger::Cast,
        BonusValue::new("Major Brutality", BonusTarget::WeaponDamage, 0.20),
    )
    .with_duration(20.0)
});

pub static MAJOR_SORCERY: Lazy<BonusData> = Lazy::new(|| {
    BonusData::new(
        "Major Sorcery",
        BonusSource::Buff,
        BonusTrigger::Cast,
        BonusValue::new("Major Sorcery", BonusTarget::SpellDamage, 0.20),
    )
    .with_duration(20.0)
});

pub static MAJOR_SAVAGERY: Lazy<BonusData> = Lazy::new(|| {
    BonusData::new(
        "Major Savagery",
        BonusSource::Buff,
        BonusTrigger::Cast,
        BonusValue::new("Major Savagery", BonusTarget::WeaponCriticalRating, 2629.0),
    )
    .with_duration(20.0)
});

pub static MAJOR_PROPHECY: Lazy<BonusData> = Lazy::new(|| {
    BonusData::new(
        "Major Prophecy",
        BonusSource::Buff,
        BonusTrigger::Cast,
        BonusValue::new("Major Prophecy", BonusTarget::SpellCriticalRating, 2629.0),
    )
    .with_duration(20.0)
});

pub static EMPOWER: Lazy<BonusData> = Lazy::new(|| {
    BonusData::new(
        "Empower",
        BonusSource::Buff,
        BonusTrigger::Cast,
        BonusValue::new("Empower", BonusTarget::HeavyAttackDamage, 0.70),
    )
    .with_duration(10.0)
});

pub static MAJOR_VULNERABILITY: Lazy<BonusData> = Lazy::new(|| {
    BonusData::new(
        "Major Vulnerability",
        BonusSource::Buff,
        BonusTrigger::Cast,
        BonusValue::new("Major Vulnerability", BonusTarget::EnemyDamageTaken, 0.10),
    )
    .with_duration(12.0)
});

pub static MINOR_VULNERABILITY: Lazy<BonusData> = Lazy::new(|| {
    BonusData::new(
        "Minor Vulnerability",
        BonusSource::Buff,
        BonusTrigger::Cast,
        BonusValue::new("Minor Vulnerability", BonusTarget::EnemyDamageTaken, 0.05),
    )
    .with_duration(10.0)
});

pub static MAJOR_BREACH: Lazy<BonusData> = Lazy::new(|| {
    BonusData::new(
        "Major Breach",
        BonusSource::Buff,
        BonusTrigger::Cast,
        BonusValue::new(
            "Major Breach",
            BonusTarget::EnemyResistanceReduction,
            5948.0,
        ),
    )
    .with_duration(20.0)
});

pub static MINOR_FORCE: Lazy<BonusData> = Lazy::new(|| {
    BonusData::new(
        "Minor Force",
        BonusSource::Buff,
        BonusTrigger::Cast,
        BonusValue::new("Minor Force", BonusTarget::CriticalDamage, 0.10),
    )
    .with_duration(20.0)
});

pub static MINOR_BREACH: Lazy<BonusData> = Lazy::new(|| {
    BonusData::new(
        "Minor Breach",
        BonusSource::Buff,
        BonusTrigger::Cast,
        BonusValue::new(
            "Minor Breach",
            BonusTarget::EnemyResistanceReduction,
            2974.0,
        ),
    )
    .with_duration(20.0)
});
