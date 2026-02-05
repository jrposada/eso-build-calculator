use super::{BonusData, ClassName, SkillLineName};
use serde::{Deserialize, Serialize};

/// Passive data for skill line passives
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PassiveData {
    pub name: String,
    pub class_name: ClassName,
    pub skill_line: SkillLineName,
    pub bonuses: Vec<BonusData>,
}

impl PassiveData {
    pub fn new(
        name: impl Into<String>,
        class_name: ClassName,
        skill_line: SkillLineName,
        bonuses: Vec<BonusData>,
    ) -> Self {
        Self {
            name: name.into(),
            class_name,
            skill_line,
            bonuses,
        }
    }
}
