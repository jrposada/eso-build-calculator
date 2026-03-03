use serde::{Deserialize, Serialize};
use std::fmt;

use super::{BonusData, BonusSource, BonusTarget, BonusTrigger, BonusValue};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Potion {
    WeaponPower,
    SpellPower,
}

impl Potion {
    /// Returns passive bonuses for this potion type.
    /// Potions have ~100% uptime on trial dummy (47.6s duration, 45s cooldown),
    /// so they are modeled as flat passive bonuses.
    pub fn bonuses(&self) -> Vec<BonusData> {
        match self {
            Potion::WeaponPower => vec![
                // +371 Weapon and Spell Damage (gold CP160 potion)
                BonusData::new(
                    "Weapon Power Potion (Damage)",
                    BonusSource::Buff,
                    BonusTrigger::Passive,
                    BonusValue::new(
                        "Weapon Power Potion (Damage)",
                        BonusTarget::WeaponAndSpellDamageFlat,
                        371.0,
                    ),
                ),
                // Major Savagery: +2,629 Weapon Critical Rating
                BonusData::new(
                    "Major Savagery",
                    BonusSource::Buff,
                    BonusTrigger::Passive,
                    BonusValue::new("Major Savagery", BonusTarget::WeaponCriticalRating, 2629.0),
                ),
            ],
            Potion::SpellPower => vec![
                // +371 Weapon and Spell Damage (gold CP160 potion)
                BonusData::new(
                    "Spell Power Potion (Damage)",
                    BonusSource::Buff,
                    BonusTrigger::Passive,
                    BonusValue::new(
                        "Spell Power Potion (Damage)",
                        BonusTarget::WeaponAndSpellDamageFlat,
                        371.0,
                    ),
                ),
                // Major Prophecy: +2,629 Spell Critical Rating
                BonusData::new(
                    "Major Prophecy",
                    BonusSource::Buff,
                    BonusTrigger::Passive,
                    BonusValue::new("Major Prophecy", BonusTarget::SpellCriticalRating, 2629.0),
                ),
            ],
        }
    }

    pub fn parse(s: &str) -> Result<Potion, String> {
        match s.to_lowercase().replace(' ', "-").as_str() {
            "weapon-power" | "weapon" => Ok(Potion::WeaponPower),
            "spell-power" | "spell" => Ok(Potion::SpellPower),
            _ => Err(format!(
                "Unknown potion '{}'. Valid: weapon-power, spell-power",
                s
            )),
        }
    }
}

impl fmt::Display for Potion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Potion::WeaponPower => write!(f, "Weapon Power"),
            Potion::SpellPower => write!(f, "Spell Power"),
        }
    }
}
