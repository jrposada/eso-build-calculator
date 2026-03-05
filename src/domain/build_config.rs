use serde::{Deserialize, Serialize};

use super::character_stats::ATTRIBUTE_POINTS_BONUS;
use super::class_name::ClassName;
use super::equipment::{ArmorDistribution, ArmorTrait, AttributeChoice, JewelryTrait, WeaponTrait};
use super::food::Food;
use super::mundus::MundusStone;
use super::weapon_type::WeaponType;
use super::weapon_enchant::WeaponEnchant;
use super::CharacterStats;
use crate::data::skill_trees::race::Race;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildMetadata {
    pub dps: f64,
    pub total_damage: f64,
    pub fight_duration: f64,
    pub bar1_skills: Vec<String>,
    pub bar2_skills: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buffed_stats: Option<CharacterStats>,
}

fn default_armor_distribution() -> ArmorDistribution {
    ArmorDistribution {
        light: 1,
        medium: 5,
        heavy: 1,
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildConfig {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub skills: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub champion_points: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub sets: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub classes: Vec<ClassName>,

    // Character
    #[serde(skip_serializing_if = "Option::is_none")]
    pub race: Option<Race>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attributes: Option<AttributeChoice>,

    // Weapons
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bar1_weapon: Option<WeaponType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bar2_weapon: Option<WeaponType>,

    // Gear traits (partial Vec = only pinned slots; free slots optimized)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub armor_traits: Vec<ArmorTrait>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub jewelry_traits: Vec<JewelryTrait>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub weapon_traits: Vec<WeaponTrait>,

    // Enchants
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bar1_enchant: Option<WeaponEnchant>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bar2_enchant: Option<WeaponEnchant>,

    // Armor
    #[serde(default = "default_armor_distribution")]
    pub armor: ArmorDistribution,

    // Buffs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mundus: Option<MundusStone>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub food: Option<Food>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub potion: Option<super::potion::Potion>,

    // Computed output
    #[serde(default)]
    pub character_stats: CharacterStats,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<BuildMetadata>,
}

impl Default for BuildConfig {
    fn default() -> Self {
        Self {
            skills: Vec::new(),
            champion_points: Vec::new(),
            sets: Vec::new(),
            classes: Vec::new(),
            race: None,
            attributes: None,
            bar1_weapon: None,
            bar2_weapon: None,
            armor_traits: Vec::new(),
            jewelry_traits: Vec::new(),
            weapon_traits: Vec::new(),
            bar1_enchant: None,
            bar2_enchant: None,
            armor: default_armor_distribution(),
            mundus: None,
            food: None,
            potion: None,
            character_stats: CharacterStats::default(),
            metadata: None,
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

impl BuildConfig {
    /// Compute character stats from gear configuration.
    /// Uses `bar1_weapon` for base damage calculation.
    pub fn compute_stats(&self) -> CharacterStats {
        let mut stats = CharacterStats::default();

        // 1. Attribute points
        match self.attributes {
            Some(AttributeChoice::Magicka) => stats.max_magicka += ATTRIBUTE_POINTS_BONUS,
            Some(AttributeChoice::Stamina) => stats.max_stamina += ATTRIBUTE_POINTS_BONUS,
            Some(AttributeChoice::None) | None => {}
        }

        // 2. Weapon base damage (replaces default 1,000)
        if let Some(weapon) = self.bar1_weapon {
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

            // 3. Weapon trait bonus (bar1 weapon trait = index 0, default Nirnhoned)
            match self
                .weapon_traits
                .first()
                .copied()
                .unwrap_or(WeaponTrait::Nirnhoned)
            {
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
        // Unspecified slots default to Divines (no infused bonus)
        let infused_armor_count = self
            .armor_traits
            .iter()
            .filter(|t| **t == ArmorTrait::Infused)
            .count() as f64;
        let non_infused_count = 7.0 - infused_armor_count;
        let armor_enchant_total = {
            let base_per_piece = TOTAL_ARMOR_ENCHANT / 7.0;
            non_infused_count * base_per_piece
                + infused_armor_count * base_per_piece * (1.0 + ARMOR_INFUSED_ENCHANT_BONUS)
        };

        // Apply enchant to the primary resource
        if self.attributes == Some(AttributeChoice::Magicka)
            || self.bar1_weapon.map_or(false, |w| w.is_destruction_staff())
        {
            stats.max_magicka += armor_enchant_total;
        } else {
            stats.max_stamina += armor_enchant_total;
        }

        // 5. Jewelry enchantments (3 pieces, weapon+spell damage)
        let mut jewelry_damage_total = 0.0;
        for jt in &self.jewelry_traits {
            let enchant = match jt {
                JewelryTrait::Infused => {
                    JEWELRY_ENCHANT_DAMAGE * (1.0 + JEWELRY_INFUSED_ENCHANT_BONUS)
                }
                _ => JEWELRY_ENCHANT_DAMAGE,
            };
            jewelry_damage_total += enchant;
        }
        stats.weapon_damage += jewelry_damage_total;
        stats.spell_damage += jewelry_damage_total;

        // 6. Jewelry trait bonuses (per-piece)
        for jt in &self.jewelry_traits {
            match jt {
                JewelryTrait::Bloodthirsty => {
                    let avg_per_piece = BLOODTHIRSTY_THRESHOLD * (BLOODTHIRSTY_MAX_PER_PIECE / 2.0);
                    stats.weapon_damage += avg_per_piece;
                    stats.spell_damage += avg_per_piece;
                }
                JewelryTrait::Robust => {
                    stats.max_stamina += JEWELRY_ROBUST_STAMINA;
                }
                JewelryTrait::Arcane => {
                    stats.max_magicka += JEWELRY_ARCANE_MAGICKA;
                }
                _ => {}
            }
        }

        // 7. Mundus stone (amplified by Divines)
        // Unspecified armor slots default to Divines for mundus calculation
        if let Some(mundus) = &self.mundus {
            let explicit_divines = self
                .armor_traits
                .iter()
                .filter(|t| **t == ArmorTrait::Divines)
                .count();
            let unspecified_slots = 7_usize.saturating_sub(self.armor_traits.len());
            let divines_count = (explicit_divines + unspecified_slots) as u8;
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
