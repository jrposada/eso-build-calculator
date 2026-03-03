pub mod heavy;
pub mod light;
pub mod medium;

use crate::domain::{ArmorWeight, BonusData};
use light::light_passives::LIGHT_ARMOR_PASSIVES;
use medium::medium_passives::MEDIUM_ARMOR_PASSIVES;

pub fn armor_passives(armor_weight: ArmorWeight) -> Vec<BonusData> {
    match armor_weight {
        ArmorWeight::Light => LIGHT_ARMOR_PASSIVES.clone(),
        ArmorWeight::Medium => MEDIUM_ARMOR_PASSIVES.clone(),
        ArmorWeight::Heavy => vec![],
    }
}
