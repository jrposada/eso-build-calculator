use crate::domain::{ArmorWeight, BonusData, BonusSource, BonusTarget, BonusTrigger, BonusValue};

/// Returns armor passive bonuses for the given armor weight.
/// Assumes 5 pieces of the primary armor weight (standard DPS setup).
pub fn armor_passives(weight: ArmorWeight) -> Vec<BonusData> {
    match weight {
        ArmorWeight::Medium => vec![
            // Wind Walker: +2% Weapon Damage per medium piece = +10% for 5 pieces
            BonusData::new(
                "Wind Walker",
                BonusSource::Passive,
                BonusTrigger::Passive,
                BonusValue::new("Wind Walker", BonusTarget::WeaponDamage, 0.10),
            ),
            // Agility: +2% crit chance per medium piece
            // 2% crit chance ≈ 2,191 crit rating for 5 pieces (10,956 for 100%, so 2% = 219.12 per %, 5 pieces × 2% = 10%)
            // Actually: 2,191 rating per 5 pieces (from UESP)
            BonusData::new(
                "Agility",
                BonusSource::Passive,
                BonusTrigger::Passive,
                BonusValue::new("Agility", BonusTarget::CriticalRating, 2191.0),
            ),
        ],
        ArmorWeight::Light => vec![
            // Prodigy: +2,629 Spell Critical Rating (with 5 pieces)
            BonusData::new(
                "Prodigy",
                BonusSource::Passive,
                BonusTrigger::Passive,
                BonusValue::new("Prodigy", BonusTarget::SpellCriticalRating, 2629.0),
            ),
            // Concentration: +4,484 Spell Penetration (with 5 pieces)
            BonusData::new(
                "Concentration",
                BonusSource::Passive,
                BonusTrigger::Passive,
                BonusValue::new(
                    "Concentration",
                    BonusTarget::PhysicalAndSpellPenetration,
                    4484.0,
                ),
            ),
        ],
        ArmorWeight::Heavy => vec![
            // Heavy armor passives not DPS-relevant
        ],
    }
}
