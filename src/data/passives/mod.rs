pub mod armor;

use crate::domain::PassiveData;
use once_cell::sync::Lazy;

use super::skill_trees::character_class::{
    arcanist::arcanist_passives::ARCANIST_PASSIVES,
    dragonknight::dragonknight_passives::DRAGONKNIGHT_PASSIVES,
    necromancer::necromancer_passives::NECROMANCER_PASSIVES,
    nightblade::nightblade_passives::NIGHTBLADE_PASSIVES,
    sorcerer::sorcerer_passives::SORCERER_PASSIVES,
    templar::templar_passives::TEMPLAR_PASSIVES,
    warden::warden_passives::WARDEN_PASSIVES,
};
use super::skill_trees::guild::{
    fighters_guild::fighters_guild_passives::FIGHTERS_GUILD_PASSIVES,
    mages_guild::mages_guild_passives::MAGES_GUILD_PASSIVES,
    psijic_order::psijic_order_passives::PSIJIC_ORDER_PASSIVES,
    undaunted::undaunted_passives::UNDAUNTED_PASSIVES,
};
use super::skill_trees::weapon::{
    bow::bow_passives::BOW_PASSIVES,
    destruction_staff::destruction_staff_passives::DESTRUCTION_STAFF_PASSIVES,
    dual_wield::dual_wield_passives::DUAL_WIELD_PASSIVES,
    two_handed::two_handed_passives::TWO_HANDED_PASSIVES,
};

pub use super::skill_trees::guild::undaunted::undaunted_passives::undaunted_mettle_bonuses;

pub static ALL_PASSIVES: Lazy<Vec<&'static PassiveData>> = Lazy::new(|| {
    let sources: &[&Lazy<Vec<PassiveData>>] = &[
        // Character classes
        &ARCANIST_PASSIVES,
        &DRAGONKNIGHT_PASSIVES,
        &NECROMANCER_PASSIVES,
        &NIGHTBLADE_PASSIVES,
        &SORCERER_PASSIVES,
        &TEMPLAR_PASSIVES,
        &WARDEN_PASSIVES,
        // Guilds
        &FIGHTERS_GUILD_PASSIVES,
        &MAGES_GUILD_PASSIVES,
        &PSIJIC_ORDER_PASSIVES,
        &UNDAUNTED_PASSIVES,
        // Weapons
        &BOW_PASSIVES,
        &DESTRUCTION_STAFF_PASSIVES,
        &DUAL_WIELD_PASSIVES,
        &TWO_HANDED_PASSIVES,
    ];

    sources.iter().flat_map(|s| s.iter()).collect()
});
