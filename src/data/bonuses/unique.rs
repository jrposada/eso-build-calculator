use crate::data::{BonusTarget, BonusTrigger};
use crate::domain::BonusData;
use once_cell::sync::Lazy;

pub static MINOR_SAVAGERY: Lazy<BonusData> = Lazy::new(|| {
    BonusData::new(
        "Minor Savagery",
        BonusTrigger::Cast,
        BonusTarget::WeaponCriticalChance,
        1314.0,
    )
    .with_duration(20.0)
});

pub static MINOR_BERSERK: Lazy<BonusData> = Lazy::new(|| {
    BonusData::new(
        "Minor Berserk",
        BonusTrigger::Cast,
        BonusTarget::Damage,
        0.5,
    )
    .with_duration(20.0)
});

pub static MINOR_PROPHECY: Lazy<BonusData> = Lazy::new(|| {
    BonusData::new(
        "Minor Prophecy",
        BonusTrigger::Cast,
        BonusTarget::SpellCriticalChance,
        1314.0,
    )
    .with_duration(20.0)
});

pub static MINOR_BRUTALITY: Lazy<BonusData> = Lazy::new(|| {
    BonusData::new(
        "Minor Brutality",
        BonusTrigger::Cast,
        BonusTarget::WeaponDamage,
        0.1,
    )
    .with_duration(20.0)
});

pub static MINOR_SORCERY: Lazy<BonusData> = Lazy::new(|| {
    BonusData::new(
        "Minor Sorcery",
        BonusTrigger::Cast,
        BonusTarget::SpellDamage,
        0.1,
    )
    .with_duration(20.0)
});

pub static MAJOR_BERSERK: Lazy<BonusData> = Lazy::new(|| {
    BonusData::new(
        "Major Berserk",
        BonusTrigger::Cast,
        BonusTarget::Damage,
        0.10,
    )
    .with_duration(20.0)
});

pub static MAJOR_BRUTALITY: Lazy<BonusData> = Lazy::new(|| {
    BonusData::new(
        "Major Brutality",
        BonusTrigger::Cast,
        BonusTarget::WeaponDamage,
        0.20,
    )
    .with_duration(20.0)
});

pub static MAJOR_SORCERY: Lazy<BonusData> = Lazy::new(|| {
    BonusData::new(
        "Major Sorcery",
        BonusTrigger::Cast,
        BonusTarget::SpellDamage,
        0.20,
    )
    .with_duration(20.0)
});

pub static MAJOR_SAVAGERY: Lazy<BonusData> = Lazy::new(|| {
    BonusData::new(
        "Major Savagery",
        BonusTrigger::Cast,
        BonusTarget::WeaponCriticalChance,
        2629.0,
    )
    .with_duration(20.0)
});

pub static MAJOR_PROPHECY: Lazy<BonusData> = Lazy::new(|| {
    BonusData::new(
        "Major Prophecy",
        BonusTrigger::Cast,
        BonusTarget::SpellCriticalChance,
        2629.0,
    )
    .with_duration(20.0)
});

pub static EMPOWER: Lazy<BonusData> = Lazy::new(|| {
    BonusData::new(
        "Empower",
        BonusTrigger::Cast,
        BonusTarget::HeavyAttackDamage,
        0.70,
    )
    .with_duration(10.0)
});

// Enemy Debuffs (increase damage taken by target)
pub static MINOR_VULNERABILITY: Lazy<BonusData> = Lazy::new(|| {
    BonusData::new(
        "Minor Vulnerability",
        BonusTrigger::Cast,
        BonusTarget::EnemyDamageTaken,
        0.05,
    )
    .with_duration(10.0)
});

pub static MAJOR_BREACH: Lazy<BonusData> = Lazy::new(|| {
    BonusData::new(
        "Major Breach",
        BonusTrigger::Cast,
        BonusTarget::EnemyResistanceReduction,
        5948.0,
    )
    .with_duration(20.0)
});

pub static MINOR_BREACH: Lazy<BonusData> = Lazy::new(|| {
    BonusData::new(
        "Minor Breach",
        BonusTrigger::Cast,
        BonusTarget::EnemyResistanceReduction,
        2974.0,
    )
    .with_duration(20.0)
});
