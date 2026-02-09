use crate::data::bonuses::CHAMPION_POINTS;
use crate::data::skills::ALL_SKILLS;
use crate::domain::{BonusData, ClassName, SkillData, SkillLineName};

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

pub fn parse_weapon_skill_line(s: &str) -> Result<SkillLineName, String> {
    let s = s.trim();
    match s.to_lowercase().as_str() {
        "bow" => Ok(SkillLineName::Bow),
        "destruction-staff" | "destructionstaff" => Ok(SkillLineName::DestructionStaff),
        "dual-wield" | "dualwield" => Ok(SkillLineName::DualWield),
        "two-handed" | "twohanded" => Ok(SkillLineName::TwoHanded),
        _ => Err(format!(
            "Invalid weapon skill line '{}'. Valid options: bow, destruction-staff, dual-wield, two-handed",
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
