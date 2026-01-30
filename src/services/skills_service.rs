use crate::data::skills::{ALL_CLASS_SKILLS, ALL_SKILLS, ALL_WEAPON_SKILLS};
use crate::data::{ClassName, Resource, SkillLineName};
use crate::domain::SkillData;
use std::collections::HashMap;

/// Options for filtering skills
#[derive(Debug, Clone, Default)]
pub struct GetSkillsOptions {
    /// Exclude base skills (only include morphs)
    pub exclude_base_skills: bool,
    /// Exclude ultimate skills
    pub exclude_ultimates: bool,
    /// Exclude skills that don't deal damage
    pub exclude_non_damaging: bool,
}

/// Options for morph selection
#[derive(Debug, Clone, Default)]
pub struct SkillsServiceOptions {
    /// List of skills to include; if None, include all skills
    pub skills: Option<Vec<&'static SkillData>>,
}

/// Service for querying and filtering skills
pub struct SkillsService {
    skills_by_skill_line: HashMap<SkillLineName, Vec<&'static SkillData>>,
}

impl SkillsService {
    pub fn new(options: SkillsServiceOptions) -> Self {
        let skills = match options.skills {
            Some(s) => s,
            None => ALL_SKILLS.iter().copied().collect(),
        };

        let mut skills_by_skill_line: HashMap<SkillLineName, Vec<&'static SkillData>> =
            HashMap::new();

        for skill in skills.iter() {
            skills_by_skill_line
                .entry(skill.skill_line)
                .or_default()
                .push(skill);
        }

        Self {
            skills_by_skill_line,
        }
    }

    /// Get the class that a skill line belongs to
    pub fn get_class(skill_line: SkillLineName) -> ClassName {
        skill_line.get_class()
    }

    /// Check if a skill line belongs to a specific class
    pub fn is_skill_line_from_class(class_name: ClassName, skill_line: SkillLineName) -> bool {
        skill_line.get_class() == class_name
    }

    /// Get all skills for a class
    pub fn get_skills_by_class(
        &self,
        class_name: ClassName,
        options: &GetSkillsOptions,
    ) -> Vec<&'static SkillData> {
        let skills = if class_name == ClassName::Weapon {
            ALL_WEAPON_SKILLS.iter().copied().collect()
        } else {
            ALL_CLASS_SKILLS
                .iter()
                .copied()
                .filter(|s| s.class_name == class_name)
                .collect()
        };

        self.filter_skills(skills, options)
    }

    /// Get all skills for a skill line
    pub fn get_skills_by_skill_line(
        &self,
        skill_line: SkillLineName,
        options: &GetSkillsOptions,
    ) -> Vec<&'static SkillData> {
        let skills = self
            .skills_by_skill_line
            .get(&skill_line)
            .cloned()
            .unwrap_or_default();

        self.filter_skills(skills, options)
    }

    /// Get all skills (class + weapon)
    pub fn get_all_skills(&self, options: &GetSkillsOptions) -> Vec<&'static SkillData> {
        let skills: Vec<_> = ALL_SKILLS.iter().copied().collect();
        self.filter_skills(skills, options)
    }

    /// Get all class skills
    pub fn get_all_class_skills(&self, options: &GetSkillsOptions) -> Vec<&'static SkillData> {
        let skills: Vec<_> = ALL_CLASS_SKILLS.iter().copied().collect();
        self.filter_skills(skills, options)
    }

    /// Get all weapon skills
    pub fn get_all_weapon_skills(&self, options: &GetSkillsOptions) -> Vec<&'static SkillData> {
        let skills: Vec<_> = ALL_WEAPON_SKILLS.iter().copied().collect();
        self.filter_skills(skills, options)
    }

    fn filter_skills(
        &self,
        skills: Vec<&'static SkillData>,
        options: &GetSkillsOptions,
    ) -> Vec<&'static SkillData> {
        skills
            .into_iter()
            .filter(|skill| {
                // Exclude base skills if requested
                if options.exclude_base_skills && skill.name == skill.base_skill_name {
                    return false;
                }
                // Exclude ultimates if requested
                if options.exclude_ultimates && skill.resource == Resource::Ultimate {
                    return false;
                }
                // Exclude non-damaging skills if requested
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
    fn test_get_skills_by_class() {
        let service = SkillsService::new(SkillsServiceOptions::default());
        let skills =
            service.get_skills_by_class(ClassName::Dragonknight, &GetSkillsOptions::default());
        assert!(!skills.is_empty());
        assert!(skills
            .iter()
            .all(|s| s.class_name == ClassName::Dragonknight));
    }

    #[test]
    fn test_get_skills_by_skill_line() {
        let service = SkillsService::new(SkillsServiceOptions::default());
        let skills = service
            .get_skills_by_skill_line(SkillLineName::ArdentFlame, &GetSkillsOptions::default());
        assert!(!skills.is_empty());
        assert!(skills
            .iter()
            .all(|s| s.skill_line == SkillLineName::ArdentFlame));
    }

    #[test]
    fn test_exclude_base_skills() {
        let service = SkillsService::new(SkillsServiceOptions::default());
        let options = GetSkillsOptions {
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
        let options = GetSkillsOptions {
            exclude_ultimates: true,
            ..Default::default()
        };
        let skills = service.get_skills_by_skill_line(SkillLineName::ArdentFlame, &options);
        assert!(skills.iter().all(|s| s.resource != Resource::Ultimate));
    }

    #[test]
    fn test_exclude_non_damaging() {
        let service = SkillsService::new(SkillsServiceOptions::default());
        let options = GetSkillsOptions {
            exclude_non_damaging: true,
            ..Default::default()
        };
        let skills = service.get_all_skills(&options);
        assert!(skills.iter().all(|s| s.damage.has_damage()));
    }
}
