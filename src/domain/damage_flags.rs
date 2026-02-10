use super::BonusTarget;
use bitflags::bitflags;
use serde::{Deserialize, Serialize};
use std::fmt;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct DamageFlags: u16 {
        // Element (bits 0-7)
        const MAGIC    = 0b0000_0000_0000_0001;
        const PHYSICAL = 0b0000_0000_0000_0010;
        const FLAME    = 0b0000_0000_0000_0100;
        const FROST    = 0b0000_0000_0000_1000;
        const SHOCK    = 0b0000_0000_0001_0000;
        const POISON   = 0b0000_0000_0010_0000;
        const DISEASE  = 0b0000_0000_0100_0000;
        const BLEED    = 0b0000_0000_1000_0000;

        // Target (bits 8-9)
        const SINGLE_TARGET = 0b0000_0001_0000_0000;
        const AOE           = 0b0000_0010_0000_0000;

        // Delivery (bits 10-11)
        const DIRECT = 0b0000_0100_0000_0000;
        const DOT    = 0b0000_1000_0000_0000;

        // Range (bits 12-14) - from UESP, no calculation impact yet
        const MELEE   = 0b0001_0000_0000_0000;
        const RANGED  = 0b0010_0000_0000_0000;
        const CHANNEL = 0b0100_0000_0000_0000;
    }
}

// Mask constants for flag categories
const ELEMENT_MASK: u16 = 0b0000_0000_1111_1111;
const TARGET_MASK: u16 = 0b0000_0011_0000_0000;

impl DamageFlags {
    /// Check if this damage component's flags match a given bonus target
    pub fn matches_bonus_target(&self, target: BonusTarget) -> bool {
        match target {
            BonusTarget::DirectDamage => self.contains(DamageFlags::DIRECT),
            BonusTarget::DotDamage => self.contains(DamageFlags::DOT),
            BonusTarget::SingleDamage => self.contains(DamageFlags::SINGLE_TARGET),
            BonusTarget::AoeDamage => self.contains(DamageFlags::AOE),
            BonusTarget::FlameDamage => self.contains(DamageFlags::FLAME),
            BonusTarget::FrostDamage => self.contains(DamageFlags::FROST),
            BonusTarget::ShockDamage => self.contains(DamageFlags::SHOCK),
            BonusTarget::PhysicalDamage => self.contains(DamageFlags::PHYSICAL),
            BonusTarget::Damage => true,
            _ => false,
        }
    }

    /// Get element display name
    pub fn element_display(&self) -> &'static str {
        let element_bits = self.bits() & ELEMENT_MASK;
        match element_bits {
            x if x == DamageFlags::MAGIC.bits() => "magic",
            x if x == DamageFlags::PHYSICAL.bits() => "physical",
            x if x == DamageFlags::FLAME.bits() => "flame",
            x if x == DamageFlags::FROST.bits() => "frost",
            x if x == DamageFlags::SHOCK.bits() => "shock",
            x if x == DamageFlags::POISON.bits() => "poison",
            x if x == DamageFlags::DISEASE.bits() => "disease",
            x if x == DamageFlags::BLEED.bits() => "bleed",
            _ => "unknown",
        }
    }

    /// Get target display name
    pub fn target_display(&self) -> &'static str {
        let target_bits = self.bits() & TARGET_MASK;
        match target_bits {
            x if x == DamageFlags::SINGLE_TARGET.bits() => "single",
            x if x == DamageFlags::AOE.bits() => "aoe",
            _ => "unknown",
        }
    }

    // Convenience constructors for common combos

    pub fn magic_single() -> Self {
        Self::MAGIC | Self::SINGLE_TARGET
    }

    pub fn magic_aoe() -> Self {
        Self::MAGIC | Self::AOE
    }

    pub fn physical_single() -> Self {
        Self::PHYSICAL | Self::SINGLE_TARGET
    }

    pub fn physical_aoe() -> Self {
        Self::PHYSICAL | Self::AOE
    }

    pub fn flame_single() -> Self {
        Self::FLAME | Self::SINGLE_TARGET
    }

    pub fn flame_aoe() -> Self {
        Self::FLAME | Self::AOE
    }

    pub fn frost_single() -> Self {
        Self::FROST | Self::SINGLE_TARGET
    }

    pub fn frost_aoe() -> Self {
        Self::FROST | Self::AOE
    }

    pub fn shock_single() -> Self {
        Self::SHOCK | Self::SINGLE_TARGET
    }

    pub fn shock_aoe() -> Self {
        Self::SHOCK | Self::AOE
    }

    pub fn poison_single() -> Self {
        Self::POISON | Self::SINGLE_TARGET
    }

    pub fn poison_aoe() -> Self {
        Self::POISON | Self::AOE
    }

    pub fn disease_single() -> Self {
        Self::DISEASE | Self::SINGLE_TARGET
    }

    pub fn disease_aoe() -> Self {
        Self::DISEASE | Self::AOE
    }

    pub fn bleed_single() -> Self {
        Self::BLEED | Self::SINGLE_TARGET
    }

    pub fn bleed_aoe() -> Self {
        Self::BLEED | Self::AOE
    }
}

impl fmt::Display for DamageFlags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.element_display(), self.target_display())
    }
}
