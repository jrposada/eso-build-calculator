use serde::{Deserialize, Serialize};

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
