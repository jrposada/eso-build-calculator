use crate::domain::{LightAttackData, WeaponType};

use super::skill_trees::weapon::{
    bow::bow_light_attacks::BOW_LIGHT_ATTACKS,
    destruction_staff::destruction_staff_light_attacks::DESTRUCTION_STAFF_LIGHT_ATTACKS,
    dual_wield::dual_wield_light_attacks::DUAL_WIELD_LIGHT_ATTACKS,
    two_handed::two_handed_light_attacks::TWO_HANDED_LIGHT_ATTACKS,
};

pub fn light_attack_for_weapon(weapon: WeaponType) -> &'static LightAttackData {
    let sources: &[&[LightAttackData]] = &[
        &BOW_LIGHT_ATTACKS,
        &DESTRUCTION_STAFF_LIGHT_ATTACKS,
        &DUAL_WIELD_LIGHT_ATTACKS,
        &TWO_HANDED_LIGHT_ATTACKS,
    ];

    sources
        .iter()
        .flat_map(|s| s.iter())
        .find(|la| la.weapon_type == weapon)
        .unwrap_or_else(|| panic!("No light attack data for weapon type {:?}", weapon))
}
