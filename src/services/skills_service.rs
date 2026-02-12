use crate::data::skills::ALL_SKILLS;
use crate::domain::{CharacterStats, Resource, SkillData, SkillLineName};
use crate::infrastructure::logger;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Default)]
pub struct SkillsServiceOptions {
    pub skills: Option<Vec<&'static SkillData>>,
}

#[derive(Debug, Clone, Default)]
pub struct MorphSelectionOptions {
    pub forced_morphs: Vec<String>,
}

#[derive(Debug, Clone, Default)]
pub struct SkillsFilter {
    pub exclude_ultimates: bool,
    pub exclude_non_damaging: bool,
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

        let skills_by_skill_line = Self::group_skills_by_skill_line(skills);

        Self {
            skills_by_skill_line,
        }
    }

    pub fn with_morph_selection(self, options: MorphSelectionOptions) -> Self {
        let skills: Vec<&'static SkillData> =
            self.skills_by_skill_line.into_values().flatten().collect();

        let selected = Self::select_morphs_from_skills(skills, &options);

        let skills_by_skill_line = Self::group_skills_by_skill_line(selected);
        Self {
            skills_by_skill_line,
        }
    }

    pub fn with_filter(mut self, filter: SkillsFilter) -> Self {
        for skills in self.skills_by_skill_line.values_mut() {
            skills.retain(|skill| {
                if filter.exclude_ultimates && skill.resource == Resource::Ultimate {
                    return false;
                }
                if filter.exclude_non_damaging && skill.damage.is_none() {
                    return false;
                }
                true
            });
        }
        self
    }

    fn group_skills_by_skill_line(
        skills: Vec<&'static SkillData>,
    ) -> HashMap<SkillLineName, Vec<&'static SkillData>> {
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

    pub fn get_skills_by_skill_line(&self, skill_line: SkillLineName) -> Vec<&'static SkillData> {
        self.skills_by_skill_line
            .get(&skill_line)
            .cloned()
            .unwrap_or_default()
    }
}

impl SkillsService {
    fn select_morphs_from_skills(
        skills: Vec<&'static SkillData>,
        options: &MorphSelectionOptions,
    ) -> Vec<&'static SkillData> {
        let morphs_by_base = Self::group_skills_by_base(&skills);
        let forced_morphs: HashSet<String> = options.forced_morphs.iter().cloned().collect();

        Self::validate_forced_morphs(&morphs_by_base, &forced_morphs);

        let mut selected = Vec::new();
        for morphs in morphs_by_base.values() {
            if let Some(forced) = morphs.iter().find(|m| forced_morphs.contains(&m.name)) {
                selected.push(*forced);
            } else if let Some(best) = Self::select_highest_damage_morph(morphs) {
                selected.push(best);
            }
        }
        selected
    }

    fn group_skills_by_base(
        skills: &[&'static SkillData],
    ) -> HashMap<String, Vec<&'static SkillData>> {
        let mut morphs_by_base: HashMap<String, Vec<&'static SkillData>> = HashMap::new();
        for skill in skills {
            if skill.name == skill.base_skill_name {
                continue;
            }
            morphs_by_base
                .entry(skill.base_skill_name.clone())
                .or_default()
                .push(*skill);
        }
        morphs_by_base
    }

    fn validate_forced_morphs(
        morphs_by_base: &HashMap<String, Vec<&'static SkillData>>,
        forced_morphs: &HashSet<String>,
    ) {
        let valid_names: HashSet<&str> = morphs_by_base
            .values()
            .flatten()
            .map(|s| s.name.as_str())
            .collect();

        let mut invalid: Vec<_> = forced_morphs
            .iter()
            .filter(|name| !valid_names.contains(name.as_str()))
            .cloned()
            .collect();

        if !invalid.is_empty() {
            invalid.sort();
            logger::warn(&format!(
                "Warning: The following morph names are invalid and will be ignored: {}",
                invalid.join(", ")
            ));
        }
    }

    fn select_highest_damage_morph(morphs: &[&'static SkillData]) -> Option<&'static SkillData> {
        let default_stats = CharacterStats::default();
        morphs.iter().copied().max_by(|a, b| {
            let damage_a = a.calculate_damage_per_cast(&[], &default_stats, None);
            let damage_b = b.calculate_damage_per_cast(&[], &default_stats, None);
            damage_a
                .partial_cmp(&damage_b)
                .unwrap_or(std::cmp::Ordering::Equal)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_skills_by_skill_line() {
        let service = SkillsService::new(SkillsServiceOptions::default());
        let skills = service.get_skills_by_skill_line(SkillLineName::ArdentFlame);
        assert!(!skills.is_empty());
        assert!(skills
            .iter()
            .all(|s| s.skill_line == SkillLineName::ArdentFlame));
    }

    #[test]
    fn test_with_filter_exclude_ultimates() {
        let service =
            SkillsService::new(SkillsServiceOptions::default()).with_filter(SkillsFilter {
                exclude_ultimates: true,
                ..Default::default()
            });
        let skills = service.get_skills_by_skill_line(SkillLineName::ArdentFlame);
        assert!(skills.iter().all(|s| s.resource != Resource::Ultimate));
    }

    #[test]
    fn test_with_morph_selection() {
        let service = SkillsService::new(SkillsServiceOptions::default())
            .with_morph_selection(MorphSelectionOptions::default());

        // Collect all skills after morph selection
        let all_skills: Vec<_> = SkillLineName::ALL
            .iter()
            .flat_map(|sl| service.get_skills_by_skill_line(*sl))
            .collect();

        // Should have fewer skills than input (morphs are deduplicated)
        assert!(all_skills.len() < ALL_SKILLS.len());

        // Each base skill should only appear once
        let mut base_names: HashSet<&str> = HashSet::new();
        for skill in &all_skills {
            assert!(
                base_names.insert(&skill.base_skill_name),
                "Duplicate base skill: {}",
                skill.base_skill_name
            );
        }
    }

    #[test]
    fn test_with_morph_selection_forced() {
        let skills: Vec<_> = ALL_SKILLS.iter().copied().collect();

        // Find a skill to force
        let forced_skill = skills.iter().find(|s| s.name != s.base_skill_name);
        if let Some(skill) = forced_skill {
            let service = SkillsService::new(SkillsServiceOptions::default()).with_morph_selection(
                MorphSelectionOptions {
                    forced_morphs: vec![skill.name.clone()],
                },
            );

            // Collect all skills after morph selection
            let all_skills: Vec<_> = SkillLineName::ALL
                .iter()
                .flat_map(|sl| service.get_skills_by_skill_line(*sl))
                .collect();

            // The forced morph should be in the selection
            assert!(all_skills.iter().any(|s| s.name == skill.name));
        }
    }
}
