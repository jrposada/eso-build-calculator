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
}

impl fmt::Display for WeaponType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WeaponType::TwoHandedSword => write!(f, "Two-Handed Sword"),
            WeaponType::TwoHandedAxe => write!(f, "Two-Handed Axe"),
            WeaponType::TwoHandedMace => write!(f, "Two-Handed Mace"),
            WeaponType::DualWieldSword => write!(f, "Dual Wield Sword"),
            WeaponType::DualWieldAxe => write!(f, "Dual WieldAxe"),
            WeaponType::DualWieldMace => write!(f, "Dual WieldMace"),
            WeaponType::DualWieldDagger => write!(f, "Dual Wield Dagger"),
            WeaponType::InfernoStaff => write!(f, "Inferno Staff"),
            WeaponType::LightningStaff => write!(f, "Lightning Staff"),
            WeaponType::IceStaff => write!(f, "Ice Staff"),
            WeaponType::Bow => write!(f, "Bow"),
        }
    }
}
