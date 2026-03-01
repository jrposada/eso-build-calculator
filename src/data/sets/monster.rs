use crate::domain::{
    BonusData, BonusSource, BonusTarget, BonusTrigger, BonusValue, SetData, SetType,
};
use once_cell::sync::Lazy;

pub static MONSTER_SETS: Lazy<Vec<SetData>> = Lazy::new(|| {
    vec![
        // Zaan — Scalecaller Peak
        SetData::new("Zaan", SetType::Monster)
            .with_threshold(
                1,
                vec![BonusData::new(
                    "Zaan 1pc",
                    BonusSource::GearSet,
                    BonusTrigger::Passive,
                    BonusValue::new("Critical Chance", BonusTarget::CriticalRating, 657.0),
                )],
            )
            .with_threshold(
                2,
                vec![
                    // TODO: Proc damage — critical light/heavy attack tethers for 10s,
                    // dealing escalating flame damage. 20s cooldown.
                    // Modeling proc DPS comes later.
                ],
            ),
        // Maw of the Infernal — Banished Cells II
        SetData::new("Maw of the Infernal", SetType::Monster)
            .with_threshold(
                1,
                vec![BonusData::new(
                    "Maw of the Infernal 1pc",
                    BonusSource::GearSet,
                    BonusTrigger::Passive,
                    BonusValue::new(
                        "Weapon and Spell Damage",
                        BonusTarget::WeaponAndSpellDamageFlat,
                        129.0,
                    ),
                )],
            )
            .with_threshold(
                2,
                vec![
                    // TODO: Proc damage — 33% chance on LA/HA to summon Daedroth for 15s,
                    // dealing 599 flame damage every 2s. 15s cooldown.
                    // Modeling proc DPS comes later.
                ],
            ),
    ]
});
