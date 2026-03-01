use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildMetadata {
    pub dps: f64,
    pub total_damage: f64,
    pub fight_duration: f64,
    pub bar1_skills: Vec<String>,
    pub bar2_skills: Vec<String>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<BuildMetadata>,
}
