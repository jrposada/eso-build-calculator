use crate::domain::{BonusData, BonusSource, BonusTarget, BonusTrigger, BonusValue};
use once_cell::sync::Lazy;
use std::collections::HashSet;

/// All DPS-relevant buffs/debuffs provided by the 21M trial dummy.
/// These are injected as passive bonuses so they apply to stat resolution
/// and damage modifiers without needing the simulator's buff system.
pub static TRIAL_DUMMY_BUFFS: Lazy<Vec<BonusData>> = Lazy::new(|| {
    vec![
        // Major Force: +15% Critical Damage
        BonusData::new(
            "Major Force",
            BonusSource::Buff,
            BonusTrigger::Passive,
            BonusValue::new("Major Force", BonusTarget::CriticalDamage, 0.15),
        ),
        // Major Courage: +258 Weapon and Spell Damage (flat)
        BonusData::new(
            "Major Courage",
            BonusSource::Buff,
            BonusTrigger::Passive,
            BonusValue::new(
                "Major Courage",
                BonusTarget::WeaponAndSpellDamageFlat,
                258.0,
            ),
        ),
        // Minor Courage: +129 Weapon and Spell Damage (flat)
        BonusData::new(
            "Minor Courage",
            BonusSource::Buff,
            BonusTrigger::Passive,
            BonusValue::new(
                "Minor Courage",
                BonusTarget::WeaponAndSpellDamageFlat,
                129.0,
            ),
        ),
        // Major Slayer: +15% Damage
        BonusData::new(
            "Major Slayer",
            BonusSource::Buff,
            BonusTrigger::Passive,
            BonusValue::new("Major Slayer", BonusTarget::Damage, 0.15),
        ),
        // Minor Brutality: +10% Weapon Damage
        BonusData::new(
            "Minor Brutality",
            BonusSource::Buff,
            BonusTrigger::Passive,
            BonusValue::new("Minor Brutality", BonusTarget::WeaponDamage, 0.10),
        ),
        // Minor Savagery: +1314 Weapon Critical Rating
        BonusData::new(
            "Minor Savagery",
            BonusSource::Buff,
            BonusTrigger::Passive,
            BonusValue::new("Minor Savagery", BonusTarget::WeaponCriticalRating, 1314.0),
        ),
        // Minor Prophecy: +1314 Spell Critical Rating
        BonusData::new(
            "Minor Prophecy",
            BonusSource::Buff,
            BonusTrigger::Passive,
            BonusValue::new("Minor Prophecy", BonusTarget::SpellCriticalRating, 1314.0),
        ),
        // Minor Sorcery: +10% Spell Damage
        BonusData::new(
            "Minor Sorcery",
            BonusSource::Buff,
            BonusTrigger::Passive,
            BonusValue::new("Minor Sorcery", BonusTarget::SpellDamage, 0.10),
        ),
        // Minor Berserk: +5% Damage
        BonusData::new(
            "Minor Berserk",
            BonusSource::Buff,
            BonusTrigger::Passive,
            BonusValue::new("Minor Berserk", BonusTarget::Damage, 0.05),
        ),
        // Major Breach: +5948 Enemy Resistance Reduction
        BonusData::new(
            "Major Breach",
            BonusSource::Buff,
            BonusTrigger::Passive,
            BonusValue::new(
                "Major Breach",
                BonusTarget::EnemyResistanceReduction,
                5948.0,
            ),
        ),
        // Minor Breach: +2974 Enemy Resistance Reduction
        BonusData::new(
            "Minor Breach",
            BonusSource::Buff,
            BonusTrigger::Passive,
            BonusValue::new(
                "Minor Breach",
                BonusTarget::EnemyResistanceReduction,
                2974.0,
            ),
        ),
        // Major Vulnerability: +10% Enemy Damage Taken
        BonusData::new(
            "Major Vulnerability",
            BonusSource::Buff,
            BonusTrigger::Passive,
            BonusValue::new(
                "Major Vulnerability",
                BonusTarget::EnemyDamageTaken,
                0.10,
            ),
        ),
        // Minor Vulnerability: +5% Enemy Damage Taken
        BonusData::new(
            "Minor Vulnerability",
            BonusSource::Buff,
            BonusTrigger::Passive,
            BonusValue::new(
                "Minor Vulnerability",
                BonusTarget::EnemyDamageTaken,
                0.05,
            ),
        ),
        // Minor Brittle: +1% Critical Damage
        BonusData::new(
            "Minor Brittle",
            BonusSource::Buff,
            BonusTrigger::Passive,
            BonusValue::new("Minor Brittle", BonusTarget::CriticalDamage, 0.01),
        ),
        // Engulfing Flames: +10% Flame Damage
        BonusData::new(
            "Engulfing Flames",
            BonusSource::Buff,
            BonusTrigger::Passive,
            BonusValue::new("Engulfing Flames", BonusTarget::FlameDamage, 0.10),
        ),
        // Infused Crusher: +2108 Enemy Resistance Reduction
        BonusData::new(
            "Infused Crusher",
            BonusSource::Buff,
            BonusTrigger::Passive,
            BonusValue::new(
                "Infused Crusher",
                BonusTarget::EnemyResistanceReduction,
                2108.0,
            ),
        ),
        // Roar of Alkosh: +6000 Enemy Resistance Reduction
        BonusData::new(
            "Roar of Alkosh",
            BonusSource::Buff,
            BonusTrigger::Passive,
            BonusValue::new(
                "Roar of Alkosh",
                BonusTarget::EnemyResistanceReduction,
                6000.0,
            ),
        ),
        // Elemental Catalyst (x3): +5% each for Flame, Frost, Shock
        BonusData::new(
            "Elemental Catalyst (Flame)",
            BonusSource::Buff,
            BonusTrigger::Passive,
            BonusValue::new(
                "Elemental Catalyst (Flame)",
                BonusTarget::FlameDamage,
                0.05,
            ),
        ),
        BonusData::new(
            "Elemental Catalyst (Frost)",
            BonusSource::Buff,
            BonusTrigger::Passive,
            BonusValue::new(
                "Elemental Catalyst (Frost)",
                BonusTarget::FrostDamage,
                0.05,
            ),
        ),
        BonusData::new(
            "Elemental Catalyst (Shock)",
            BonusSource::Buff,
            BonusTrigger::Passive,
            BonusValue::new(
                "Elemental Catalyst (Shock)",
                BonusTarget::ShockDamage,
                0.05,
            ),
        ),
    ]
});

/// Names of all trial dummy buffs, used to suppress duplicate buffs in the simulator
/// when a player's skill provides the same buff that the trial dummy already provides.
pub static TRIAL_BUFF_NAMES: Lazy<HashSet<String>> = Lazy::new(|| {
    TRIAL_DUMMY_BUFFS
        .iter()
        .map(|b| b.name.clone())
        .collect()
});
