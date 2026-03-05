use serde::{Deserialize, Serialize};
use std::fmt;

use super::skill_line_name::SkillLineName;
use super::weapon_type::WeaponType;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum WeaponChoice {
    SkillLine(SkillLineName),
    Specific(WeaponType),
}

impl WeaponChoice {
    pub fn parse(s: &str) -> Result<WeaponChoice, String> {
        // Try specific weapon type first
        if let Ok(wt) = WeaponType::parse(s) {
            return Ok(WeaponChoice::Specific(wt));
        }
        // Fall back to skill line names
        match s.trim().to_lowercase().as_str() {
            "destruction-staff" | "destructionstaff" => {
                Ok(WeaponChoice::SkillLine(SkillLineName::DestructionStaff))
            }
            "dual-wield" | "dualwield" => Ok(WeaponChoice::SkillLine(SkillLineName::DualWield)),
            "two-handed" | "twohanded" => Ok(WeaponChoice::SkillLine(SkillLineName::TwoHanded)),
            "bow" => Ok(WeaponChoice::Specific(WeaponType::Bow)),
            _ => Err(format!(
                "Invalid weapon '{}'. Valid options: bow, destruction-staff, dual-wield, two-handed, \
                 two-handed-sword, two-handed-axe, two-handed-mace, dual-wield-sword, dual-wield-axe, \
                 dual-wield-mace, dual-wield-dagger, inferno-staff, lightning-staff, ice-staff",
                s
            )),
        }
    }

    pub fn skill_line(&self) -> SkillLineName {
        match self {
            WeaponChoice::SkillLine(sl) => *sl,
            WeaponChoice::Specific(wt) => wt.skill_line(),
        }
    }

    pub fn weapon_type(&self) -> Option<WeaponType> {
        match self {
            WeaponChoice::SkillLine(_) => None,
            WeaponChoice::Specific(wt) => Some(*wt),
        }
    }
}

impl fmt::Display for WeaponChoice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WeaponChoice::SkillLine(sl) => write!(f, "{}", sl),
            WeaponChoice::Specific(wt) => write!(f, "{}", wt),
        }
    }
}
