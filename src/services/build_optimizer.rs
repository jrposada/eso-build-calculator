use crate::data::bonuses::CHAMPION_POINTS;
use crate::data::skills::ALL_SKILLS;
use crate::data::{ClassName, SkillLineName};
use crate::domain::{BonusData, Build, ChampionPointBonus, Skill, SkillData, BUILD_CONSTRAINTS};
use crate::infrastructure::{combinatorics, format, logger, table};
use crate::services::passive_service::PassiveServiceOptions;
use crate::services::skills_service::SkillsServiceOptions;
use crate::services::{
    FilterSkillsOptions, MorphSelector, MorphSelectorOptions, PassiveService, SkillsService,
};
use rayon::iter::ParallelBridge;
use rayon::prelude::*;
use rayon::ThreadPoolBuilder;
use std::collections::HashSet;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Instant;

#[derive(Debug, Clone, Default)]
pub struct BuildOptimizerOptions {
    pub verbose: bool,
    pub pure_class: Option<ClassName>,
    pub required_class_names: Vec<ClassName>,
    pub required_weapon_skill_lines: Vec<SkillLineName>,
    pub forced_morphs: Vec<String>,
    pub parallelism: u8,
}

pub struct BuildOptimizer {
    required_class_names: Vec<ClassName>,
    class_names: HashSet<ClassName>,
    required_weapon_skill_lines: Vec<SkillLineName>,
    weapon_skill_line_names: HashSet<SkillLineName>,
    skill_names: HashSet<String>,
    parallelism: u8,

    champion_point_combinations: Vec<Vec<ChampionPointBonus>>,
    skills_combinations: Vec<Vec<&'static SkillData>>,
    passive_bonuses_list: Vec<Vec<BonusData>>,
    total_possible_build_count: u64,
}

impl BuildOptimizer {
    pub fn new(options: BuildOptimizerOptions) -> Self {
        let verbose = options.verbose;
        let parallelism = options.parallelism;
        let required_class_names = options.required_class_names;
        let required_weapon_skill_lines = options.required_weapon_skill_lines;

        let skills = Self::prepare_skills(&options.forced_morphs, verbose);
        let skills_service = SkillsService::new(SkillsServiceOptions {
            skills: Some(skills),
        });

        let champion_point_combinations = Self::generate_champion_point_combinations(verbose);

        let (class_names, class_skill_line_combinations) =
            Self::generate_class_skill_line_combinations(
                options.pure_class,
                &required_class_names,
                verbose,
            );

        let (weapon_skill_line_names, weapon_skill_line_combinations) =
            Self::generate_weapon_skill_line_combinations(&required_weapon_skill_lines, verbose);

        let skill_line_combinations = combinatorics::cartesian_product(
            &class_skill_line_combinations,
            &weapon_skill_line_combinations,
        );

        if verbose {
            logger::dim(&format!(
                "Generated {} total skill line combinations (class + weapon)",
                skill_line_combinations.len()
            ));
        }

        let (skill_names, skills_combinations) =
            Self::generate_skills_combinations(&skill_line_combinations, &skills_service);

        let passive_bonuses_list =
            Self::generate_passive_bonuses(&skill_line_combinations, verbose);

        let total_possible_build_count =
            Self::calculate_total_build_count(&champion_point_combinations, &skills_combinations);

        let optimizer = Self {
            required_class_names,
            class_names,
            required_weapon_skill_lines,
            weapon_skill_line_names,
            skill_names,
            parallelism,
            champion_point_combinations,
            skills_combinations,
            passive_bonuses_list,
            total_possible_build_count,
        };

        logger::log(&optimizer.to_string());

        optimizer
    }

    fn prepare_skills(forced_morphs: &[String], verbose: bool) -> Vec<&'static SkillData> {
        let morph_selector = MorphSelector::new(MorphSelectorOptions {
            forced_morphs: forced_morphs.to_vec(),
        });

        if verbose {
            logger::dim(&format!(
                "Total skills before greedy morph selection: {}",
                ALL_SKILLS.len()
            ));
        }

        let all_skills: Vec<&SkillData> = ALL_SKILLS.iter().copied().collect();
        let skills = morph_selector.select_morphs(&all_skills);

        if verbose {
            logger::dim(&format!(
                "Total skills after greedy morph selection: {}",
                skills.len()
            ));
        }

        skills
    }

    fn generate_champion_point_combinations(verbose: bool) -> Vec<Vec<ChampionPointBonus>> {
        let cp_vec: Vec<_> = CHAMPION_POINTS.iter().cloned().collect();
        let combinations =
            combinatorics::generate_combinations(&cp_vec, BUILD_CONSTRAINTS.champion_point_count);

        if verbose {
            logger::dim(&format!(
                "Generated {} champion point combinations",
                combinations.len()
            ));
        }

        combinations
    }

    fn generate_class_skill_line_combinations(
        pure_class: Option<ClassName>,
        required_class_names: &[ClassName],
        verbose: bool,
    ) -> (HashSet<ClassName>, Vec<Vec<SkillLineName>>) {
        let mut class_names: HashSet<ClassName> = HashSet::new();

        let class_name_combinations: Vec<Vec<ClassName>> = if let Some(class) = pure_class {
            class_names.insert(class);
            vec![vec![class]]
        } else {
            combinatorics::generate_combinations(
                &ClassName::CLASS_ONLY.to_vec(),
                BUILD_CONSTRAINTS.class_skill_line_count,
            )
            .into_iter()
            .filter(|combination| {
                let has_required = required_class_names.is_empty()
                    || required_class_names
                        .iter()
                        .all(|required| combination.contains(required));

                if has_required {
                    for class in combination {
                        class_names.insert(*class);
                    }
                }

                has_required
            })
            .collect()
        };

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
                class_name_combinations.len(),
                class_names.len(),
                required_str
            ));
        }

        let skill_line_combinations: Vec<Vec<SkillLineName>> = class_name_combinations
            .iter()
            .map(|class_combination| {
                class_combination
                    .iter()
                    .flat_map(|class_name| SkillLineName::for_class(*class_name))
                    .collect()
            })
            .collect();

        (class_names, skill_line_combinations)
    }

    fn generate_weapon_skill_line_combinations(
        required_weapon_skill_lines: &[SkillLineName],
        verbose: bool,
    ) -> (HashSet<SkillLineName>, Vec<Vec<SkillLineName>>) {
        let mut weapon_skill_line_names: HashSet<SkillLineName> = HashSet::new();

        let combinations: Vec<Vec<SkillLineName>> = combinatorics::generate_combinations(
            &SkillLineName::WEAPON.to_vec(),
            BUILD_CONSTRAINTS.weapon_skill_line_count,
        )
        .into_iter()
        .filter(|combination| {
            let has_required = required_weapon_skill_lines.is_empty()
                || required_weapon_skill_lines
                    .iter()
                    .all(|required| combination.contains(required));

            if has_required {
                for weapon in combination {
                    weapon_skill_line_names.insert(*weapon);
                }
            }

            has_required
        })
        .collect();

        if verbose {
            let required_str = if required_weapon_skill_lines.is_empty() {
                "none".to_string()
            } else {
                required_weapon_skill_lines
                    .iter()
                    .map(|w| w.to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            };
            logger::dim(&format!(
                "Generated {} weapon skill line combinations using {} weapons (required: {})",
                combinations.len(),
                weapon_skill_line_names.len(),
                required_str
            ));
        }

        (weapon_skill_line_names, combinations)
    }

    fn generate_skills_combinations(
        skill_line_combinations: &[Vec<SkillLineName>],
        skills_service: &SkillsService,
    ) -> (HashSet<String>, Vec<Vec<&'static SkillData>>) {
        let mut skill_names: HashSet<String> = HashSet::new();
        let filter_options = FilterSkillsOptions {
            exclude_base_skills: true,
            exclude_ultimates: true,
            exclude_non_damaging: true,
        };

        let combinations: Vec<Vec<&'static SkillData>> = skill_line_combinations
            .iter()
            .map(|skill_line_combination| {
                skill_line_combination
                    .iter()
                    .flat_map(|skill_line| {
                        let skills =
                            skills_service.get_skills_by_skill_line(*skill_line, &filter_options);
                        for skill in &skills {
                            skill_names.insert(skill.name.clone());
                        }
                        skills
                    })
                    .collect()
            })
            .collect();

        (skill_names, combinations)
    }

    fn generate_passive_bonuses(
        skill_line_combinations: &[Vec<SkillLineName>],
        verbose: bool,
    ) -> Vec<Vec<BonusData>> {
        let all_used_skill_lines: HashSet<SkillLineName> =
            skill_line_combinations.iter().flatten().copied().collect();

        let passive_service = PassiveService::new(PassiveServiceOptions {
            skill_lines: Some(all_used_skill_lines),
        });

        let bonuses: Vec<Vec<BonusData>> = skill_line_combinations
            .iter()
            .map(|skill_lines| {
                skill_lines
                    .iter()
                    .flat_map(|sl| passive_service.get_passives_by_skill_line(*sl))
                    .flat_map(|passive| passive.bonuses.iter().cloned())
                    .collect()
            })
            .collect();

        if verbose {
            logger::dim(&format!(
                "Pre-computed passive bonuses for {} skill line combinations",
                bonuses.len()
            ));
        }

        bonuses
    }

    fn calculate_total_build_count(
        champion_point_combinations: &[Vec<ChampionPointBonus>],
        skills_combinations: &[Vec<&'static SkillData>],
    ) -> u64 {
        let skill_combinations_count: u64 = skills_combinations
            .iter()
            .map(|skills| {
                combinatorics::count_combinations(skills.len(), BUILD_CONSTRAINTS.skill_count)
            })
            .sum();

        champion_point_combinations.len() as u64 * skill_combinations_count
    }

    pub fn find_optimal_build(&self) -> Option<Build> {
        let start_time = Instant::now();

        logger::log(&format!("Using {} threads...", self.parallelism));

        let evaluated_count = AtomicU64::new(0);
        let last_progress_update = AtomicU64::new(0);
        let best_damage = AtomicU64::new(0);

        let pool = ThreadPoolBuilder::new()
            .num_threads(self.parallelism as usize)
            .build()
            .expect("Failed to create thread pool");

        // Pre-compute CP bonuses for each combination to avoid repeated conversion
        let cp_bonuses_list: Vec<Vec<BonusData>> = self
            .champion_point_combinations
            .iter()
            .map(|cp_combo| cp_combo.iter().map(|cp| cp.to_bonus_data()).collect())
            .collect();

        // Use fine-grained parallelism: flatten all (cp_combo, skills,
        // passive_bonuses, skill_combo) combinations and parallelize at the
        // individual build level using par_bridge()
        let best_build: Option<Build> = pool.install(|| {
            cp_bonuses_list
                .iter()
                .flat_map(|cp_bonuses| {
                    self.skills_combinations
                        .iter()
                        .zip(self.passive_bonuses_list.iter())
                        .flat_map(move |(skills, passive_bonuses)| {
                            combinatorics::CombinationIterator::new(
                                skills,
                                BUILD_CONSTRAINTS.skill_count,
                            )
                            .map(move |skill_combo| (cp_bonuses, passive_bonuses, skill_combo))
                        })
                })
                .par_bridge()
                .map(|(cp_bonuses, passive_bonuses, skill_combo)| {
                    let count = evaluated_count.fetch_add(1, Ordering::Relaxed) + 1;

                    // Create skills from data
                    let build_skills: Vec<Skill> = skill_combo
                        .iter()
                        .map(|&data| Skill::new(data.clone()))
                        .collect();

                    let build = Build::new(build_skills, cp_bonuses.clone(), passive_bonuses);
                    let damage = build.total_damage;

                    // Atomically update global best damage for progress display
                    let damage_bits = damage.to_bits();
                    let _ = best_damage.fetch_max(damage_bits, Ordering::Relaxed);

                    // Progress update every 100k iterations
                    if count % 100_000 == 0 {
                        let last = last_progress_update.swap(count, Ordering::Relaxed);
                        if count > last {
                            let progress =
                                (count as f64 / self.total_possible_build_count as f64) * 100.0;
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
                                format::format_number(count),
                                format::format_number(best as u64),
                                format::format_duration((eta * 1000.0) as u64)
                            ));
                        }
                    }

                    build
                })
                .reduce_with(|a, b| {
                    if a.total_damage > b.total_damage {
                        a
                    } else {
                        b
                    }
                })
        });

        let total_evaluated = evaluated_count.load(Ordering::Relaxed);
        let elapsed = start_time.elapsed();

        logger::log(&format!(
            "Completed: {} builds evaluated in {:.1}s",
            format::format_number(total_evaluated),
            elapsed.as_secs_f64()
        ));

        best_build
    }
}

impl std::fmt::Display for BuildOptimizer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut lines = Vec::new();

        let config_data = vec![
            vec![
                "Skills".to_string(),
                BUILD_CONSTRAINTS.skill_count.to_string(),
            ],
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
            vec!["Workers".to_string(), self.parallelism.to_string()],
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

        let required_weapons: Vec<_> = self
            .required_weapon_skill_lines
            .iter()
            .map(|w| w.to_string())
            .collect();
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
            vec![
                "Used Skills".to_string(),
                self.skill_names.len().to_string(),
            ],
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

        lines.push(format!(
            "Total builds to be evaluated: {}",
            format::format_number(self.total_possible_build_count)
        ));

        write!(f, "{}", lines.join("\n"))
    }
}
