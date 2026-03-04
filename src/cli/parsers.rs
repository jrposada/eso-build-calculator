use crate::domain::{ClassName, WeaponType};

pub fn parse_class_name(s: &str) -> Result<ClassName, String> {
    let s = s.trim();
    match s.to_lowercase().as_str() {
        "arcanist" => Ok(ClassName::Arcanist),
        "dragonknight" => Ok(ClassName::Dragonknight),
        "nightblade" => Ok(ClassName::Nightblade),
        "sorcerer" => Ok(ClassName::Sorcerer),
        "templar" => Ok(ClassName::Templar),
        "warden" => Ok(ClassName::Warden),
        _ => Err(format!(
            "Invalid class '{}'. Valid options: arcanist, dragonknight, nightblade, sorcerer, templar, warden",
            s
        )),
    }
}

pub fn parse_weapon(s: &str) -> Result<WeaponType, String> {
    let s = s.trim();
    // Try parsing as a specific weapon type first
    if let Ok(wt) = WeaponType::parse(s) {
        return Ok(wt);
    }
    // Fall back to skill line name → default weapon type
    match s.to_lowercase().as_str() {
        "destruction-staff" | "destructionstaff" => Ok(WeaponType::InfernoStaff),
        "dual-wield" | "dualwield" => Ok(WeaponType::DualWieldDagger),
        "two-handed" | "twohanded" => Ok(WeaponType::TwoHandedSword),
        // "bow" is already handled by WeaponType::parse above
        _ => Err(format!(
            "Invalid weapon '{}'. Valid options: bow, destruction-staff, dual-wield, two-handed, \
             two-handed-sword, two-handed-axe, two-handed-mace, dual-wield-sword, dual-wield-axe, \
             dual-wield-mace, dual-wield-dagger, inferno-staff, lightning-staff, ice-staff",
            s
        )),
    }
}
