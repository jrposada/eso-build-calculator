use crate::data::passives::{ALL_CLASS_PASSIVES, ALL_WEAPON_PASSIVES};
use crate::data::SkillLineName;
use crate::domain::PassiveData;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Default)]
pub struct PassiveServiceOptions {
    pub skill_lines: Option<HashSet<SkillLineName>>,
}

pub struct PassiveService {
    passives_by_skill_line: HashMap<SkillLineName, Vec<&'static PassiveData>>,
}

impl PassiveService {
    pub fn new(options: PassiveServiceOptions) -> Self {
        let mut passives_by_skill_line: HashMap<SkillLineName, Vec<&'static PassiveData>> =
            HashMap::new();

        // Combine and group all passives
        for passive in ALL_CLASS_PASSIVES.iter().chain(ALL_WEAPON_PASSIVES.iter()) {
            if let Some(ref allowed) = options.skill_lines {
                if !allowed.contains(&passive.skill_line) {
                    continue;
                }
            }
            passives_by_skill_line
                .entry(passive.skill_line)
                .or_default()
                .push(passive);
        }

        Self {
            passives_by_skill_line,
        }
    }

    pub fn get_passives_by_skill_line(&self, skill_line: SkillLineName) -> &[&'static PassiveData] {
        self.passives_by_skill_line
            .get(&skill_line)
            .map(|v| v.as_slice())
            .unwrap_or(&[])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_passive_service_new() {
        let service = PassiveService::new(PassiveServiceOptions::default());
        let passives = service.get_passives_by_skill_line(SkillLineName::ArdentFlame);
        assert!(!passives.is_empty());
        assert!(passives
            .iter()
            .all(|p| p.skill_line == SkillLineName::ArdentFlame));
    }

    #[test]
    fn test_passive_service_with_skill_line_filter() {
        let mut allowed = HashSet::new();
        allowed.insert(SkillLineName::ArdentFlame);

        let service = PassiveService::new(PassiveServiceOptions {
            skill_lines: Some(allowed),
        });

        // Should have ArdentFlame passives
        let passives = service.get_passives_by_skill_line(SkillLineName::ArdentFlame);
        assert!(!passives.is_empty());

        // Should NOT have other skill line passives
        let other_passives = service.get_passives_by_skill_line(SkillLineName::DraconicPower);
        assert!(other_passives.is_empty());
    }
}
