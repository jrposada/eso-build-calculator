use crate::domain::{Skill, SkillData};
use crate::infrastructure::logger;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

/// Options for morph selection
#[derive(Debug, Clone, Default)]
pub struct MorphSelectorOptions {
    /// Skill names to force select (overrides greedy selection)
    pub forced_morphs: Vec<String>,
}

/// Selects morphs using greedy strategy or forced selections
pub struct MorphSelector {
    forced_morphs: HashSet<String>,
}

impl MorphSelector {
    pub fn new(options: MorphSelectorOptions) -> Self {
        Self {
            forced_morphs: options.forced_morphs.into_iter().collect(),
        }
    }

    /// Select one morph per base skill using greedy strategy (highest damage)
    /// or forced selections from CLI options.
    pub fn select_morphs<'a>(&self, skills: &[&'a SkillData]) -> Vec<&'a SkillData> {
        let invalid_morphs = self.validate_forced_morphs(skills);
        if !invalid_morphs.is_empty() {
            let mut sorted_morphs: Vec<_> = invalid_morphs.iter().collect();
            sorted_morphs.sort();
            logger::warn(&format!(
                "Warning: The following morph names are invalid and will be ignored: {}",
                sorted_morphs
                    .into_iter()
                    .cloned()
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
        }

        // Group skills by base skill name
        let mut skills_by_base: HashMap<&str, Vec<&'a SkillData>> = HashMap::new();
        for skill in skills {
            skills_by_base
                .entry(&skill.base_skill_name)
                .or_default()
                .push(skill);
        }

        let mut selected_skills: Vec<&'a SkillData> = Vec::new();

        for (_, morphs) in skills_by_base {
            // Check if any morph is forced
            if let Some(forced_morph) = morphs.iter().find(|m| self.forced_morphs.contains(&m.name))
            {
                selected_skills.push(forced_morph);
            } else {
                // Use greedy strategy: select morph with highest base damage
                if let Some(best_morph) = self.select_highest_damage_morph(&morphs) {
                    selected_skills.push(best_morph);
                }
            }
        }

        selected_skills
    }

    /// Returns list of invalid morph names that don't exist in the skill list
    fn validate_forced_morphs(&self, skills: &[&SkillData]) -> HashSet<String> {
        let valid_morph_names: HashSet<&str> = skills.iter().map(|s| s.name.as_str()).collect();
        let mut invalid_morphs = HashSet::new();

        for morph_name in &self.forced_morphs {
            if !valid_morph_names.contains(morph_name.as_str()) {
                invalid_morphs.insert(morph_name.clone());
            }
        }

        invalid_morphs
    }

    fn select_highest_damage_morph<'a>(&self, morphs: &[&'a SkillData]) -> Option<&'a SkillData> {
        if morphs.is_empty() {
            return None;
        }
        if morphs.len() == 1 {
            return Some(morphs[0]);
        }

        let mut best_morph: Option<&'a SkillData> = None;
        let mut best_damage = f64::NEG_INFINITY;

        for morph_data in morphs {
            let skill = Skill::from_data(Arc::new((*morph_data).clone()));
            let damage = skill.calculate_damage_per_cast(&[]);

            if damage > best_damage {
                best_damage = damage;
                best_morph = Some(morph_data);
            }
        }

        best_morph
    }
}

impl Default for MorphSelector {
    fn default() -> Self {
        Self::new(MorphSelectorOptions::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::services::SkillsService;

    #[test]
    fn test_morph_selector_groups_by_base() {
        let service = SkillsService::new();
        let skills: Vec<_> = service.get_all_skills(&Default::default());
        let selector = MorphSelector::default();

        let selected = selector.select_morphs(&skills);

        // Should have fewer skills than input (morphs are deduplicated)
        assert!(selected.len() < skills.len());

        // Each base skill should only appear once
        let mut base_names: HashSet<&str> = HashSet::new();
        for skill in &selected {
            assert!(
                base_names.insert(&skill.base_skill_name),
                "Duplicate base skill: {}",
                skill.base_skill_name
            );
        }
    }

    #[test]
    fn test_forced_morphs() {
        let service = SkillsService::new();
        let skills: Vec<_> = service.get_all_skills(&Default::default());

        // Find a skill to force
        let forced_skill = skills.iter().find(|s| s.name != s.base_skill_name);
        if let Some(skill) = forced_skill {
            let selector = MorphSelector::new(MorphSelectorOptions {
                forced_morphs: vec![skill.name.clone()],
            });

            let selected = selector.select_morphs(&skills);

            // The forced morph should be in the selection
            assert!(selected.iter().any(|s| s.name == skill.name));
        }
    }
}
