use crate::data::sets::ALL_SETS;
use crate::domain::{SetData, SetType};
use std::collections::HashMap;

#[derive(Debug, Clone, Default)]
pub struct SetsServiceOptions {
    pub sets: Option<Vec<&'static SetData>>,
}

pub struct SetsService {
    sets_by_type: HashMap<SetType, Vec<&'static SetData>>,
}

impl SetsService {
    pub fn new(options: SetsServiceOptions) -> Self {
        let sets = match options.sets {
            Some(s) => s,
            None => ALL_SETS.iter().copied().collect(),
        };

        let mut sets_by_type: HashMap<SetType, Vec<&'static SetData>> = HashMap::new();
        for set in sets {
            sets_by_type.entry(set.set_type).or_default().push(set);
        }

        Self { sets_by_type }
    }

    pub fn get_set_by_name(&self, name: &str) -> Option<&'static SetData> {
        self.sets_by_type
            .values()
            .flatten()
            .find(|s| s.name.eq_ignore_ascii_case(name))
            .copied()
    }

    pub fn get_sets_by_type(&self, set_type: SetType) -> Vec<&'static SetData> {
        self.sets_by_type
            .get(&set_type)
            .cloned()
            .unwrap_or_default()
    }

    pub fn all_sets(&self) -> Vec<&'static SetData> {
        self.sets_by_type.values().flatten().copied().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sets_service_new() {
        let service = SetsService::new(SetsServiceOptions::default());
        let all = service.all_sets();
        assert!(!all.is_empty());
    }

    #[test]
    fn test_get_set_by_name() {
        let service = SetsService::new(SetsServiceOptions::default());
        let set = service.get_set_by_name("Mother's Sorrow");
        assert!(set.is_some());
        assert_eq!(set.unwrap().name, "Mother's Sorrow");
    }

    #[test]
    fn test_get_set_by_name_case_insensitive() {
        let service = SetsService::new(SetsServiceOptions::default());
        let set = service.get_set_by_name("mother's sorrow");
        assert!(set.is_some());
    }

    #[test]
    fn test_get_set_by_name_not_found() {
        let service = SetsService::new(SetsServiceOptions::default());
        let set = service.get_set_by_name("Nonexistent Set");
        assert!(set.is_none());
    }

    #[test]
    fn test_get_sets_by_type() {
        let service = SetsService::new(SetsServiceOptions::default());
        let normal = service.get_sets_by_type(SetType::Normal);
        assert!(!normal.is_empty());
        assert!(normal.iter().all(|s| s.set_type == SetType::Normal));
    }

    #[test]
    fn test_get_monster_sets() {
        let service = SetsService::new(SetsServiceOptions::default());
        let monster = service.get_sets_by_type(SetType::Monster);
        assert!(!monster.is_empty());
        assert!(monster.iter().all(|s| s.set_type == SetType::Monster));
    }
}
