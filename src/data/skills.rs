use crate::domain::SkillData;
use once_cell::sync::Lazy;

use super::skill_trees::character_class::{
    arcanist::arcanist_skills::ARCANIST_SKILLS,
    dragonknight::dragonknight_skills::DRAGONKNIGHT_SKILLS,
    necromancer::necromancer_skills::NECROMANCER_SKILLS,
    nightblade::nightblade_skills::NIGHTBLADE_SKILLS, sorcerer::sorcerer_skills::SORCERER_SKILLS,
    templar::templar_skills::TEMPLAR_SKILLS, warden::warden_skills::WARDEN_SKILLS,
};
use super::skill_trees::guild::{
    fighters_guild::fighters_guild_skills::FIGHTERS_GUILD_SKILLS,
    mages_guild::mages_guild_skills::MAGES_GUILD_SKILLS,
    psijic_order::psijic_order_skills::PSIJIC_ORDER_SKILLS,
    undaunted::undaunted_skills::UNDAUNTED_SKILLS,
};
use super::skill_trees::weapon::{
    bow::bow_skills::BOW_SKILLS,
    destruction_staff::destruction_staff_skills::DESTRUCTION_STAFF_SKILLS,
    dual_wield::dual_wield_skills::DUAL_WIELD_SKILLS,
    two_handed::two_handed_skills::TWO_HANDED_SKILLS,
};

pub static ALL_SKILLS: Lazy<Vec<&'static SkillData>> = Lazy::new(|| {
    let sources: &[&Lazy<Vec<SkillData>>] = &[
        // Character classes
        &ARCANIST_SKILLS,
        &DRAGONKNIGHT_SKILLS,
        &NECROMANCER_SKILLS,
        &NIGHTBLADE_SKILLS,
        &SORCERER_SKILLS,
        &TEMPLAR_SKILLS,
        &WARDEN_SKILLS,
        // Guilds
        &FIGHTERS_GUILD_SKILLS,
        &MAGES_GUILD_SKILLS,
        &PSIJIC_ORDER_SKILLS,
        &UNDAUNTED_SKILLS,
        // Weapons
        &BOW_SKILLS,
        &DESTRUCTION_STAFF_SKILLS,
        &DUAL_WIELD_SKILLS,
        &TWO_HANDED_SKILLS,
    ];

    sources.iter().flat_map(|s| s.iter()).collect()
});
