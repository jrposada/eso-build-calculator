use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildConfig {
    pub skills: Vec<String>,
    pub champion_points: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bar1_weapon: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bar2_weapon: Option<String>,
}
