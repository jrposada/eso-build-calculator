use crate::data::passives::ALL_PASSIVES;
use crate::domain::PassiveData;
use crate::domain::SkillLineName;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Default)]
pub struct PassivesServiceOptions {
    pub passives: Option<Vec<&'static PassiveData>>,
}

#[derive(Debug, Clone, Default)]
pub struct PassivesFilter {
    pub skill_lines: Option<HashSet<SkillLineName>>,
}

pub struct PassivesService {
    passives_by_skill_line: HashMap<SkillLineName, Vec<&'static PassiveData>>,
}

impl PassivesService {
    pub fn new(options: PassivesServiceOptions) -> Self {
        let passives = match options.passives {
            Some(p) => p,
            None => ALL_PASSIVES.iter().copied().collect(),
        };

        let passives_by_skill_line = Self::group_passives_by_skill_line(passives);

        Self {
            passives_by_skill_line,
        }
    }

    pub fn with_filter(mut self, filter: PassivesFilter) -> Self {
        if let Some(skill_lines) = filter.skill_lines {
            self.passives_by_skill_line
                .retain(|skill_line, _| skill_lines.contains(skill_line));
        }
        self
    }

    fn group_passives_by_skill_line(
        passives: Vec<&'static PassiveData>,
    ) -> HashMap<SkillLineName, Vec<&'static PassiveData>> {
        let mut passives_by_skill_line: HashMap<SkillLineName, Vec<&'static PassiveData>> =
            HashMap::new();

        for passive in passives {
            passives_by_skill_line
                .entry(passive.skill_line)
                .or_default()
                .push(passive);
        }

        passives_by_skill_line
    }

    pub fn get_passives_by_skill_line(
        &self,
        skill_line: SkillLineName,
    ) -> Vec<&'static PassiveData> {
        self.passives_by_skill_line
            .get(&skill_line)
            .cloned()
            .unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_passives_service_new() {
        let service = PassivesService::new(PassivesServiceOptions::default());
        let passives = service.get_passives_by_skill_line(SkillLineName::ArdentFlame);
        assert!(!passives.is_empty());
        assert!(passives
            .iter()
            .all(|p| p.skill_line == SkillLineName::ArdentFlame));
    }

    #[test]
    fn test_passives_service_with_filter() {
        let mut allowed = HashSet::new();
        allowed.insert(SkillLineName::ArdentFlame);

        let service =
            PassivesService::new(PassivesServiceOptions::default()).with_filter(PassivesFilter {
                skill_lines: Some(allowed),
            });

        // Should have ArdentFlame passives
        let passives = service.get_passives_by_skill_line(SkillLineName::ArdentFlame);
        assert!(!passives.is_empty());

        // Should NOT have other skill line passives (filtered out at construction)
        let other_passives = service.get_passives_by_skill_line(SkillLineName::DraconicPower);
        assert!(other_passives.is_empty());
    }
}
