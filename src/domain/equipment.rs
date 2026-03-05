use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;

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

/// Distribution of armor pieces across the three weights (light, medium, heavy).
/// Sum must be ≤ 7; free slots (7 - sum) are optimized by trying all valid allocations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ArmorDistribution {
    pub light: u8,
    pub medium: u8,
    pub heavy: u8,
}

impl ArmorDistribution {
    pub fn parse(s: &str) -> Result<Self, String> {
        let parts: Vec<&str> = s.split(',').collect();
        if parts.len() != 3 {
            return Err(format!(
                "Armor distribution requires exactly 3 comma-separated values (light,medium,heavy), got {}",
                parts.len()
            ));
        }
        let light: u8 = parts[0]
            .trim()
            .parse()
            .map_err(|_| format!("Invalid light armor count: '{}'", parts[0].trim()))?;
        let medium: u8 = parts[1]
            .trim()
            .parse()
            .map_err(|_| format!("Invalid medium armor count: '{}'", parts[1].trim()))?;
        let heavy: u8 = parts[2]
            .trim()
            .parse()
            .map_err(|_| format!("Invalid heavy armor count: '{}'", parts[2].trim()))?;
        let sum = light + medium + heavy;
        if sum > 7 {
            return Err(format!(
                "Armor piece total must be ≤ 7, got {} ({},{},{})",
                sum, light, medium, heavy
            ));
        }
        Ok(Self {
            light,
            medium,
            heavy,
        })
    }

    /// Returns the dominant armor weight (≥ 5 pieces), or `None` if no weight dominates.
    pub fn dominant_weight(&self) -> Option<ArmorWeight> {
        if self.light >= 5 {
            Some(ArmorWeight::Light)
        } else if self.medium >= 5 {
            Some(ArmorWeight::Medium)
        } else if self.heavy >= 5 {
            Some(ArmorWeight::Heavy)
        } else {
            None
        }
    }

    /// Count of non-zero weights (for Undaunted Mettle).
    pub fn type_count(&self) -> u8 {
        (self.light > 0) as u8 + (self.medium > 0) as u8 + (self.heavy > 0) as u8
    }

    /// Number of unassigned armor slots.
    pub fn free_slots(&self) -> u8 {
        7 - self.light - self.medium - self.heavy
    }

    /// All valid distributions filling free slots to sum=7.
    /// Returns `vec![self]` when sum == 7.
    pub fn completions(&self) -> Vec<ArmorDistribution> {
        let free = self.free_slots();
        if free == 0 {
            return vec![*self];
        }
        let mut results = Vec::new();
        for add_l in 0..=free {
            for add_m in 0..=(free - add_l) {
                let add_h = free - add_l - add_m;
                results.push(ArmorDistribution {
                    light: self.light + add_l,
                    medium: self.medium + add_m,
                    heavy: self.heavy + add_h,
                });
            }
        }
        results
    }
}

impl fmt::Display for ArmorDistribution {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{},{},{}", self.light, self.medium, self.heavy)
    }
}

impl Serialize for ArmorDistribution {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for ArmorDistribution {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        ArmorDistribution::parse(&s).map_err(serde::de::Error::custom)
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub enum AttributeChoice {
    #[default]
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
