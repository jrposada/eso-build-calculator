use serde::{Deserialize, Serialize};
use std::fmt;

use super::set_proc::SetProcEffect;
use super::BonusData;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SetType {
    Normal,
    Monster,
    Mythic,
    Arena,
}

impl SetType {
    /// Maximum number of pieces that can be equipped for this set type.
    pub fn max_pieces(&self) -> u8 {
        match self {
            SetType::Normal => 5,
            SetType::Monster => 2,
            SetType::Mythic => 1,
            SetType::Arena => 2,
        }
    }
}

impl fmt::Display for SetType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            SetType::Normal => "Normal",
            SetType::Monster => "Monster",
            SetType::Mythic => "Mythic",
            SetType::Arena => "Arena",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, Clone)]
pub struct SetBonusThreshold {
    pub piece_count: u8,
    pub bonuses: Vec<BonusData>,
    pub proc_effects: Vec<SetProcEffect>,
}

#[derive(Debug, Clone)]
pub struct SetData {
    pub name: String,
    pub set_type: SetType,
    pub item_slots: Vec<String>,
    pub thresholds: Vec<SetBonusThreshold>,
}

impl SetData {
    pub fn new(name: impl Into<String>, set_type: SetType) -> Self {
        Self {
            name: name.into(),
            set_type,
            item_slots: Vec::new(),
            thresholds: Vec::new(),
        }
    }

    pub fn with_item_slots(mut self, slots: Vec<&str>) -> Self {
        self.item_slots = slots.into_iter().map(|s| s.to_string()).collect();
        self
    }

    pub fn with_threshold(mut self, piece_count: u8, bonuses: Vec<BonusData>) -> Self {
        self.thresholds.push(SetBonusThreshold {
            piece_count,
            bonuses,
            proc_effects: Vec::new(),
        });
        self
    }

    pub fn with_proc_effects(mut self, piece_count: u8, effects: Vec<SetProcEffect>) -> Self {
        if let Some(threshold) = self
            .thresholds
            .iter_mut()
            .find(|t| t.piece_count == piece_count)
        {
            threshold.proc_effects = effects;
        } else {
            self.thresholds.push(SetBonusThreshold {
                piece_count,
                bonuses: Vec::new(),
                proc_effects: effects,
            });
        }
        self
    }

    pub fn add_proc_effects(&mut self, piece_count: u8, effects: Vec<SetProcEffect>) {
        if let Some(threshold) = self
            .thresholds
            .iter_mut()
            .find(|t| t.piece_count == piece_count)
        {
            threshold.proc_effects = effects;
        } else {
            self.thresholds.push(SetBonusThreshold {
                piece_count,
                bonuses: Vec::new(),
                proc_effects: effects,
            });
        }
    }

    /// Returns cumulative bonuses at the given piece count.
    /// E.g., at 5 pieces returns all bonuses from thresholds with piece_count <= 5.
    pub fn bonuses_at(&self, piece_count: u8) -> Vec<&BonusData> {
        self.thresholds
            .iter()
            .filter(|t| t.piece_count <= piece_count)
            .flat_map(|t| t.bonuses.iter())
            .collect()
    }

    /// Returns cumulative proc effects at the given piece count.
    pub fn proc_effects_at(&self, piece_count: u8) -> Vec<&SetProcEffect> {
        self.thresholds
            .iter()
            .filter(|t| t.piece_count <= piece_count)
            .flat_map(|t| t.proc_effects.iter())
            .collect()
    }
}

impl SetData {
    /// Splits a slice of sets into (normal+arena, monster, mythic) groups.
    pub fn split_by_type(
        sets: &[&'static SetData],
    ) -> (
        Vec<&'static SetData>,
        Vec<&'static SetData>,
        Vec<&'static SetData>,
    ) {
        let mut normals = Vec::new();
        let mut monsters = Vec::new();
        let mut mythics = Vec::new();
        for &set in sets {
            match set.set_type {
                SetType::Normal | SetType::Arena => normals.push(set),
                SetType::Monster => monsters.push(set),
                SetType::Mythic => mythics.push(set),
            }
        }
        (normals, monsters, mythics)
    }
}

impl fmt::Display for SetData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({})", self.name, self.set_type)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::{BonusSource, BonusTarget, BonusTrigger, BonusValue};

    fn test_set() -> SetData {
        SetData::new("Test Set", SetType::Normal)
            .with_threshold(
                2,
                vec![BonusData::new(
                    "Test 2pc",
                    BonusSource::GearSet,
                    BonusTrigger::Passive,
                    BonusValue::new("Max Magicka", BonusTarget::MaxMagickaFlat, 1096.0),
                )],
            )
            .with_threshold(
                3,
                vec![BonusData::new(
                    "Test 3pc",
                    BonusSource::GearSet,
                    BonusTrigger::Passive,
                    BonusValue::new("Spell Critical", BonusTarget::SpellCriticalRating, 657.0),
                )],
            )
            .with_threshold(
                4,
                vec![BonusData::new(
                    "Test 4pc",
                    BonusSource::GearSet,
                    BonusTrigger::Passive,
                    BonusValue::new("Max Magicka", BonusTarget::MaxMagickaFlat, 1096.0),
                )],
            )
            .with_threshold(
                5,
                vec![BonusData::new(
                    "Test 5pc",
                    BonusSource::GearSet,
                    BonusTrigger::Passive,
                    BonusValue::new("Spell Critical", BonusTarget::SpellCriticalRating, 1528.0),
                )],
            )
    }

    #[test]
    fn test_set_type_max_pieces() {
        assert_eq!(SetType::Normal.max_pieces(), 5);
        assert_eq!(SetType::Monster.max_pieces(), 2);
        assert_eq!(SetType::Mythic.max_pieces(), 1);
        assert_eq!(SetType::Arena.max_pieces(), 2);
    }

    #[test]
    fn test_bonuses_at_full_set() {
        let set = test_set();
        let bonuses = set.bonuses_at(5);
        assert_eq!(bonuses.len(), 4);
    }

    #[test]
    fn test_bonuses_at_partial_set() {
        let set = test_set();
        let bonuses = set.bonuses_at(3);
        assert_eq!(bonuses.len(), 2);
    }

    #[test]
    fn test_bonuses_at_below_minimum() {
        let set = test_set();
        let bonuses = set.bonuses_at(1);
        assert_eq!(bonuses.len(), 0);
    }

    #[test]
    fn test_bonuses_at_exact_threshold() {
        let set = test_set();
        let bonuses = set.bonuses_at(2);
        assert_eq!(bonuses.len(), 1);
        assert_eq!(bonuses[0].name, "Test 2pc");
    }

    #[test]
    fn test_bonuses_cumulative() {
        let set = test_set();
        let at_4 = set.bonuses_at(4);
        assert_eq!(at_4.len(), 3);
        // Should include 2pc, 3pc, and 4pc bonuses
        let names: Vec<&str> = at_4.iter().map(|b| b.name.as_str()).collect();
        assert!(names.contains(&"Test 2pc"));
        assert!(names.contains(&"Test 3pc"));
        assert!(names.contains(&"Test 4pc"));
    }
}
