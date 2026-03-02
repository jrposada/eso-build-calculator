use serde::{Deserialize, Serialize};
use std::fmt;

use super::DamageFlags;

/// Weapon enchant (glyph) types for DPS calculations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum WeaponEnchant {
    /// Glyph of Flame: deals flame damage on proc
    Flame,
    /// Glyph of Poison: deals poison damage on proc
    Poison,
    /// Glyph of Shock: deals shock damage on proc
    Shock,
    /// Glyph of Weapon Damage (Berserker): adds W/SD instead of proc damage
    Berserker,
}

/// Base damage for CP160 gold weapon enchant glyphs.
/// These are flat damage values that proc on weapon attacks.
const ENCHANT_BASE_DAMAGE: f64 = 1_747.0;

/// Cooldown for weapon enchants:
/// - Non-Infused: 4s (but we model Infused by default)
/// - Infused front bar: ~2s effective (halved)
/// - Infused back bar: ~1s effective (halved + proc on ability cast)
const ENCHANT_COOLDOWN_INFUSED: f64 = 2.0;

impl WeaponEnchant {
    /// Base proc damage at CP160 gold quality.
    pub fn base_damage(&self) -> f64 {
        match self {
            WeaponEnchant::Flame | WeaponEnchant::Poison | WeaponEnchant::Shock => {
                ENCHANT_BASE_DAMAGE
            }
            WeaponEnchant::Berserker => 0.0, // No proc damage, adds W/SD instead
        }
    }

    /// Internal cooldown between procs (seconds), assuming Infused trait.
    pub fn cooldown(&self) -> f64 {
        ENCHANT_COOLDOWN_INFUSED
    }

    /// DamageFlags for the enchant proc's damage type.
    pub fn damage_flags(&self) -> DamageFlags {
        match self {
            WeaponEnchant::Flame => DamageFlags::FLAME | DamageFlags::DIRECT | DamageFlags::SINGLE_TARGET,
            WeaponEnchant::Poison => DamageFlags::PHYSICAL | DamageFlags::DIRECT | DamageFlags::SINGLE_TARGET,
            WeaponEnchant::Shock => DamageFlags::SHOCK | DamageFlags::DIRECT | DamageFlags::SINGLE_TARGET,
            WeaponEnchant::Berserker => DamageFlags::empty(),
        }
    }

    /// Status effect triggered by this enchant type.
    pub fn status_effect(&self) -> Option<EnchantStatusEffect> {
        match self {
            WeaponEnchant::Flame => Some(EnchantStatusEffect {
                name: "Burning",
                total_damage: 1_200.0,
                duration: 4.0,
                flags: DamageFlags::FLAME | DamageFlags::DOT | DamageFlags::SINGLE_TARGET,
            }),
            WeaponEnchant::Poison => Some(EnchantStatusEffect {
                name: "Poisoned",
                total_damage: 1_200.0,
                duration: 6.0,
                flags: DamageFlags::PHYSICAL | DamageFlags::DOT | DamageFlags::SINGLE_TARGET,
            }),
            WeaponEnchant::Shock => None, // Concussed is a debuff, not damage
            WeaponEnchant::Berserker => None,
        }
    }

    pub fn parse(s: &str) -> Result<WeaponEnchant, String> {
        match s.to_lowercase().as_str() {
            "flame" | "fire" => Ok(WeaponEnchant::Flame),
            "poison" => Ok(WeaponEnchant::Poison),
            "shock" | "lightning" => Ok(WeaponEnchant::Shock),
            "berserker" | "weapon-damage" => Ok(WeaponEnchant::Berserker),
            _ => Err(format!(
                "Unknown weapon enchant '{}'. Valid: flame, poison, shock, berserker",
                s
            )),
        }
    }
}

impl fmt::Display for WeaponEnchant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WeaponEnchant::Flame => write!(f, "Flame"),
            WeaponEnchant::Poison => write!(f, "Poison"),
            WeaponEnchant::Shock => write!(f, "Shock"),
            WeaponEnchant::Berserker => write!(f, "Berserker"),
        }
    }
}

/// Status effect data for enchant procs.
pub struct EnchantStatusEffect {
    pub name: &'static str,
    pub total_damage: f64,
    pub duration: f64,
    pub flags: DamageFlags,
}
