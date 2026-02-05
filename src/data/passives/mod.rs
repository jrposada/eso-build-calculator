pub mod arcanist;
pub mod dragonknight;
pub mod nightblade;
pub mod sorcerer;
pub mod templar;
pub mod warden;
pub mod weapon;

use crate::domain::PassiveData;
use once_cell::sync::Lazy;

pub use arcanist::ARCANIST_PASSIVES;
pub use dragonknight::DRAGONKNIGHT_PASSIVES;
pub use nightblade::NIGHTBLADE_PASSIVES;
pub use sorcerer::SORCERER_PASSIVES;
pub use templar::TEMPLAR_PASSIVES;
pub use warden::WARDEN_PASSIVES;
pub use weapon::WEAPON_PASSIVES;

pub static ALL_CLASS_PASSIVES: Lazy<Vec<&'static PassiveData>> = Lazy::new(|| {
    let mut passives: Vec<&'static PassiveData> = Vec::new();
    passives.extend(DRAGONKNIGHT_PASSIVES.iter());
    passives.extend(SORCERER_PASSIVES.iter());
    passives.extend(NIGHTBLADE_PASSIVES.iter());
    passives.extend(TEMPLAR_PASSIVES.iter());
    passives.extend(WARDEN_PASSIVES.iter());
    passives.extend(ARCANIST_PASSIVES.iter());
    passives
});

pub static ALL_WEAPON_PASSIVES: Lazy<Vec<&'static PassiveData>> =
    Lazy::new(|| WEAPON_PASSIVES.iter().collect());

pub static ALL_PASSIVES: Lazy<Vec<&'static PassiveData>> = Lazy::new(|| {
    let mut passives: Vec<&'static PassiveData> = Vec::new();
    passives.extend(ALL_CLASS_PASSIVES.iter().copied());
    passives.extend(ALL_WEAPON_PASSIVES.iter().copied());
    passives
});
