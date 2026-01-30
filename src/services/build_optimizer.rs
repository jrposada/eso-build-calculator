use crate::data::bonuses::CHAMPION_POINTS;
use crate::data::skills::ALL_SKILLS;
use crate::data::{ClassName, SkillLineName};
use crate::domain::{Build, BonusData, ChampionPointBonus, Skill, SkillData, BUILD_CONSTRAINTS};
use crate::infrastructure::{combinatorics, logger, table};
use crate::services::{GetSkillsOptions, MorphSelector, MorphSelectorOptions, SkillsService};
use rayon::prelude::*;
use std::collections::HashSet;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Instant;

/// Options for the build optimizer
#[derive(Debug, Clone, Default)]
pub struct BuildOptimizerOptions {
    pub verbose: bool,
    pub required_class_names: Vec<ClassName>,
    pub required_weapons: Vec<SkillLineName>,
    pub forced_morphs: Vec<String>,
}

/// Build optimizer that finds the optimal skill/champion point combination
pub struct BuildOptimizer {
    #[allow(dead_code)]
    skills_service: SkillsService,
    #[allow(dead_code)]
    verbose: bool,

    required_class_names: Vec<ClassName>,
    required_weapons: Vec<SkillLineName>,
    class_names: HashSet<ClassName>,
    weapon_skill_line_names: HashSet<SkillLineName>,
    #[allow(dead_code)]
    skill_names: HashSet<String>,

    champion_point_combinations: Vec<Vec<ChampionPointBonus>>,
    skills_combinations: Vec<Vec<&'static SkillData>>,
    skill_combinations_count: u64,
}

impl BuildOptimizer {
    pub fn new(options: BuildOptimizerOptions) -> Self {
        let morph_selector = MorphSelector::new(MorphSelectorOptions {
            forced_morphs: options.forced_morphs.clone(),
        });

        if options.verbose {
            logger::dim(&format!(
                "Total skills before greedy morph selection: {}",
                ALL_SKILLS.len()
            ));
        }

        let all_skills: Vec<&SkillData> = ALL_SKILLS.iter().copied().collect();
        let skills = morph_selector.select_morphs(&all_skills);

        if options.verbose {
            logger::dim(&format!(
                "Total skills after greedy morph selection: {}",
                skills.len()
            ));
        }

        let skills_service = SkillsService::new();
        let required_class_names = options.required_class_names;
        let required_weapons = options.required_weapons;
        let verbose = options.verbose;

        // Generate champion point combinations
        let cp_vec: Vec<_> = CHAMPION_POINTS.iter().cloned().collect();
        let champion_point_combinations =
            combinatorics::generate_combinations(&cp_vec, BUILD_CONSTRAINTS.champion_point_count);

        if verbose {
            logger::dim(&format!(
                "Generated {} champion point combinations",
                champion_point_combinations.len()
            ));
        }

        // Generate class combinations
        let mut class_names: HashSet<ClassName> = HashSet::new();
        let class_class_name_combinations: Vec<Vec<ClassName>> = combinatorics::generate_combinations(
            &ClassName::CLASS_ONLY.to_vec(),
            BUILD_CONSTRAINTS.class_skill_line_count,
        )
        .into_iter()
        .filter(|combination| {
            let has_required_class = if required_class_names.is_empty() {
                true
            } else {
                required_class_names
                    .iter()
                    .all(|required| combination.contains(required))
            };

            if has_required_class {
                for class in combination {
                    class_names.insert(*class);
                }
            }

            has_required_class
        })
        .collect();

        if verbose {
            let required_str = if required_class_names.is_empty() {
                "none".to_string()
            } else {
                required_class_names
                    .iter()
                    .map(|c| c.to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            };
            logger::dim(&format!(
                "Generated {} class combinations using {} classes (required: {})",
                class_class_name_combinations.len(),
                class_names.len(),
                required_str
            ));
        }

        // Map class combinations to skill line combinations
        let class_skill_line_combinations: Vec<Vec<SkillLineName>> = class_class_name_combinations
            .iter()
            .map(|class_combination| {
                class_combination
                    .iter()
                    .flat_map(|class_name| SkillLineName::for_class(*class_name))
                    .collect()
            })
            .collect();

        // Generate weapon skill line combinations
        let mut weapon_skill_line_names: HashSet<SkillLineName> = HashSet::new();
        let weapon_skill_lines_combinations: Vec<Vec<SkillLineName>> =
            combinatorics::generate_combinations(
                &SkillLineName::WEAPON.to_vec(),
                BUILD_CONSTRAINTS.weapon_skill_line_count,
            )
            .into_iter()
            .filter(|combination| {
                let has_required_weapon = if required_weapons.is_empty() {
                    true
                } else {
                    required_weapons
                        .iter()
                        .all(|required| combination.contains(required))
                };

                if has_required_weapon {
                    for weapon in combination {
                        weapon_skill_line_names.insert(*weapon);
                    }
                }

                has_required_weapon
            })
            .collect();

        if verbose {
            let required_str = if required_weapons.is_empty() {
                "none".to_string()
            } else {
                required_weapons
                    .iter()
                    .map(|w| w.to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            };
            logger::dim(&format!(
                "Generated {} weapon skill line combinations using {} weapons (required: {})",
                weapon_skill_lines_combinations.len(),
                weapon_skill_line_names.len(),
                required_str
            ));
        }

        // Cartesian product of class and weapon skill line combinations
        let skill_line_combinations = combinatorics::cartesian_product(
            &class_skill_line_combinations,
            &weapon_skill_lines_combinations,
        );

        if verbose {
            logger::dim(&format!(
                "Generated {} total skill line combinations (class + weapon)",
                skill_line_combinations.len()
            ));
        }

        // Get skills for each skill line combination
        let mut skill_names: HashSet<String> = HashSet::new();
        let filter_options = GetSkillsOptions {
            exclude_base_skills: true,
            exclude_ultimates: true,
            exclude_non_damaging: true,
        };

        let skills_combinations: Vec<Vec<&'static SkillData>> = skill_line_combinations
            .iter()
            .map(|skill_line_combination| {
                skill_line_combination
                    .iter()
                    .flat_map(|skill_line| {
                        let skill_line_skills =
                            skills_service.get_skills_by_skill_line(*skill_line, &filter_options);
                        for skill in &skill_line_skills {
                            skill_names.insert(skill.name.clone());
                        }
                        skill_line_skills
                    })
                    .collect()
            })
            .collect();

        // Calculate total skill combinations
        let skill_combinations_count: u64 = skills_combinations
            .iter()
            .map(|skills| combinatorics::count_combinations(skills.len(), BUILD_CONSTRAINTS.skill_count))
            .sum();

        let optimizer = Self {
            skills_service,
            verbose,
            required_class_names,
            required_weapons,
            class_names,
            weapon_skill_line_names,
            skill_names,
            champion_point_combinations,
            skills_combinations,
            skill_combinations_count,
        };

        logger::info(&optimizer.to_string());

        optimizer
    }

    /// Find the optimal build that maximizes total damage per cast
    /// Uses Rayon for parallel evaluation
    pub fn find_optimal_build(&self) -> Option<Build> {
        let start_time = Instant::now();
        let total_combinations =
            self.champion_point_combinations.len() as u64 * self.skill_combinations_count;

        logger::info(&format!(
            "Starting optimization with {} total combinations...",
            format_number(total_combinations as i64)
        ));

        let evaluated_count = AtomicU64::new(0);
        let last_progress_update = AtomicU64::new(0);
        let best_damage = AtomicU64::new(0);

        // Parallel iteration over champion point combinations
        let best_build: Option<Build> = self
            .champion_point_combinations
            .par_iter()
            .map(|cp_combo| {
                let cp_bonuses: Vec<BonusData> = cp_combo
                    .iter()
                    .map(|cp| cp.to_bonus_data())
                    .collect();

                let mut local_best: Option<Build> = None;

                for skills in &self.skills_combinations {
                    // Generate skill combinations
                    for skill_combo in
                        combinatorics::CombinationIterator::new(skills, BUILD_CONSTRAINTS.skill_count)
                    {
                        let count = evaluated_count.fetch_add(1, Ordering::Relaxed) + 1;

                        // Create skills from data
                        let build_skills: Vec<Skill> = skill_combo
                            .iter()
                            .map(|&data| Skill::new(data.clone()))
                            .collect();

                        let build = Build::new(build_skills, cp_bonuses.clone());
                        let damage = build.total_damage_per_cast();

                        // Update local best
                        if local_best
                            .as_ref()
                            .is_none_or(|b| damage > b.total_damage_per_cast())
                        {
                            local_best = Some(build);

                            // Atomically update global best damage for progress display
                            let damage_bits = damage.to_bits();
                            let _ = best_damage.fetch_max(damage_bits, Ordering::Relaxed);
                        }

                        // Progress update every 100k iterations
                        if count % 100_000 == 0 {
                            let last = last_progress_update.swap(count, Ordering::Relaxed);
                            if count > last {
                                let progress = (count as f64 / total_combinations as f64) * 100.0;
                                let elapsed = start_time.elapsed().as_secs_f64();
                                let eta = if progress > 0.0 {
                                    elapsed * (100.0 - progress) / progress
                                } else {
                                    0.0
                                };
                                let best = f64::from_bits(best_damage.load(Ordering::Relaxed));

                                logger::progress(&format!(
                                    "Progress: {:.1}% ({}) | Best: {} | ETA: {}",
                                    progress,
                                    format_number(count as i64),
                                    format_number(best as i64),
                                    format_duration(eta)
                                ));
                            }
                        }
                    }
                }

                local_best
            })
            .reduce(
                || None,
                |a, b| match (a, b) {
                    (Some(a), Some(b)) => {
                        if a.total_damage_per_cast() > b.total_damage_per_cast() {
                            Some(a)
                        } else {
                            Some(b)
                        }
                    }
                    (Some(a), None) => Some(a),
                    (None, Some(b)) => Some(b),
                    (None, None) => None,
                },
            );

        let total_evaluated = evaluated_count.load(Ordering::Relaxed);
        let elapsed = start_time.elapsed();

        logger::info(&format!(
            "Completed: {} builds evaluated in {:.1}s",
            format_number(total_evaluated as i64),
            elapsed.as_secs_f64()
        ));

        best_build
    }
}

impl std::fmt::Display for BuildOptimizer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut lines = Vec::new();

        // Configuration table
        let config_data = vec![
            vec!["Skills".to_string(), BUILD_CONSTRAINTS.skill_count.to_string()],
            vec![
                "Champion Points".to_string(),
                BUILD_CONSTRAINTS.champion_point_count.to_string(),
            ],
            vec![
                "Class Skill Lines".to_string(),
                BUILD_CONSTRAINTS.class_skill_line_count.to_string(),
            ],
            vec![
                "Weapon Skill Lines".to_string(),
                BUILD_CONSTRAINTS.weapon_skill_line_count.to_string(),
            ],
            vec!["Workers".to_string(), num_cpus::get().to_string()],
        ];

        lines.push(table::table(
            &config_data,
            table::TableOptions {
                title: Some("Configuration".to_string()),
                columns: vec![
                    table::ColumnDefinition::new("Constraint", 25),
                    table::ColumnDefinition::new("Value", 10).align_right(),
                ],
                footer: None,
            },
        ));

        // Skill line configuration table
        let used_classes: Vec<_> = self.class_names.iter().map(|c| c.to_string()).collect();
        let used_classes_str = if used_classes.is_empty() {
            "None".to_string()
        } else {
            let mut sorted = used_classes;
            sorted.sort();
            sorted.join(", ")
        };

        let required_classes: Vec<_> = self
            .required_class_names
            .iter()
            .map(|c| c.to_string())
            .collect();
        let required_classes_str = if required_classes.is_empty() {
            "None".to_string()
        } else {
            let mut sorted = required_classes;
            sorted.sort();
            sorted.join(", ")
        };

        let used_weapons: Vec<_> = self
            .weapon_skill_line_names
            .iter()
            .map(|w| w.to_string())
            .collect();
        let used_weapons_str = if used_weapons.is_empty() {
            "None".to_string()
        } else {
            let mut sorted = used_weapons;
            sorted.sort();
            sorted.join(", ")
        };

        let required_weapons: Vec<_> = self.required_weapons.iter().map(|w| w.to_string()).collect();
        let required_weapons_str = if required_weapons.is_empty() {
            "None".to_string()
        } else {
            let mut sorted = required_weapons;
            sorted.sort();
            sorted.join(", ")
        };

        let name_width = [
            &used_classes_str,
            &required_classes_str,
            &used_weapons_str,
            &required_weapons_str,
        ]
        .iter()
        .map(|s| s.len())
        .max()
        .unwrap_or(20)
        .max(20);

        let skill_config_data = vec![
            vec!["Used Classes".to_string(), used_classes_str],
            vec!["Required Classes".to_string(), required_classes_str],
            vec!["Used Weapons".to_string(), used_weapons_str],
            vec!["Required Weapons".to_string(), required_weapons_str],
            vec![
                "Used Champion Points".to_string(),
                CHAMPION_POINTS.len().to_string(),
            ],
            vec!["Required Champion Points".to_string(), "N/A".to_string()],
        ];

        lines.push(table::table(
            &skill_config_data,
            table::TableOptions {
                title: Some("Skill Line Configuration".to_string()),
                columns: vec![
                    table::ColumnDefinition::new("Setting", 25),
                    table::ColumnDefinition::new("Name", name_width),
                ],
                footer: None,
            },
        ));

        let total_combinations =
            self.champion_point_combinations.len() as u64 * self.skill_combinations_count;
        lines.push(format!(
            "Estimated total combinations to evaluate: {}",
            format_number(total_combinations as i64)
        ));

        write!(f, "{}", lines.join("\n"))
    }
}

fn format_number(n: i64) -> String {
    let s = n.to_string();
    let mut result = String::new();
    for (i, c) in s.chars().rev().enumerate() {
        if i > 0 && i % 3 == 0 {
            result.push(',');
        }
        result.push(c);
    }
    result.chars().rev().collect()
}

fn format_duration(seconds: f64) -> String {
    let secs = seconds as u64;
    let mins = secs / 60;
    let hours = mins / 60;

    if hours > 0 {
        format!("{}h {}m", hours, mins % 60)
    } else if mins > 0 {
        format!("{}m {}s", mins, secs % 60)
    } else {
        format!("{}s", secs)
    }
}
