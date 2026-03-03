use crate::domain::CharacterStats;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildMetadata {
    pub dps: f64,
    pub total_damage: f64,
    pub fight_duration: f64,
    pub bar1_skills: Vec<String>,
    pub bar2_skills: Vec<String>,
}

fn default_true() -> bool {
    true
}

fn default_armor_types() -> u8 {
    3
}

fn default_avg_resource_pct() -> f64 {
    50.0
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildConfig {
    pub skills: Vec<String>,
    pub champion_points: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub sets: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bar1_weapon: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bar2_weapon: Option<String>,
    #[serde(default)]
    pub character_stats: CharacterStats,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub race: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mundus: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub food: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub armor_trait: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jewelry_trait: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weapon_trait: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bar1_enchant: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bar2_enchant: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub armor_weight: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub potion: Option<String>,
    #[serde(default = "default_armor_types")]
    pub armor_types: u8,
    #[serde(default = "default_avg_resource_pct")]
    pub avg_resource_pct: f64,
    #[serde(default = "default_true")]
    pub trial: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<BuildMetadata>,
}
