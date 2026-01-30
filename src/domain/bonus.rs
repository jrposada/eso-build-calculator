use crate::data::{BonusClassName, BonusType, ClassName, SkillLineName};
use serde::{Deserialize, Serialize};

/// Bonus data representing a stat modifier
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BonusData {
    /// Optional identifier for unique bonuses
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Display name of the bonus
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// How the bonus is applied
    pub class_name: BonusClassName,

    /// What stat the bonus affects
    pub bonus_type: BonusType,

    /// The bonus value (percentage or flat)
    pub value: f64,
}

impl BonusData {
    pub fn new(class_name: BonusClassName, bonus_type: BonusType, value: f64) -> Self {
        Self {
            id: None,
            name: None,
            class_name,
            bonus_type,
            value,
        }
    }

    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn with_id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }
}

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

/// Champion point bonus (always passive with a name)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChampionPointBonus {
    pub name: String,
    pub bonus_type: BonusType,
    pub value: f64,
}

impl ChampionPointBonus {
    pub fn new(name: impl Into<String>, bonus_type: BonusType, value: f64) -> Self {
        Self {
            name: name.into(),
            bonus_type,
            value,
        }
    }

    /// Convert to BonusData for calculations
    pub fn to_bonus_data(&self) -> BonusData {
        BonusData {
            id: None,
            name: Some(self.name.clone()),
            class_name: BonusClassName::Passive,
            bonus_type: self.bonus_type,
            value: self.value,
        }
    }
}
