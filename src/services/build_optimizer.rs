use crate::data::bonuses::CHAMPION_POINTS;
use crate::data::skills::ALL_SKILLS;
use crate::domain::{
    BonusData, BonusTrigger, Build, CharacterStats, ResolvedBonus, SkillData, BUILD_CONSTRAINTS,
};
use crate::domain::{ClassName, ResolveContext, SkillLineName};
use crate::infrastructure::{combinatorics, format, logger, table};
use crate::services::passives_service::{PassivesFilter, PassivesServiceOptions};
use crate::services::skills_service::{MorphSelectionOptions, SkillsFilter, SkillsServiceOptions};
use crate::services::{PassivesService, SkillsService};
use rayon::prelude::*;
use rayon::ThreadPoolBuilder;
use smallvec::SmallVec;
use std::collections::HashSet;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Instant;

#[derive(Debug, Clone)]
pub struct BuildOptimizerOptions {
    pub character_stats: CharacterStats,
    pub verbose: bool,
    pub pure_class: Option<ClassName>,
    pub required_class_names: Vec<ClassName>,
    pub required_weapon_skill_lines: Vec<SkillLineName>,
    pub required_champion_points: Vec<BonusData>,
    pub forced_morphs: Vec<String>,
    pub parallelism: u8,
    pub max_pool_size: Option<usize>,
}

/// Three-way split of bonuses for the optimizer fast path:
/// - `pre_resolved`: simple non-AbilitySlottedCount bonuses pre-resolved to ResolvedBonus
/// - `ability_count`: simple AbilitySlottedCount bonuses (multiplier depends on skills)
/// - `alt`: alternative bonuses (need per-eval resolution)
type PreSplitBonuses = (Vec<ResolvedBonus>, Vec<BonusData>, Vec<BonusData>);

pub struct BuildOptimizer {
    character_stats: CharacterStats,
    required_class_names: Vec<ClassName>,
    class_names: HashSet<ClassName>,
    required_weapon_skill_lines: Vec<SkillLineName>,
    weapon_skill_line_names: HashSet<SkillLineName>,
    required_champion_points: Vec<String>,
    champion_point_names: HashSet<String>,
    skill_names: HashSet<String>,
    parallelism: u8,

    /// Pre-split champion point combinations: (pre_resolved, ability_count, alt)
    champion_point_combinations: Vec<PreSplitBonuses>,
    /// Original champion point BonusData for final Build reconstruction
    champion_point_original: Vec<Vec<BonusData>>,
    spammable_skills: Vec<Vec<&'static SkillData>>,
    non_spammable_skills: Vec<Vec<&'static SkillData>>,
    /// Pre-split passive bonus lists: (pre_resolved, ability_count, alt)
    passive_bonuses_list: Vec<PreSplitBonuses>,
    /// Original passive BonusData for final Build reconstruction
    passive_original: Vec<Vec<BonusData>>,
    total_possible_build_count: u64,
}

// Constructor
impl BuildOptimizer {
    pub fn new(options: BuildOptimizerOptions) -> Self {
        let verbose = options.verbose;
        let parallelism = options.parallelism;
        let required_class_names = options.required_class_names;
        let required_weapon_skill_lines = options.required_weapon_skill_lines;
        let required_champion_points: Vec<String> = options
            .required_champion_points
            .iter()
            .map(|cp| cp.name.clone())
            .collect();
        let pure_class = options.pure_class;

        if verbose {
            logger::dim(&format!(
                "Total skills before morph selection: {}",
                ALL_SKILLS.len()
            ));
        }

        let skills_service = SkillsService::new(SkillsServiceOptions::default())
            .with_morph_selection(MorphSelectionOptions {
                forced_morphs: options.forced_morphs.clone(),
            })
            .with_filter(SkillsFilter {
                exclude_ultimates: true,
                exclude_non_damaging: true, // TODO: only exclude if no buff
            });

        let (class_names, class_skill_line_combinations) =
            Self::generate_class_skill_line_combinations(
                pure_class,
                &required_class_names,
                verbose,
            );

        let (weapon_skill_line_names, weapon_skill_line_combinations) =
            Self::generate_weapon_skill_line_combinations(&required_weapon_skill_lines, verbose);

        let guild_skill_lines = SkillLineName::GUILD.to_vec();
        let skill_line_combinations: Vec<Vec<SkillLineName>> = combinatorics::cartesian_product(
            &class_skill_line_combinations,
            &weapon_skill_line_combinations,
        )
        .into_iter()
        .map(|mut combo| {
            combo.extend_from_slice(&guild_skill_lines);
            combo
        })
        .collect();

        if verbose {
            logger::dim(&format!(
                "Generated {} total skill line combinations (class + weapon)",
                skill_line_combinations.len()
            ));
        }

        let mut skill_names: HashSet<String> = HashSet::new();
        let spammable_skills = Self::generate_spammable_skills(
            &skill_line_combinations,
            &skills_service,
            &mut skill_names,
        );
        let non_spammable_skills = Self::generate_non_spammable_skills(
            &skill_line_combinations,
            &skills_service,
            &mut skill_names,
            options.max_pool_size,
            &options.character_stats,
            verbose,
        );

        let has_any_spammable = spammable_skills.iter().any(|skills| !skills.is_empty());
        if !has_any_spammable {
            logger::error(
                "No spammable skills found in skill pool. Cannot optimize without a spammable.",
            );
            std::process::exit(1);
        }

        let (passive_bonuses_list, passive_original) =
            Self::generate_passive_bonuses(&skill_line_combinations, verbose);

        let (champion_point_names, champion_point_combinations, champion_point_original) =
            Self::generate_champion_point_combinations(&required_champion_points, verbose);

        let total_possible_build_count = Self::calculate_total_build_count(
            &champion_point_combinations,
            &spammable_skills,
            &non_spammable_skills,
        );

        let optimizer = Self {
            character_stats: options.character_stats,
            required_class_names,
            class_names,
            required_weapon_skill_lines,
            weapon_skill_line_names,
            required_champion_points,
            champion_point_names,
            skill_names,
            parallelism,
            champion_point_combinations,
            champion_point_original,
            spammable_skills,
            non_spammable_skills,
            passive_bonuses_list,
            passive_original,
            total_possible_build_count,
        };

        logger::log(&optimizer.to_string());

        optimizer
    }

    /// Three-way split a list of bonuses into:
    /// - pre_resolved: simple non-AbilitySlottedCount → ResolvedBonus (multiplier always 1.0)
    /// - ability_count: simple AbilitySlottedCount → BonusData (multiplier depends on skills)
    /// - alt: alternative bonuses → BonusData (need per-eval resolution)
    fn three_way_split(bonuses: Vec<BonusData>) -> PreSplitBonuses {
        let default_ctx = ResolveContext::default();
        let mut pre_resolved = Vec::new();
        let mut ability_count = Vec::new();
        let mut alt = Vec::new();

        for bonus in bonuses {
            if bonus.has_alternative() {
                alt.push(bonus);
            } else if bonus.trigger == BonusTrigger::AbilitySlottedCount {
                ability_count.push(bonus);
            } else {
                let bv = bonus.resolve_ref(&default_ctx);
                pre_resolved.push(ResolvedBonus {
                    target: bv.target,
                    value: bv.value,
                    skill_line_filter: bonus.skill_line_filter,
                    execute_threshold: bonus.execute_threshold,
                });
            }
        }

        (pre_resolved, ability_count, alt)
    }

    fn generate_champion_point_combinations(
        required_champion_points: &[String],
        verbose: bool,
    ) -> (HashSet<String>, Vec<PreSplitBonuses>, Vec<Vec<BonusData>>) {
        let mut champion_point_names: HashSet<String> = HashSet::new();
        let cp_vec: Vec<_> = CHAMPION_POINTS.iter().cloned().collect();

        let filtered: Vec<Vec<BonusData>> =
            combinatorics::generate_combinations(&cp_vec, BUILD_CONSTRAINTS.champion_point_count)
                .into_iter()
                .filter(|combination| {
                    let has_required = required_champion_points.is_empty()
                        || required_champion_points
                            .iter()
                            .all(|required| combination.iter().any(|cp| &cp.name == required));

                    if has_required {
                        for cp in combination {
                            champion_point_names.insert(cp.name.clone());
                        }
                    }

                    has_required
                })
                .collect();

        let split: Vec<PreSplitBonuses> = filtered
            .iter()
            .map(|combo| Self::three_way_split(combo.clone()))
            .collect();

        if verbose {
            let required_str = if required_champion_points.is_empty() {
                "none".to_string()
            } else {
                required_champion_points.join(", ")
            };
            logger::dim(&format!(
                "Generated {} champion point combinations using {} CPs (required: {})",
                split.len(),
                champion_point_names.len(),
                required_str
            ));
        }

        (champion_point_names, split, filtered)
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

    fn generate_spammable_skills(
        skill_line_combinations: &[Vec<SkillLineName>],
        skills_service: &SkillsService,
        skill_names: &mut HashSet<String>,
    ) -> Vec<Vec<&'static SkillData>> {
        skill_line_combinations
            .iter()
            .map(|skill_line_combination| {
                skill_line_combination
                    .iter()
                    .flat_map(|skill_line| {
                        let skills: Vec<_> = skills_service
                            .get_skills_by_skill_line(*skill_line)
                            .into_iter()
                            .filter(|s| s.spammable && s.bonuses.is_none())
                            .collect();
                        for skill in &skills {
                            skill_names.insert(skill.name.clone());
                        }
                        skills
                    })
                    .collect()
            })
            .collect()
    }

    fn generate_non_spammable_skills(
        skill_line_combinations: &[Vec<SkillLineName>],
        skills_service: &SkillsService,
        skill_names: &mut HashSet<String>,
        max_pool_size: Option<usize>,
        character_stats: &CharacterStats,
        verbose: bool,
    ) -> Vec<Vec<&'static SkillData>> {
        skill_line_combinations
            .iter()
            .map(|skill_line_combination| {
                let mut skills: Vec<&'static SkillData> = skill_line_combination
                    .iter()
                    .flat_map(|skill_line| {
                        skills_service
                            .get_skills_by_skill_line(*skill_line)
                            .into_iter()
                            .filter(|s| !(s.spammable && s.bonuses.is_none()))
                    })
                    .collect();

                // Sort by standalone damage (descending) and cap to max_pool_size
                if let Some(cap) = max_pool_size {
                    if skills.len() > cap {
                        skills.sort_by(|a, b| {
                            let da = a.calculate_damage_per_cast(&[], character_stats, None);
                            let db = b.calculate_damage_per_cast(&[], character_stats, None);
                            db.partial_cmp(&da).unwrap_or(std::cmp::Ordering::Equal)
                        });
                        if verbose {
                            logger::dim(&format!(
                                "Capping non-spammable pool from {} to {} skills",
                                skills.len(),
                                cap
                            ));
                        }
                        skills.truncate(cap);
                    }
                }

                for skill in &skills {
                    skill_names.insert(skill.name.clone());
                }
                skills
            })
            .collect()
    }

    fn generate_passive_bonuses(
        skill_line_combinations: &[Vec<SkillLineName>],
        verbose: bool,
    ) -> (Vec<PreSplitBonuses>, Vec<Vec<BonusData>>) {
        let all_used_skill_lines: HashSet<SkillLineName> =
            skill_line_combinations.iter().flatten().copied().collect();

        let passives_service =
            PassivesService::new(PassivesServiceOptions::default()).with_filter(PassivesFilter {
                skill_lines: Some(all_used_skill_lines),
            });

        let originals: Vec<Vec<BonusData>> = skill_line_combinations
            .iter()
            .map(|skill_lines| {
                skill_lines
                    .iter()
                    .flat_map(|sl| passives_service.get_passives_by_skill_line(*sl))
                    .flat_map(|passive| passive.bonuses.iter().cloned())
                    .collect()
            })
            .collect();

        let split: Vec<PreSplitBonuses> = originals
            .iter()
            .map(|all| Self::three_way_split(all.clone()))
            .collect();

        if verbose {
            logger::dim(&format!(
                "Pre-computed passive bonuses for {} skill line combinations",
                split.len()
            ));
        }

        (split, originals)
    }

    fn calculate_total_build_count(
        champion_point_combinations: &[PreSplitBonuses],
        spammable_skills: &[Vec<&'static SkillData>],
        non_spammable_skills: &[Vec<&'static SkillData>],
    ) -> u64 {
        let skill_combinations_count: u64 = spammable_skills
            .iter()
            .zip(non_spammable_skills.iter())
            .map(|(spammable, non_spammable)| {
                spammable.len() as u64
                    * combinatorics::count_combinations(
                        non_spammable.len(),
                        BUILD_CONSTRAINTS.skill_count - 1,
                    )
            })
            .sum();

        champion_point_combinations.len() as u64 * skill_combinations_count
    }
}

// Optimize
impl BuildOptimizer {
    pub fn find_optimal_build(&self) -> Option<Build> {
        logger::log(&format!("Using {} threads...", self.parallelism));

        let start_time = Instant::now();

        let evaluated_count = AtomicU64::new(0);
        let last_progress_update = AtomicU64::new(0);
        let best_damage = AtomicU64::new(0);

        let pool = ThreadPoolBuilder::new()
            .num_threads(self.parallelism as usize)
            .build()
            .expect("Failed to create thread pool");

        // Lightweight candidate — stores indices for final Build reconstruction
        struct Candidate {
            damage: f64,
            skills: SmallVec<[&'static SkillData; 10]>,
            cp_idx: usize,
            sl_idx: usize,
        }

        // Pre-collect lightweight work units to avoid par_bridge() mutex contention.
        // Each work unit is (cp_idx, skill_line_idx, spammable_idx) — a few KB total.
        let work_units = self.collect_work_units();

        let best_candidate: Option<Candidate> = pool.install(|| {
            work_units
                .par_iter()
                .map(|&(sl_idx, spam_idx)| {
                    let (passive_pre_resolved, passive_ability_count, passive_alt) =
                        &self.passive_bonuses_list[sl_idx];
                    let spammable_skill = self.spammable_skills[sl_idx][spam_idx];
                    let non_spammable = &self.non_spammable_skills[sl_idx];

                    let mut local_best = Candidate {
                        damage: f64::NEG_INFINITY,
                        skills: SmallVec::new(),
                        cp_idx: 0,
                        sl_idx,
                    };

                    // Track progress and update local best for a single evaluation
                    let mut track = |damage: f64,
                                     combo: &SmallVec<[&'static SkillData; 10]>,
                                     cp_idx: usize| {
                        let count = evaluated_count.fetch_add(1, Ordering::Relaxed) + 1;
                        if damage > local_best.damage {
                            local_best = Candidate {
                                damage,
                                skills: combo.clone(),
                                cp_idx,
                                sl_idx,
                            };
                        }
                        let _ = best_damage.fetch_max(damage.to_bits(), Ordering::Relaxed);
                        if count % 1_000_000 == 0 {
                            let last = last_progress_update.swap(count, Ordering::Relaxed);
                            if count > last {
                                let progress = (count as f64
                                    / self.total_possible_build_count as f64)
                                    * 100.0;
                                let elapsed = start_time.elapsed().as_secs_f64();
                                let eta = if progress > 0.0 {
                                    elapsed * (100.0 - progress) / progress
                                } else {
                                    0.0
                                };
                                let best =
                                    f64::from_bits(best_damage.load(Ordering::Relaxed));
                                logger::progress(&format!(
                                    "Progress: {:.1}% ({}) | Best: {} | ETA: {}",
                                    progress,
                                    format::format_number(count),
                                    format::format_number(best as u64),
                                    format::format_duration((eta * 1000.0) as u64)
                                ));
                            }
                        }
                    };

                    if self.champion_point_combinations.len() > 1 {
                        // Multiple CP combos: cache passive context per skill combo
                        for mut combo in combinatorics::CombinationIterator::new(
                            non_spammable,
                            BUILD_CONSTRAINTS.skill_count - 1,
                        ) {
                            combo.push(spammable_skill);
                            let passive_ctx = Build::cache_passive_context(
                                &combo,
                                passive_pre_resolved,
                                passive_ability_count,
                                &self.character_stats,
                            );
                            for (cp_idx, (cp_pre_resolved, cp_ability_count, cp_alt)) in
                                self.champion_point_combinations.iter().enumerate()
                            {
                                let damage = Build::compute_total_damage_cached(
                                    &combo,
                                    &passive_ctx,
                                    passive_alt,
                                    cp_pre_resolved,
                                    cp_ability_count,
                                    cp_alt,
                                );
                                track(damage, &combo, cp_idx);
                            }
                        }
                    } else {
                        // Single CP combo: direct path, no caching overhead
                        let (cp_pre_resolved, cp_ability_count, cp_alt) =
                            &self.champion_point_combinations[0];
                        for mut combo in combinatorics::CombinationIterator::new(
                            non_spammable,
                            BUILD_CONSTRAINTS.skill_count - 1,
                        ) {
                            combo.push(spammable_skill);
                            let damage = Build::compute_total_damage(
                                &combo,
                                cp_pre_resolved,
                                cp_ability_count,
                                cp_alt,
                                passive_pre_resolved,
                                passive_ability_count,
                                passive_alt,
                                &self.character_stats,
                            );
                            track(damage, &combo, 0);
                        }
                    }

                    local_best
                })
                .reduce_with(|a, b| if a.damage > b.damage { a } else { b })
        });

        let total_evaluated = evaluated_count.load(Ordering::Relaxed);
        let elapsed = start_time.elapsed();

        logger::log(&format!(
            "Completed: {} builds evaluated in {:.1}s",
            format::format_number(total_evaluated),
            elapsed.as_secs_f64()
        ));

        // Construct the full Build only for the winner using original BonusData
        best_candidate.map(|c| {
            let cp_bonuses = &self.champion_point_original[c.cp_idx];
            let passive_bonuses = &self.passive_original[c.sl_idx];

            Build::new(
                c.skills.to_vec(),
                cp_bonuses,
                passive_bonuses,
                self.character_stats.clone(),
            )
        })
    }

    /// Pre-collect lightweight work unit indices: (skill_line_idx, spammable_idx).
    /// Skills are outermost; each work unit iterates all CP combos internally.
    fn collect_work_units(&self) -> Vec<(usize, usize)> {
        let mut units = Vec::new();
        for sl_idx in 0..self.spammable_skills.len() {
            for spam_idx in 0..self.spammable_skills[sl_idx].len() {
                units.push((sl_idx, spam_idx));
            }
        }
        units
    }
}

// Format
impl BuildOptimizer {
    fn fmt_configuration_table(&self) -> String {
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

        table::table(
            &config_data,
            table::TableOptions {
                title: Some("Configuration".to_string()),
                columns: vec![
                    table::ColumnDefinition::new("Constraint", 25),
                    table::ColumnDefinition::new("Value", 10).align_right(),
                ],
                footer: None,
            },
        )
    }

    fn fmt_sorted_list<T: ToString>(items: impl IntoIterator<Item = T>) -> String {
        let mut sorted: Vec<_> = items.into_iter().map(|i| i.to_string()).collect();
        if sorted.is_empty() {
            "None".to_string()
        } else {
            sorted.sort();
            sorted.join(", ")
        }
    }

    fn fmt_skill_line_configuration_table(&self) -> String {
        let used_classes_str = Self::fmt_sorted_list(self.class_names.iter());
        let required_classes_str = Self::fmt_sorted_list(self.required_class_names.iter());
        let used_weapons_str = Self::fmt_sorted_list(self.weapon_skill_line_names.iter());
        let required_weapons_str = Self::fmt_sorted_list(self.required_weapon_skill_lines.iter());
        let used_cp_str = Self::fmt_sorted_list(self.champion_point_names.iter());
        let required_cp_str = Self::fmt_sorted_list(self.required_champion_points.iter());

        let name_width = [
            &used_classes_str,
            &required_classes_str,
            &used_weapons_str,
            &required_weapons_str,
            &used_cp_str,
            &required_cp_str,
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
            vec!["Used Champion Points".to_string(), used_cp_str],
            vec!["Required Champion Points".to_string(), required_cp_str],
            vec![
                "Used Skills".to_string(),
                self.skill_names.len().to_string(),
            ],
        ];

        table::table(
            &skill_config_data,
            table::TableOptions {
                title: Some("Skill Line Configuration".to_string()),
                columns: vec![
                    table::ColumnDefinition::new("Setting", 25),
                    table::ColumnDefinition::new("Name", name_width),
                ],
                footer: None,
            },
        )
    }
}

impl std::fmt::Display for BuildOptimizer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut lines = Vec::new();
        lines.push(self.fmt_configuration_table());
        lines.push(self.fmt_skill_line_configuration_table());
        lines.push(format!(
            "Total builds to be evaluated: {}",
            format::format_number(self.total_possible_build_count)
        ));
        write!(f, "{}", lines.join("\n"))
    }
}
