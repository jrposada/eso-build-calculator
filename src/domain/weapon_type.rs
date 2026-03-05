use super::SkillLineName;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum WeaponType {
    TwoHandedSword,
    TwoHandedAxe,
    TwoHandedMace,
    DualWieldSword,
    DualWieldAxe,
    DualWieldMace,
    DualWieldDagger,
    InfernoStaff,
    LightningStaff,
    IceStaff,
    Bow,
}

impl WeaponType {
    pub fn is_two_handed(&self) -> bool {
        matches!(
            self,
            WeaponType::TwoHandedSword | WeaponType::TwoHandedAxe | WeaponType::TwoHandedMace
        )
    }

    pub fn is_dual_wield(&self) -> bool {
        matches!(
            self,
            WeaponType::DualWieldSword
                | WeaponType::DualWieldAxe
                | WeaponType::DualWieldMace
                | WeaponType::DualWieldDagger
        )
    }

    pub fn is_destruction_staff(&self) -> bool {
        matches!(
            self,
            WeaponType::InfernoStaff | WeaponType::LightningStaff | WeaponType::IceStaff
        )
    }

    pub fn is_bow(&self) -> bool {
        matches!(self, WeaponType::Bow)
    }

    pub fn skill_line(&self) -> SkillLineName {
        match self {
            WeaponType::TwoHandedSword | WeaponType::TwoHandedAxe | WeaponType::TwoHandedMace => {
                SkillLineName::TwoHanded
            }
            WeaponType::DualWieldSword
            | WeaponType::DualWieldAxe
            | WeaponType::DualWieldMace
            | WeaponType::DualWieldDagger => SkillLineName::DualWield,
            WeaponType::InfernoStaff | WeaponType::LightningStaff | WeaponType::IceStaff => {
                SkillLineName::DestructionStaff
            }
            WeaponType::Bow => SkillLineName::Bow,
        }
    }

    pub fn parse(s: &str) -> Result<WeaponType, String> {
        match s.to_lowercase().replace(' ', "-").as_str() {
            "two-handed-sword" => Ok(WeaponType::TwoHandedSword),
            "two-handed-axe" => Ok(WeaponType::TwoHandedAxe),
            "two-handed-mace" => Ok(WeaponType::TwoHandedMace),
            "dual-wield-sword" => Ok(WeaponType::DualWieldSword),
            "dual-wield-axe" => Ok(WeaponType::DualWieldAxe),
            "dual-wield-mace" => Ok(WeaponType::DualWieldMace),
            "dual-wield-dagger" => Ok(WeaponType::DualWieldDagger),
            "inferno-staff" => Ok(WeaponType::InfernoStaff),
            "lightning-staff" => Ok(WeaponType::LightningStaff),
            "ice-staff" => Ok(WeaponType::IceStaff),
            "bow" => Ok(WeaponType::Bow),
            _ => Err(format!(
                "Unknown weapon type '{}'. Valid types: two-handed-sword, two-handed-axe, \
                 two-handed-mace, dual-wield-sword, dual-wield-axe, dual-wield-mace, \
                 dual-wield-dagger, inferno-staff, lightning-staff, ice-staff, bow",
                s
            )),
        }
    }
}

impl fmt::Display for WeaponType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WeaponType::TwoHandedSword => write!(f, "Two-Handed Sword"),
            WeaponType::TwoHandedAxe => write!(f, "Two-Handed Axe"),
            WeaponType::TwoHandedMace => write!(f, "Two-Handed Mace"),
            WeaponType::DualWieldSword => write!(f, "Dual Wield Sword"),
            WeaponType::DualWieldAxe => write!(f, "Dual Wield Axe"),
            WeaponType::DualWieldMace => write!(f, "Dual Wield Mace"),
            WeaponType::DualWieldDagger => write!(f, "Dual Wield Dagger"),
            WeaponType::InfernoStaff => write!(f, "Inferno Staff"),
            WeaponType::LightningStaff => write!(f, "Lightning Staff"),
            WeaponType::IceStaff => write!(f, "Ice Staff"),
            WeaponType::Bow => write!(f, "Bow"),
        }
    }
}
