use crate::data::skills::ALL_SKILLS;
use crate::data::{Resource, SkillLineName};
use crate::domain::SkillData;
use std::collections::HashMap;

#[derive(Debug, Clone, Default)]
pub struct FilterSkillsOptions {
    pub exclude_base_skills: bool,
    pub exclude_ultimates: bool,
    pub exclude_non_damaging: bool,
}

#[derive(Debug, Clone, Default)]
pub struct SkillsServiceOptions {
    pub skills: Option<Vec<&'static SkillData>>,
}

pub struct SkillsService {
    skills_by_skill_line: HashMap<SkillLineName, Vec<&'static SkillData>>,
}

impl SkillsService {
    pub fn new(options: SkillsServiceOptions) -> Self {
        let skills = match options.skills {
            Some(s) => s,
            None => ALL_SKILLS.iter().copied().collect(),
        };

        let skills_by_skill_line  = Self::group_skills_by_skill_line(skills);

        Self {
            skills_by_skill_line,
        }
    }

    fn group_skills_by_skill_line(skills: Vec<&'static SkillData>) -> HashMap<SkillLineName, Vec<&'static SkillData>> {
        let mut skills_by_skill_line: HashMap<SkillLineName, Vec<&'static SkillData>> =
            HashMap::new();

        for skill in skills {
            skills_by_skill_line
                .entry(skill.skill_line)
                .or_default()
                .push(skill);
        }

        skills_by_skill_line
    }

    pub fn get_skills_by_skill_line(
        &self,
        skill_line: SkillLineName,
        options: &FilterSkillsOptions,
    ) -> Vec<&'static SkillData> {
        let skills = self
            .skills_by_skill_line
            .get(&skill_line)
            .cloned()
            .unwrap_or_default();

        self.filter_skills(skills, options)
    }

    fn filter_skills(
        &self,
        skills: Vec<&'static SkillData>,
        options: &FilterSkillsOptions,
    ) -> Vec<&'static SkillData> {
        skills
            .into_iter()
            .filter(|skill| {
                if options.exclude_base_skills && skill.name == skill.base_skill_name {
                    return false;
                }
                if options.exclude_ultimates && skill.resource == Resource::Ultimate {
                    return false;
                }
                if options.exclude_non_damaging && !skill.damage.has_damage() {
                    return false;
                }
                true
            })
            .collect()
    }
}

impl Default for SkillsService {
    fn default() -> Self {
        Self::new(SkillsServiceOptions::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_skills_by_skill_line() {
        let service = SkillsService::new(SkillsServiceOptions::default());
        let skills = service
            .get_skills_by_skill_line(SkillLineName::ArdentFlame, &FilterSkillsOptions::default());
        assert!(!skills.is_empty());
        assert!(skills
            .iter()
            .all(|s| s.skill_line == SkillLineName::ArdentFlame));
    }

    #[test]
    fn test_exclude_base_skills() {
        let service = SkillsService::new(SkillsServiceOptions::default());
        let options = FilterSkillsOptions {
            exclude_base_skills: true,
            ..Default::default()
        };
        let skills = service.get_skills_by_skill_line(SkillLineName::ArdentFlame, &options);
        // All remaining skills should be morphs (name != base_skill_name)
        assert!(skills.iter().all(|s| s.name != s.base_skill_name));
    }

    #[test]
    fn test_exclude_ultimates() {
        let service = SkillsService::new(SkillsServiceOptions::default());
        let options = FilterSkillsOptions {
            exclude_ultimates: true,
            ..Default::default()
        };
        let skills = service.get_skills_by_skill_line(SkillLineName::ArdentFlame, &options);
        assert!(skills.iter().all(|s| s.resource != Resource::Ultimate));
    }
}
