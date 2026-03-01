use crate::domain::{
    BonusData, BonusSource, BonusTarget, BonusTrigger, BonusValue, SetData, SetType,
};
use once_cell::sync::Lazy;

pub static MYTHIC_SETS: Lazy<Vec<SetData>> = Lazy::new(|| {
    vec![
        // Harpooner's Wading Kilt — Mythic Leg Armor
        // TODO: Complex stacking mechanic — dealing direct damage grants Hunter's Focus
        // (up to 10 stacks, 1/sec). Each stack: +110 Crit Chance, +1% Crit Damage.
        // Taking direct damage removes 5 stacks (1/sec). Stacks last 20s.
        // For now, model at average 7 stacks as a rough approximation.
        SetData::new("Harpooner's Wading Kilt", SetType::Mythic).with_threshold(
            1,
            vec![
                BonusData::new(
                    "Harpooner's Wading Kilt 1pc (Crit Chance)",
                    BonusSource::GearSet,
                    BonusTrigger::Passive,
                    BonusValue::new(
                        "Critical Chance (avg 7 stacks)",
                        BonusTarget::CriticalRating,
                        770.0, // 110 * 7 stacks
                    ),
                ),
                BonusData::new(
                    "Harpooner's Wading Kilt 1pc (Crit Damage)",
                    BonusSource::GearSet,
                    BonusTrigger::Passive,
                    BonusValue::new(
                        "Critical Damage (avg 7 stacks)",
                        BonusTarget::CriticalDamage,
                        0.07, // 1% * 7 stacks
                    ),
                ),
            ],
        ),
    ]
});
