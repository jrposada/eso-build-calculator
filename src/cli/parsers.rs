use crate::data::bonuses::CHAMPION_POINTS;
use crate::data::sets::ALL_SETS;
use crate::data::skills::ALL_SKILLS;
use crate::domain::{BonusData, ClassName, SetData, SkillData, WeaponType};

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

pub fn parse_champion_point(s: &str) -> Result<BonusData, String> {
    let s = s.trim();
    // Normalize input: replace hyphens with spaces for matching
    let normalized = s.to_lowercase().replace('-', " ");

    CHAMPION_POINTS
        .iter()
        .find(|cp| cp.name.to_lowercase().replace('-', " ") == normalized)
        .cloned()
        .ok_or_else(|| {
            format!(
                "Invalid champion point '{}'. Valid options: backstabber, biting-aura, deadly-aim, \
                exploiter, fighting-finesse, master-at-arms, thaumaturge",
                s
            )
        })
}

pub fn parse_set(s: &str) -> Result<&'static SetData, String> {
    let s = s.trim();
    let normalized = s.to_lowercase().replace('-', " ");

    ALL_SETS
        .iter()
        .find(|set| set.name.to_lowercase().replace('-', " ") == normalized)
        .copied()
        .ok_or_else(|| {
            let names: Vec<_> = ALL_SETS.iter().map(|s| s.name.as_str()).collect();
            format!("Invalid set '{}'. Valid options: {}", s, names.join(", "))
        })
}

pub fn parse_skill(s: &str) -> Result<&'static SkillData, String> {
    let s = s.trim();
    // Normalize input: replace hyphens with spaces for matching
    let normalized = s.to_lowercase().replace('-', " ");

    ALL_SKILLS
        .iter()
        .find(|skill| skill.name.to_lowercase().replace('-', " ") == normalized)
        .copied()
        .ok_or_else(|| format!("Invalid skill name '{}'", s))
}
