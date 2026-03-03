use crate::data::skill_trees::armor::{
    light::light_passives::LIGHT_ARMOR_PASSIVES, medium::medium_passives::MEDIUM_ARMOR_PASSIVES,
};
use crate::domain::{ArmorWeight, BonusData};

pub fn armor_passives(armor_weight: ArmorWeight) -> Vec<BonusData> {
    match armor_weight {
        ArmorWeight::Light => LIGHT_ARMOR_PASSIVES.clone(),
        ArmorWeight::Medium => MEDIUM_ARMOR_PASSIVES.clone(),
        ArmorWeight::Heavy => vec![],
    }
}
