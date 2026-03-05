use super::{BonusData, SkillLineName, SkillTree};
use serde::{Deserialize, Serialize};

/// Passive data for skill line passives
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PassiveData {
    pub name: String,
    pub class_name: SkillTree,
    pub skill_line: SkillLineName,
    pub skill_id: Option<u32>,
    pub bonuses: Vec<BonusData>,
}

impl PassiveData {
    pub fn new(
        name: impl Into<String>,
        class_name: SkillTree,
        skill_line: SkillLineName,
        bonuses: Vec<BonusData>,
    ) -> Self {
        Self {
            name: name.into(),
            class_name,
            skill_line,
            skill_id: None,
            bonuses,
        }
    }

    pub fn with_skill_id(mut self, skill_id: u32) -> Self {
        self.skill_id = Some(skill_id);
        self
    }
}
