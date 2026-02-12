mod arcanist;
mod dragonknight;
mod necromancer;
mod nightblade;
mod sorcerer;
mod templar;
mod warden;
mod weapon;

pub use arcanist::ARCANIST_SKILLS;
pub use dragonknight::DRAGONKNIGHT_SKILLS;
pub use necromancer::NECROMANCER_SKILLS;
pub use nightblade::NIGHTBLADE_SKILLS;
pub use sorcerer::SORCERER_SKILLS;
pub use templar::TEMPLAR_SKILLS;
pub use warden::WARDEN_SKILLS;
pub use weapon::WEAPON_SKILLS;

use crate::domain::SkillData;
use once_cell::sync::Lazy;

pub static ALL_CLASS_SKILLS: Lazy<Vec<&'static SkillData>> = Lazy::new(|| {
    let mut skills = Vec::new();
    skills.extend(DRAGONKNIGHT_SKILLS.iter());
    skills.extend(NECROMANCER_SKILLS.iter());
    skills.extend(SORCERER_SKILLS.iter());
    skills.extend(NIGHTBLADE_SKILLS.iter());
    skills.extend(TEMPLAR_SKILLS.iter());
    skills.extend(WARDEN_SKILLS.iter());
    skills.extend(ARCANIST_SKILLS.iter());
    skills
});

pub static ALL_WEAPON_SKILLS: Lazy<Vec<&'static SkillData>> =
    Lazy::new(|| WEAPON_SKILLS.iter().collect());

pub static ALL_SKILLS: Lazy<Vec<&'static SkillData>> = Lazy::new(|| {
    let mut skills = Vec::new();
    skills.extend(ALL_CLASS_SKILLS.iter().copied());
    skills.extend(ALL_WEAPON_SKILLS.iter().copied());
    skills
});
