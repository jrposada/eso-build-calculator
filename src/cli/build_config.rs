use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildConfig {
    pub skills: Vec<String>,
    pub champion_points: Vec<String>,
}
