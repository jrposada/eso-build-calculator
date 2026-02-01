use serde::{Deserialize, Serialize};

use crate::domain::{DotDamage, HitDamage};

/// Skill damage containing hits and dots
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct SkillDamage {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hits: Option<Vec<HitDamage>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dots: Option<Vec<DotDamage>>,
}

impl SkillDamage {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_hits(mut self, hits: Vec<HitDamage>) -> Self {
        self.hits = Some(hits);
        self
    }

    pub fn with_dots(mut self, dots: Vec<DotDamage>) -> Self {
        self.dots = Some(dots);
        self
    }

    pub fn has_damage(&self) -> bool {
        let has_hits = self.hits.as_ref().is_some_and(|h| !h.is_empty());
        let has_dots = self.dots.as_ref().is_some_and(|d| !d.is_empty());
        has_hits || has_dots
    }
}
