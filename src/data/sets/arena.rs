use crate::domain::{
    BonusData, BonusSource, BonusTarget, BonusTrigger, BonusValue, SetData, SetType,
};
use once_cell::sync::Lazy;

pub static ARENA_SETS: Lazy<Vec<SetData>> = Lazy::new(|| {
    vec![
        // Crushing Wall — Maelstrom Arena Inferno Staff
        // Note: Staves count as 2 items, so 1 staff activates the 2pc bonus.
        SetData::new("Crushing Wall", SetType::Arena).with_threshold(
            2,
            vec![BonusData::new(
                "Crushing Wall 2pc",
                BonusSource::GearSet,
                BonusTrigger::Passive,
                // +1250 damage to Wall of Elements. Modeled as flat Light Attack Damage
                // since it effectively boosts a specific skill's output.
                // TODO: Model as Wall of Elements specific bonus once skill-specific
                // set bonuses are supported.
                BonusValue::new(
                    "Wall of Elements Damage",
                    BonusTarget::LightAttackDamage,
                    0.0, // Placeholder — skill-specific bonus, not a generic stat
                ),
            )],
        ),
        // Wrath of Elements — Vateshran Hollows Destruction Staff
        // Note: Staves count as 2 items, so 1 staff activates the 2pc bonus.
        SetData::new("Wrath of Elements", SetType::Arena).with_threshold(
            2,
            vec![
                // TODO: Proc damage — casting Weakness to Elements tethers for 10s,
                // dealing 331 Flame/Shock/Frost damage per second, increasing by 1%
                // each tick up to 20%. 10s cooldown.
                // Modeling proc DPS comes later.
            ],
        ),
    ]
});
