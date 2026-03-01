use crate::domain::{
    BonusData, BonusSource, BonusTarget, BonusTrigger, BonusValue, SetData, SetType,
};
use once_cell::sync::Lazy;

pub static NORMAL_SETS: Lazy<Vec<SetData>> = Lazy::new(|| {
    vec![
        // Mother's Sorrow — Overland (Deshaan), Light Armor
        SetData::new("Mother's Sorrow", SetType::Normal)
            .with_threshold(
                2,
                vec![BonusData::new(
                    "Mother's Sorrow 2pc",
                    BonusSource::GearSet,
                    BonusTrigger::Passive,
                    BonusValue::new("Max Magicka", BonusTarget::MaxMagickaFlat, 1096.0),
                )],
            )
            .with_threshold(
                3,
                vec![BonusData::new(
                    "Mother's Sorrow 3pc",
                    BonusSource::GearSet,
                    BonusTrigger::Passive,
                    BonusValue::new("Critical Chance", BonusTarget::CriticalRating, 657.0),
                )],
            )
            .with_threshold(
                4,
                vec![BonusData::new(
                    "Mother's Sorrow 4pc",
                    BonusSource::GearSet,
                    BonusTrigger::Passive,
                    BonusValue::new("Critical Chance", BonusTarget::CriticalRating, 657.0),
                )],
            )
            .with_threshold(
                5,
                vec![BonusData::new(
                    "Mother's Sorrow 5pc",
                    BonusSource::GearSet,
                    BonusTrigger::Passive,
                    BonusValue::new("Critical Chance", BonusTarget::CriticalRating, 1528.0),
                )],
            ),
        // Law of Julianos — Craftable (Wrothgar), Light Armor
        SetData::new("Law of Julianos", SetType::Normal)
            .with_threshold(
                2,
                vec![BonusData::new(
                    "Law of Julianos 2pc",
                    BonusSource::GearSet,
                    BonusTrigger::Passive,
                    BonusValue::new("Critical Chance", BonusTarget::CriticalRating, 657.0),
                )],
            )
            .with_threshold(
                3,
                vec![BonusData::new(
                    "Law of Julianos 3pc",
                    BonusSource::GearSet,
                    BonusTrigger::Passive,
                    BonusValue::new("Max Magicka", BonusTarget::MaxMagickaFlat, 1096.0),
                )],
            )
            .with_threshold(
                4,
                vec![BonusData::new(
                    "Law of Julianos 4pc",
                    BonusSource::GearSet,
                    BonusTrigger::Passive,
                    BonusValue::new("Critical Chance", BonusTarget::CriticalRating, 657.0),
                )],
            )
            .with_threshold(
                5,
                vec![BonusData::new(
                    "Law of Julianos 5pc",
                    BonusSource::GearSet,
                    BonusTrigger::Passive,
                    BonusValue::new(
                        "Weapon and Spell Damage",
                        BonusTarget::WeaponAndSpellDamageFlat,
                        300.0,
                    ),
                )],
            ),
        // Hunding's Rage — Craftable (Alliance zones), Medium Armor
        SetData::new("Hunding's Rage", SetType::Normal)
            .with_threshold(
                2,
                vec![BonusData::new(
                    "Hunding's Rage 2pc",
                    BonusSource::GearSet,
                    BonusTrigger::Passive,
                    BonusValue::new("Critical Chance", BonusTarget::CriticalRating, 657.0),
                )],
            )
            .with_threshold(
                3,
                vec![BonusData::new(
                    "Hunding's Rage 3pc",
                    BonusSource::GearSet,
                    BonusTrigger::Passive,
                    BonusValue::new("Max Stamina", BonusTarget::MaxStaminaFlat, 1096.0),
                )],
            )
            .with_threshold(
                4,
                vec![BonusData::new(
                    "Hunding's Rage 4pc",
                    BonusSource::GearSet,
                    BonusTrigger::Passive,
                    BonusValue::new("Critical Chance", BonusTarget::CriticalRating, 657.0),
                )],
            )
            .with_threshold(
                5,
                vec![BonusData::new(
                    "Hunding's Rage 5pc",
                    BonusSource::GearSet,
                    BonusTrigger::Passive,
                    BonusValue::new(
                        "Weapon and Spell Damage",
                        BonusTarget::WeaponAndSpellDamageFlat,
                        300.0,
                    ),
                )],
            ),
        // Pillar of Nirn — Dungeon (Falkreath Hold), Medium Armor
        SetData::new("Pillar of Nirn", SetType::Normal)
            .with_threshold(
                2,
                vec![BonusData::new(
                    "Pillar of Nirn 2pc",
                    BonusSource::GearSet,
                    BonusTrigger::Passive,
                    BonusValue::new("Critical Chance", BonusTarget::CriticalRating, 657.0),
                )],
            )
            .with_threshold(
                3,
                vec![BonusData::new(
                    "Pillar of Nirn 3pc",
                    BonusSource::GearSet,
                    BonusTrigger::Passive,
                    BonusValue::new("Critical Chance", BonusTarget::CriticalRating, 657.0),
                )],
            )
            .with_threshold(
                4,
                vec![BonusData::new(
                    "Pillar of Nirn 4pc",
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
                5,
                vec![
                    // TODO: Proc damage — creates a fissure dealing 803 Bleed + 2405 Bleed over 10s.
                    // Triggers every 10s. Modeling proc DPS comes later.
                ],
            ),
        // Whorl of the Depths — Trial (Dreadsail Reef), Light Armor
        SetData::new("Whorl of the Depths", SetType::Normal)
            .with_threshold(
                2,
                vec![BonusData::new(
                    "Whorl of the Depths 2pc",
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
                3,
                vec![
                    // Minor Slayer: +5% damage to Dungeon, Trial, and Arena Monsters
                    // Modeled as generic Damage since we assume trial/dungeon context
                    BonusData::new(
                        "Whorl of the Depths 3pc",
                        BonusSource::GearSet,
                        BonusTrigger::Passive,
                        BonusValue::new("Minor Slayer", BonusTarget::Damage, 0.05),
                    ),
                ],
            )
            .with_threshold(
                4,
                vec![BonusData::new(
                    "Whorl of the Depths 4pc",
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
                5,
                vec![
                    // TODO: Proc damage — light attack triggers frost DoT for 8s,
                    // then whirlpool for 6s dealing frost damage/sec. 18s cooldown.
                    // Modeling proc DPS comes later.
                ],
            ),
    ]
});
