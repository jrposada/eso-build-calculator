use serde::{Deserialize, Serialize};
use std::fmt;

use super::character_stats::ATTRIBUTE_POINTS_BONUS;
use super::food::Food;
use super::mundus::MundusStone;
use crate::data::skill_trees::race::Race;
use super::weapon_type::WeaponType;
use super::CharacterStats;

/// Armor weight determines which armor passives apply.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ArmorWeight {
    Medium,
    Light,
    Heavy,
}

impl ArmorWeight {
    pub fn parse(s: &str) -> Result<ArmorWeight, String> {
        match s.to_lowercase().as_str() {
            "medium" => Ok(ArmorWeight::Medium),
            "light" => Ok(ArmorWeight::Light),
            "heavy" => Ok(ArmorWeight::Heavy),
            _ => Err(format!(
                "Unknown armor weight '{}'. Valid: medium, light, heavy",
                s
            )),
        }
    }
}

impl fmt::Display for ArmorWeight {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ArmorWeight::Medium => write!(f, "Medium"),
            ArmorWeight::Light => write!(f, "Light"),
            ArmorWeight::Heavy => write!(f, "Heavy"),
        }
    }
}

/// Armor trait applied to all 7 armor pieces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ArmorTrait {
    Divines,
    Infused,
    WellFitted,
    Sturdy,
    Impenetrable,
    Training,
    Reinforced,
    Nirnhoned,
}

/// DPS-relevant armor traits for gear optimization.
pub const DPS_ARMOR_TRAITS: &[ArmorTrait] = &[ArmorTrait::Divines, ArmorTrait::Infused];

impl ArmorTrait {
    pub fn parse(s: &str) -> Result<ArmorTrait, String> {
        match s.to_lowercase().as_str() {
            "divines" => Ok(ArmorTrait::Divines),
            "infused" => Ok(ArmorTrait::Infused),
            "well-fitted" | "wellfitted" => Ok(ArmorTrait::WellFitted),
            "sturdy" => Ok(ArmorTrait::Sturdy),
            "impenetrable" | "impen" => Ok(ArmorTrait::Impenetrable),
            "training" => Ok(ArmorTrait::Training),
            "reinforced" => Ok(ArmorTrait::Reinforced),
            "nirnhoned" => Ok(ArmorTrait::Nirnhoned),
            _ => Err(format!(
                "Unknown armor trait '{}'. Valid: divines, infused, well-fitted, sturdy, \
                 impenetrable, training, reinforced, nirnhoned",
                s
            )),
        }
    }
}

impl fmt::Display for ArmorTrait {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ArmorTrait::Divines => write!(f, "Divines"),
            ArmorTrait::Infused => write!(f, "Infused"),
            ArmorTrait::WellFitted => write!(f, "Well-Fitted"),
            ArmorTrait::Sturdy => write!(f, "Sturdy"),
            ArmorTrait::Impenetrable => write!(f, "Impenetrable"),
            ArmorTrait::Training => write!(f, "Training"),
            ArmorTrait::Reinforced => write!(f, "Reinforced"),
            ArmorTrait::Nirnhoned => write!(f, "Nirnhoned"),
        }
    }
}

/// Jewelry trait applied to all 3 jewelry pieces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum JewelryTrait {
    Bloodthirsty,
    Infused,
    Arcane,
    Robust,
    Harmony,
    Protective,
    Swift,
    Triune,
}

/// DPS-relevant jewelry traits for gear optimization.
pub const DPS_JEWELRY_TRAITS: &[JewelryTrait] = &[
    JewelryTrait::Bloodthirsty,
    JewelryTrait::Infused,
    JewelryTrait::Arcane,
    JewelryTrait::Robust,
];

impl JewelryTrait {
    pub fn parse(s: &str) -> Result<JewelryTrait, String> {
        match s.to_lowercase().as_str() {
            "bloodthirsty" => Ok(JewelryTrait::Bloodthirsty),
            "infused" => Ok(JewelryTrait::Infused),
            "arcane" => Ok(JewelryTrait::Arcane),
            "robust" => Ok(JewelryTrait::Robust),
            "harmony" => Ok(JewelryTrait::Harmony),
            "protective" => Ok(JewelryTrait::Protective),
            "swift" => Ok(JewelryTrait::Swift),
            "triune" => Ok(JewelryTrait::Triune),
            _ => Err(format!(
                "Unknown jewelry trait '{}'. Valid: bloodthirsty, infused, arcane, robust, \
                 harmony, protective, swift, triune",
                s
            )),
        }
    }
}

impl fmt::Display for JewelryTrait {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            JewelryTrait::Bloodthirsty => write!(f, "Bloodthirsty"),
            JewelryTrait::Infused => write!(f, "Infused"),
            JewelryTrait::Arcane => write!(f, "Arcane"),
            JewelryTrait::Robust => write!(f, "Robust"),
            JewelryTrait::Harmony => write!(f, "Harmony"),
            JewelryTrait::Protective => write!(f, "Protective"),
            JewelryTrait::Swift => write!(f, "Swift"),
            JewelryTrait::Triune => write!(f, "Triune"),
        }
    }
}

/// Weapon trait applied to the equipped weapon(s).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum WeaponTrait {
    Nirnhoned,
    Precise,
    Sharpened,
    Charged,
    Infused,
    Decisive,
    Defending,
    Powered,
    Training,
}

/// DPS-relevant weapon traits for gear optimization.
pub const DPS_WEAPON_TRAITS: &[WeaponTrait] = &[
    WeaponTrait::Nirnhoned,
    WeaponTrait::Precise,
    WeaponTrait::Sharpened,
];

impl WeaponTrait {
    pub fn parse(s: &str) -> Result<WeaponTrait, String> {
        match s.to_lowercase().as_str() {
            "nirnhoned" => Ok(WeaponTrait::Nirnhoned),
            "precise" => Ok(WeaponTrait::Precise),
            "sharpened" => Ok(WeaponTrait::Sharpened),
            "charged" => Ok(WeaponTrait::Charged),
            "infused" => Ok(WeaponTrait::Infused),
            "decisive" => Ok(WeaponTrait::Decisive),
            "defending" => Ok(WeaponTrait::Defending),
            "powered" => Ok(WeaponTrait::Powered),
            "training" => Ok(WeaponTrait::Training),
            _ => Err(format!(
                "Unknown weapon trait '{}'. Valid: nirnhoned, precise, sharpened, charged, \
                 infused, decisive, defending, powered, training",
                s
            )),
        }
    }
}

impl fmt::Display for WeaponTrait {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WeaponTrait::Nirnhoned => write!(f, "Nirnhoned"),
            WeaponTrait::Precise => write!(f, "Precise"),
            WeaponTrait::Sharpened => write!(f, "Sharpened"),
            WeaponTrait::Charged => write!(f, "Charged"),
            WeaponTrait::Infused => write!(f, "Infused"),
            WeaponTrait::Decisive => write!(f, "Decisive"),
            WeaponTrait::Defending => write!(f, "Defending"),
            WeaponTrait::Powered => write!(f, "Powered"),
            WeaponTrait::Training => write!(f, "Training"),
        }
    }
}

/// Attribute point allocation choice.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AttributeChoice {
    None,
    Magicka,
    Stamina,
}

/// DPS-relevant attribute choices for gear optimization.
pub const DPS_ATTRIBUTES: &[AttributeChoice] = &[
    AttributeChoice::None,
    AttributeChoice::Magicka,
    AttributeChoice::Stamina,
];

impl fmt::Display for AttributeChoice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AttributeChoice::None => write!(f, "None"),
            AttributeChoice::Magicka => write!(f, "Magicka"),
            AttributeChoice::Stamina => write!(f, "Stamina"),
        }
    }
}

/// Full gear configuration that determines character stats from equipment choices.
#[derive(Debug, Clone)]
pub struct GearConfig {
    pub race: Option<Race>,
    pub mundus: Option<MundusStone>,
    pub food: Option<Food>,
    pub armor_trait: ArmorTrait,
    pub jewelry_trait: JewelryTrait,
    pub weapon_trait: WeaponTrait,
    pub attributes: AttributeChoice,
    pub armor_weight: ArmorWeight,
}

impl Default for GearConfig {
    fn default() -> Self {
        Self {
            race: None,
            mundus: None,
            food: None,
            armor_trait: ArmorTrait::Divines,
            jewelry_trait: JewelryTrait::Bloodthirsty,
            weapon_trait: WeaponTrait::Nirnhoned,
            attributes: AttributeChoice::None,
            armor_weight: ArmorWeight::Medium,
        }
    }
}

// Weapon base damage at CP160 gold quality
const TWO_HANDED_BASE_DAMAGE: f64 = 1_571.0;
const ONE_HANDED_BASE_DAMAGE: f64 = 1_335.0;

// Armor enchantment values (CP160 gold glyphs)
const LARGE_ARMOR_ENCHANT: f64 = 868.0; // chest, head, legs
const SMALL_ARMOR_ENCHANT: f64 = 351.0; // shoulders, hands, waist, feet
const TOTAL_ARMOR_ENCHANT: f64 = LARGE_ARMOR_ENCHANT * 3.0 + SMALL_ARMOR_ENCHANT * 4.0; // 4,008

// Jewelry enchantment values (CP160 gold, Glyph of Increase Physical Harm)
const JEWELRY_ENCHANT_DAMAGE: f64 = 174.0; // per piece
const JEWELRY_PIECE_COUNT: f64 = 3.0;

// Trait values (legendary quality)
const ARMOR_INFUSED_ENCHANT_BONUS: f64 = 0.12; // +12% enchant per piece
const JEWELRY_INFUSED_ENCHANT_BONUS: f64 = 0.60; // +60% enchant effect
const JEWELRY_ROBUST_STAMINA: f64 = 877.0; // per piece
const JEWELRY_ARCANE_MAGICKA: f64 = 877.0; // per piece
const BLOODTHIRSTY_MAX_PER_PIECE: f64 = 350.0; // +350 WD/SD per piece at 0% HP
const BLOODTHIRSTY_THRESHOLD: f64 = 0.90; // Scales below 90% enemy HP
const WEAPON_NIRNHONED_BONUS: f64 = 0.15; // +15% of weapon base as W/SD
const WEAPON_PRECISE_CRIT_RATING: f64 = 1_117.0;
const WEAPON_SHARPENED_PENETRATION: f64 = 3_276.0;

const ARMOR_PIECE_COUNT: u8 = 7;

impl GearConfig {
    /// Compute character stats from gear configuration and weapon types.
    /// `bar1_weapon` is the primary weapon used for base damage calculation.
    pub fn compute_stats(&self, bar1_weapon: Option<WeaponType>) -> CharacterStats {
        let mut stats = CharacterStats::default();

        // 1. Attribute points
        match self.attributes {
            AttributeChoice::Magicka => stats.max_magicka += ATTRIBUTE_POINTS_BONUS,
            AttributeChoice::Stamina => stats.max_stamina += ATTRIBUTE_POINTS_BONUS,
            AttributeChoice::None => {}
        }

        // 2. Weapon base damage (replaces default 1,000)
        if let Some(weapon) = bar1_weapon {
            let base = if weapon.is_two_handed() {
                TWO_HANDED_BASE_DAMAGE
            } else {
                ONE_HANDED_BASE_DAMAGE
            };

            if weapon.is_destruction_staff() {
                stats.spell_damage = base;
            } else {
                stats.weapon_damage = base;
            }

            // 3. Weapon trait bonus
            match self.weapon_trait {
                WeaponTrait::Nirnhoned => {
                    let bonus = base * WEAPON_NIRNHONED_BONUS;
                    if weapon.is_destruction_staff() {
                        stats.spell_damage += bonus;
                    } else {
                        stats.weapon_damage += bonus;
                    }
                }
                WeaponTrait::Precise => {
                    stats.critical_rating += WEAPON_PRECISE_CRIT_RATING;
                }
                WeaponTrait::Sharpened => {
                    stats.penetration += WEAPON_SHARPENED_PENETRATION;
                }
                _ => {} // Other traits not modeled for DPS
            }
        }

        // 4. Armor enchantments (7 pieces, all stamina or magicka)
        let armor_enchant_total = match self.armor_trait {
            ArmorTrait::Infused => {
                TOTAL_ARMOR_ENCHANT * (1.0 + ARMOR_INFUSED_ENCHANT_BONUS)
            }
            _ => TOTAL_ARMOR_ENCHANT,
        };

        // Apply enchant to the primary resource
        if self.attributes == AttributeChoice::Magicka
            || bar1_weapon.map_or(false, |w| w.is_destruction_staff())
        {
            stats.max_magicka += armor_enchant_total;
        } else {
            stats.max_stamina += armor_enchant_total;
        }

        // 5. Jewelry enchantments (3 pieces, weapon+spell damage)
        let jewelry_enchant_per_piece = match self.jewelry_trait {
            JewelryTrait::Infused => JEWELRY_ENCHANT_DAMAGE * (1.0 + JEWELRY_INFUSED_ENCHANT_BONUS),
            _ => JEWELRY_ENCHANT_DAMAGE,
        };
        let jewelry_damage_total = jewelry_enchant_per_piece * JEWELRY_PIECE_COUNT;
        stats.weapon_damage += jewelry_damage_total;
        stats.spell_damage += jewelry_damage_total;

        // 6. Jewelry trait bonus (if not Infused, which was handled above)
        match self.jewelry_trait {
            JewelryTrait::Bloodthirsty => {
                // Bloodthirsty: up to +350 WD/SD per piece, scaling linearly below 90% HP.
                // Fight-average: 90% of fight below threshold, average bonus = max/2,
                // so avg = 0.90 * (max / 2) per piece.
                let avg_per_piece = BLOODTHIRSTY_THRESHOLD
                    * (BLOODTHIRSTY_MAX_PER_PIECE / 2.0);
                let total = avg_per_piece * JEWELRY_PIECE_COUNT;
                stats.weapon_damage += total;
                stats.spell_damage += total;
            }
            JewelryTrait::Robust => {
                stats.max_stamina += JEWELRY_ROBUST_STAMINA * JEWELRY_PIECE_COUNT;
            }
            JewelryTrait::Arcane => {
                stats.max_magicka += JEWELRY_ARCANE_MAGICKA * JEWELRY_PIECE_COUNT;
            }
            _ => {} // Other traits not modeled for DPS
        }

        // 7. Mundus stone (amplified by Divines)
        if let Some(mundus) = &self.mundus {
            let divines_count = match self.armor_trait {
                ArmorTrait::Divines => ARMOR_PIECE_COUNT,
                _ => 0,
            };
            mundus.apply(&mut stats, divines_count);
        }

        // 8. Racial passives
        if let Some(race) = &self.race {
            race.apply(&mut stats);
        }

        // 9. Food
        if let Some(food) = &self.food {
            food.apply(&mut stats);
        }

        stats
    }
}
