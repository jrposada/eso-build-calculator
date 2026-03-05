use crate::data::bonuses::{TRIAL_BUFF_NAMES, TRIAL_DUMMY_BUFFS};
use crate::data::sets::ALL_SETS;
use crate::data::skill_trees::armor::armor_passives;
use crate::data::skill_trees::guild::undaunted::undaunted_passives::undaunted_mettle_bonuses;
use crate::domain::{
    ArmorDistribution, ArmorWeight, BonusData, Build, BuildConfig, BuildMetadata, CharacterStats,
    ClassName, Potion, SetData, SetProcEffect, SimulationResult, SkillData, WeaponEnchant,
};
use crate::infrastructure::{format, logger};
use crate::services::{
    format_armor_traits, format_jewelry_traits, generate_distributions, infer_weapons,
    stats_differ_significantly, BarDistribution, BuildOptimizer, BuildOptimizerOptions,
    FightSimulator, GearOptimizer, GearOptimizerOptions, SetOptimizer, SetOptimizerOptions,
};
use rayon::prelude::*;
use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use std::time::Instant;

pub struct OptimizePipelineOptions {
    pub verbose: bool,
    pub pure: bool,
    pub parallelism: u8,
    pub max_pool_size: Option<usize>,
    pub baseline: BuildConfig,
    pub trial: bool,
    pub avg_resource_pct: f64,
    pub required_weapon_skill_lines: Vec<crate::domain::SkillLineName>,
}

/// Result of the optimization pipeline. Serializes to the same JSON shape as BuildConfig.
pub struct OptimizePipelineResult {
    pub build_config: BuildConfig,
    pub simulation: Option<SimulationSummary>,
}

pub struct SimulationSummary {
    pub bar_distribution: BarDistribution,
    pub result: SimulationResult,
    pub distributions_tested: usize,
    pub set_names: Vec<(String, u8)>,
}

pub struct OptimizePipeline;

impl OptimizePipeline {
    pub fn run(options: OptimizePipelineOptions) -> OptimizePipelineResult {
        let character_stats = options.baseline.compute_stats();
        let baseline_stats = character_stats.clone();

        // Resolve constraints from baseline
        let required_skills: Vec<&'static SkillData> = options
            .baseline
            .skills
            .iter()
            .map(|name| {
                SkillData::parse(name).unwrap_or_else(|e| panic!("Invalid skill '{}': {}", name, e))
            })
            .collect();
        let required_champion_points: Vec<BonusData> = options
            .baseline
            .champion_points
            .iter()
            .map(|name| {
                BonusData::parse_champion_point(name)
                    .unwrap_or_else(|e| panic!("Invalid CP '{}': {}", name, e))
            })
            .collect();
        let pinned_sets: Vec<&'static SetData> = options
            .baseline
            .sets
            .iter()
            .map(|name| {
                SetData::parse(name).unwrap_or_else(|e| panic!("Invalid set '{}': {}", name, e))
            })
            .collect();
        let required_class_names: Vec<ClassName> = options.baseline.classes.clone();
        let required_weapon_skill_lines: Vec<crate::domain::SkillLineName> = {
            let mut lines: Vec<crate::domain::SkillLineName> = [
                options.baseline.bar1_weapon,
                options.baseline.bar2_weapon,
            ]
            .iter()
            .filter_map(|w| w.map(|wt| wt.skill_line()))
            .chain(options.required_weapon_skill_lines.iter().copied())
            .collect();
            lines.dedup();
            lines
        };

        // Resolve pinned set bonuses for Phase 0
        let (set_bonuses, set_names, _set_proc_effects) = resolve_set_bonuses(&pinned_sets);

        // Resolve trial dummy buffs
        let extra_bonuses = if options.trial {
            TRIAL_DUMMY_BUFFS.clone()
        } else {
            Vec::new()
        };

        // Resolve armor passives and potion bonuses
        let completions = options.baseline.armor.completions();

        // Deduplicate completions by (dominant_weight, type_count)
        let mut unique_passive_sets: Vec<(Option<ArmorWeight>, u8, ArmorDistribution)> = Vec::new();
        for c in &completions {
            let key = (c.dominant_weight(), c.type_count());
            if !unique_passive_sets
                .iter()
                .any(|(dw, tc, _)| *dw == key.0 && *tc == key.1)
            {
                unique_passive_sets.push((key.0, key.1, *c));
            }
        }

        // Use the best-guess heuristic: prefer medium dominant + 3-type, else first completion
        let best_guess = unique_passive_sets
            .iter()
            .find(|(dw, tc, _)| *dw == Some(ArmorWeight::Medium) && *tc == 3)
            .or_else(|| unique_passive_sets.first())
            .map(|(_, _, c)| *c)
            .unwrap_or(options.baseline.armor);

        let mut armor_passive_bonuses = if let Some(dw) = best_guess.dominant_weight() {
            armor_passives(dw)
        } else {
            Vec::new()
        };
        let potion = options.baseline.potion.unwrap_or(Potion::WeaponPower);
        armor_passive_bonuses.extend(undaunted_mettle_bonuses(best_guess.type_count()));
        armor_passive_bonuses.extend(potion.bonuses());

        // ── Phase 0: BuildOptimizer with baseline stats ──
        logger::info("Phase 0: Finding optimal skill/CP build...");

        let optimizer = BuildOptimizer::new(BuildOptimizerOptions {
            character_stats,
            verbose: options.verbose,
            pure: options.pure,
            required_class_names: required_class_names.clone(),
            required_weapon_skill_lines: required_weapon_skill_lines.clone(),
            required_champion_points: required_champion_points.clone(),
            required_skills: required_skills.clone(),
            parallelism: options.parallelism,
            max_pool_size: options.max_pool_size,
            set_bonuses,
            set_names,
            extra_bonuses: extra_bonuses.clone(),
            armor_passive_bonuses: armor_passive_bonuses.clone(),
        });

        let start = Instant::now();
        let mut builds = optimizer.find_optimal_build();
        let elapsed = start.elapsed();

        if builds.is_empty() {
            logger::error("No valid build found with the given constraints.");
            std::process::exit(1);
        }

        logger::info(&builds[0].to_string());
        logger::info(&std::format!("Phase 0 completed in {:.2?}", elapsed));

        // ── Free-slot armor optimization ──
        let winning_armor = if unique_passive_sets.len() > 1 {
            let mut best_armor = best_guess;
            let mut best_dpc = builds[0].total_damage_per_cast;
            let source = &builds[0];

            for &(dw, tc, dist) in &unique_passive_sets {
                let mut passives = if let Some(weight) = dw {
                    armor_passives(weight)
                } else {
                    Vec::new()
                };
                passives.extend(undaunted_mettle_bonuses(tc));
                passives.extend(potion.bonuses());

                let build = Build::new_with_extra(
                    source.skills().to_vec(),
                    source.cp_bonuses(),
                    &passives,
                    &[],
                    Vec::new(),
                    source.character_stats().clone(),
                    &extra_bonuses,
                );
                if build.total_damage_per_cast > best_dpc {
                    best_dpc = build.total_damage_per_cast;
                    best_armor = dist;
                }
            }

            if best_armor != best_guess {
                logger::info(&std::format!(
                    "Armor optimization: {} beats default (free slots filled optimally)",
                    best_armor
                ));
                armor_passive_bonuses = if let Some(dw) = best_armor.dominant_weight() {
                    armor_passives(dw)
                } else {
                    Vec::new()
                };
                armor_passive_bonuses.extend(undaunted_mettle_bonuses(best_armor.type_count()));
                armor_passive_bonuses.extend(potion.bonuses());
            }
            best_armor
        } else {
            best_guess
        };

        // ── Phase 1: Gear Optimization ──
        let gear_options = GearOptimizerOptions {
            top_k: 3,
            verbose: options.verbose,
        };

        let winning_gear = if gear_options.all_pinned(&options.baseline) {
            if options.verbose {
                logger::dim("Phase 1: All gear dimensions pinned, skipping gear optimization.");
            }
            None
        } else {
            logger::info("Phase 1: Optimizing gear (race, mundus, food, traits)...");
            let gear_start = Instant::now();
            let result = GearOptimizer::optimize(&builds, &gear_options, &options.baseline);
            let gear_elapsed = gear_start.elapsed();

            let g = &result.build_config;
            logger::success(&std::format!(
                "Best gear: Race={}, Mundus={}, Food={}, Armor={}, Jewelry={}, Weapon={}, Attributes={}",
                g.race.map_or("None".to_string(), |r| r.to_string()),
                g.mundus.map_or("None".to_string(), |m| m.to_string()),
                g.food.map_or("None".to_string(), |f| f.to_string()),
                format_armor_traits(&g.armor_traits),
                format_jewelry_traits(&g.jewelry_traits),
                g.weapon_traits.first().map_or("None".to_string(), |t| t.to_string()),
                g.attributes.map_or("None".to_string(), |a| a.to_string()),
            ));
            logger::info(&std::format!("Phase 1 completed in {:.2?}", gear_elapsed));
            Some(result)
        };

        // ── Phase 2: Conditional BuildOptimizer re-run ──
        if let Some(ref gear_result) = winning_gear {
            let new_stats = gear_result.character_stats.clone();

            if stats_differ_significantly(&baseline_stats, &new_stats, 0.05) {
                logger::info("Phase 2: Gear stats changed >5%, re-running build optimizer...");

                let (set_bonuses, set_names, _set_proc_effects) = resolve_set_bonuses(&pinned_sets);
                let rerun_optimizer = BuildOptimizer::new(BuildOptimizerOptions {
                    character_stats: new_stats,
                    verbose: options.verbose,
                    pure: options.pure,
                    required_class_names: required_class_names.clone(),
                    required_weapon_skill_lines: required_weapon_skill_lines.clone(),
                    required_champion_points: required_champion_points.clone(),
                    required_skills: required_skills.clone(),
                    parallelism: options.parallelism,
                    max_pool_size: options.max_pool_size,
                    set_bonuses,
                    set_names,
                    extra_bonuses: extra_bonuses.clone(),
                    armor_passive_bonuses: armor_passive_bonuses.clone(),
                });

                let rerun_start = Instant::now();
                let new_builds = rerun_optimizer.find_optimal_build();
                let rerun_elapsed = rerun_start.elapsed();

                if !new_builds.is_empty() {
                    logger::info(&new_builds[0].to_string());
                    logger::info(&std::format!("Phase 2 completed in {:.2?}", rerun_elapsed));
                    builds = new_builds;
                } else {
                    logger::warn("Phase 2 re-run found no valid builds, keeping Phase 0 results.");
                }
            } else if options.verbose {
                logger::dim("Phase 2: Stats within 5% of baseline, skipping re-run.");
            }
        }

        // ── Phase 3: Set Optimization (always runs) ──
        logger::info("Phase 3: Optimizing gear sets...");
        let (pinned_normal, pinned_monster, pinned_mythic_vec) =
            SetData::split_by_type(&pinned_sets);
        let set_result = SetOptimizer::optimize(
            &builds,
            &SetOptimizerOptions {
                top_k: 10,
                pinned_normal,
                pinned_monster,
                pinned_mythic: pinned_mythic_vec.into_iter().next(),
                parallelism: options.parallelism,
                verbose: options.verbose,
            },
        );
        let builds = if let Some(result) = set_result {
            let source = &builds[result.build_idx];
            let best_with_sets = Build::new_with_extra(
                source.skills().to_vec(),
                source.cp_bonuses(),
                source.passive_bonuses(),
                &result.set_bonuses,
                result.set_names,
                source.character_stats().clone(),
                &extra_bonuses,
            );
            logger::info(&best_with_sets.to_string());
            vec![best_with_sets]
        } else {
            logger::warn("Set optimization found no valid loadout.");
            builds
        };

        // ── Phase 4: Fight Simulation ──
        let sim_result = run_simulation(&options, &builds);

        let best_build = &builds[0];
        let export_build = sim_result
            .as_ref()
            .map(|(build_idx, _, _, _, _, _)| &builds[*build_idx])
            .unwrap_or(best_build);
        let sim_data = sim_result
            .as_ref()
            .map(|(_, dist, result, _, _, _)| (dist, result));
        let buffed_stats = sim_result.as_ref().map(|(_, _, _, _, _, stats)| stats);

        let (winning_bar1, winning_bar2) = sim_result
            .as_ref()
            .map(|(_, _, _, e1, e2, _)| (*e1, *e2))
            .unwrap_or((
                options
                    .baseline
                    .bar1_enchant
                    .unwrap_or(WeaponEnchant::Flame),
                options
                    .baseline
                    .bar2_enchant
                    .unwrap_or(WeaponEnchant::Flame),
            ));

        let winning_build_config = winning_gear.as_ref().map(|g| &g.build_config);

        // Build the metadata from simulation data
        let metadata = sim_data.map(|(dist, result)| BuildMetadata {
            dps: result.dps,
            total_damage: result.total_damage,
            fight_duration: result.fight_duration,
            bar1_skills: dist
                .bar1
                .skills
                .iter()
                .map(|s| s.name.to_string())
                .collect(),
            bar2_skills: dist
                .bar2
                .skills
                .iter()
                .map(|s| s.name.to_string())
                .collect(),
            buffed_stats: buffed_stats.cloned(),
        });

        let build_config = BuildConfig {
            skills: export_build.skill_names(),
            champion_points: export_build.champion_point_names(),
            sets: export_build
                .set_names()
                .iter()
                .map(|(name, _)| name.clone())
                .collect(),
            classes: options.baseline.classes.clone(),
            bar1_weapon: options.baseline.bar1_weapon,
            bar2_weapon: options.baseline.bar2_weapon,
            character_stats: export_build.character_stats().clone(),
            race: winning_build_config.and_then(|g| g.race),
            mundus: winning_build_config.and_then(|g| g.mundus),
            food: winning_build_config.and_then(|g| g.food),
            armor_traits: winning_build_config
                .map(|g| g.armor_traits.clone())
                .unwrap_or_else(|| options.baseline.armor_traits.clone()),
            jewelry_traits: winning_build_config
                .map(|g| g.jewelry_traits.clone())
                .unwrap_or_else(|| options.baseline.jewelry_traits.clone()),
            weapon_traits: winning_build_config
                .map(|g| g.weapon_traits.clone())
                .unwrap_or_else(|| options.baseline.weapon_traits.clone()),
            bar1_enchant: Some(winning_bar1),
            bar2_enchant: Some(winning_bar2),
            armor: winning_armor,
            potion: Some(potion),
            attributes: winning_build_config
                .and_then(|g| g.attributes)
                .or(options.baseline.attributes),
            metadata,
        };

        // Build simulation summary for display
        let simulation =
            sim_result.map(
                |(best_build_idx, best_dist, result, _, _, _)| SimulationSummary {
                    bar_distribution: best_dist,
                    result,
                    distributions_tested: 0, // set below if needed
                    set_names: builds[best_build_idx].set_names().to_vec(),
                },
            );

        OptimizePipelineResult {
            build_config,
            simulation,
        }
    }
}

pub(crate) fn resolve_set_bonuses(
    sets: &[&'static SetData],
) -> (Vec<BonusData>, Vec<(String, u8)>, Vec<SetProcEffect>) {
    let mut set_bonuses: Vec<BonusData> = Vec::new();
    let mut set_names: Vec<(String, u8)> = Vec::new();
    let mut set_proc_effects: Vec<SetProcEffect> = Vec::new();
    for set in sets {
        let piece_count = set.set_type.max_pieces();
        let bonuses = set.bonuses_at(piece_count);
        set_bonuses.extend(bonuses.into_iter().cloned());
        set_proc_effects.extend(set.proc_effects_at(piece_count).into_iter().cloned());
        set_names.push((set.name.clone(), piece_count));
    }
    (set_bonuses, set_names, set_proc_effects)
}

fn run_simulation(
    options: &OptimizePipelineOptions,
    builds: &[Build],
) -> Option<(
    usize,
    BarDistribution,
    SimulationResult,
    WeaponEnchant,
    WeaponEnchant,
    CharacterStats,
)> {
    // Derive bar weapons from options, or infer from skills
    let pinned_bar1 = options.baseline.bar1_weapon;
    let pinned_bar2 = options.baseline.bar2_weapon;

    let inferred = {
        let top_skills = builds[0].skills();
        match infer_weapons(top_skills) {
            Ok(weapons) => Some(weapons),
            Err(e) => {
                if pinned_bar1.is_none() && pinned_bar2.is_none() {
                    logger::warn(&std::format!(
                        "Could not infer weapons for simulation: {}. Skipping fight simulation.",
                        e
                    ));
                    return None;
                }
                None
            }
        }
    };

    let (bar1_weapon, bar2_weapon) = match (pinned_bar1, pinned_bar2) {
        (Some(w1), Some(w2)) => (w1, w2),
        (Some(w1), None) => {
            let w2 = inferred.map(|(_, w2)| w2).unwrap_or(w1);
            (w1, w2)
        }
        (None, Some(w2)) => {
            let w1 = inferred.map(|(w1, _)| w1).unwrap_or(w2);
            (w1, w2)
        }
        (None, None) => match inferred {
            Some((w1, w2)) => (w1, w2),
            None => return None,
        },
    };

    logger::info(&std::format!(
        "Phase 4: Running fight simulation on top {} candidates (Bar1: {}, Bar2: {})...",
        builds.len(),
        bar1_weapon,
        bar2_weapon
    ));

    let sim_start = Instant::now();

    // Pre-compute work items
    let work: Vec<(usize, FightSimulator, Vec<BarDistribution>)> = builds
        .iter()
        .enumerate()
        .filter_map(|(build_idx, build)| {
            let distributions = generate_distributions(build.skills(), bar1_weapon, bar2_weapon);
            if distributions.is_empty() {
                return None;
            }
            let mut suppressed = if options.trial {
                TRIAL_BUFF_NAMES.clone()
            } else {
                std::collections::HashSet::new()
            };
            let potion = options.baseline.potion.unwrap_or(Potion::WeaponPower);
            for bonus in potion.bonuses() {
                suppressed.insert(bonus.name.clone());
            }
            let bar1_enchant = options.baseline.bar1_enchant.or(Some(WeaponEnchant::Flame));
            let bar2_enchant = options.baseline.bar2_enchant.or(Some(WeaponEnchant::Flame));
            let proc_effects: Vec<SetProcEffect> = build
                .set_names()
                .iter()
                .flat_map(|(name, _)| {
                    ALL_SETS
                        .iter()
                        .filter(move |s| s.name == *name)
                        .flat_map(|s| {
                            s.proc_effects_at(s.set_type.max_pieces())
                                .into_iter()
                                .cloned()
                        })
                })
                .collect();
            let simulator = FightSimulator::new(
                build.effective_stats(),
                build.resolved_bonuses(),
                suppressed,
            )
            .with_enchants(bar1_enchant, bar2_enchant)
            .with_set_procs(proc_effects)
            .with_avg_resource_pct(options.avg_resource_pct);
            Some((build_idx, simulator, distributions))
        })
        .collect();

    let total_sims: usize = work.iter().map(|(_, _, d)| d.len()).sum();
    let completed = AtomicUsize::new(0);
    let best_dps_bits = AtomicU64::new(f64::NEG_INFINITY.to_bits());

    let results: Vec<_> = work
        .par_iter()
        .filter_map(|(build_idx, simulator, distributions)| {
            let mut local_best: Option<(usize, SimulationResult)> = None;
            for (dist_idx, dist) in distributions.iter().enumerate() {
                let result = simulator.simulate(dist);
                if local_best
                    .as_ref()
                    .map_or(true, |(_, r)| result.dps > r.dps)
                {
                    local_best = Some((dist_idx, result));
                }

                let done = completed.fetch_add(1, Ordering::Relaxed) + 1;

                if let Some((_, ref best)) = local_best {
                    let new_bits = best.dps.to_bits();
                    let mut current = best_dps_bits.load(Ordering::Relaxed);
                    loop {
                        if f64::from_bits(current) >= best.dps {
                            break;
                        }
                        match best_dps_bits.compare_exchange_weak(
                            current,
                            new_bits,
                            Ordering::Relaxed,
                            Ordering::Relaxed,
                        ) {
                            Ok(_) => break,
                            Err(actual) => current = actual,
                        }
                    }
                }

                if done % 10 == 0 || done == total_sims {
                    let best = f64::from_bits(best_dps_bits.load(Ordering::Relaxed));
                    let best_str = if best > 0.0 {
                        format::format_number(best as u64)
                    } else {
                        "---".to_string()
                    };
                    logger::progress(&std::format!(
                        "Simulating: {}/{} | Best DPS: {}",
                        done,
                        total_sims,
                        best_str
                    ));
                }
            }
            local_best
                .map(|(dist_idx, result)| (*build_idx, dist_idx, distributions.clone(), result))
        })
        .collect();

    let sim_elapsed = sim_start.elapsed();

    if let Some((best_build_idx, best_dist_idx, distributions, mut result)) = results
        .into_iter()
        .max_by(|(_, _, _, a), (_, _, _, b)| a.dps.partial_cmp(&b.dps).unwrap())
    {
        if best_build_idx > 0 {
            logger::info(&std::format!(
                "Simulation selected build #{} (of {} candidates) as best DPS.",
                best_build_idx + 1,
                builds.len()
            ));
            logger::info(&builds[best_build_idx].to_string());
        }
        let best_dist = distributions[best_dist_idx].clone();

        // ── Enchant optimization sweep ──
        let bar1_pinned = options.baseline.bar1_enchant.is_some();
        let bar2_pinned = options.baseline.bar2_enchant.is_some();
        let mut winning_bar1 = options
            .baseline
            .bar1_enchant
            .unwrap_or(WeaponEnchant::Flame);
        let mut winning_bar2 = options
            .baseline
            .bar2_enchant
            .unwrap_or(WeaponEnchant::Flame);

        if !bar1_pinned || !bar2_pinned {
            let all_enchants = [
                WeaponEnchant::Flame,
                WeaponEnchant::Poison,
                WeaponEnchant::Shock,
                WeaponEnchant::Berserker,
            ];
            let bar1_candidates: Vec<WeaponEnchant> = if bar1_pinned {
                vec![winning_bar1]
            } else {
                all_enchants.to_vec()
            };
            let bar2_candidates: Vec<WeaponEnchant> = if bar2_pinned {
                vec![winning_bar2]
            } else {
                all_enchants.to_vec()
            };

            let build = &builds[best_build_idx];
            let mut suppressed = if options.trial {
                TRIAL_BUFF_NAMES.clone()
            } else {
                std::collections::HashSet::new()
            };
            let potion = options.baseline.potion.unwrap_or(Potion::WeaponPower);
            for bonus in potion.bonuses() {
                suppressed.insert(bonus.name.clone());
            }
            let proc_effects: Vec<SetProcEffect> = build
                .set_names()
                .iter()
                .flat_map(|(name, _)| {
                    ALL_SETS
                        .iter()
                        .filter(move |s| s.name == *name)
                        .flat_map(|s| {
                            s.proc_effects_at(s.set_type.max_pieces())
                                .into_iter()
                                .cloned()
                        })
                })
                .collect();

            let combo_count = bar1_candidates.len() * bar2_candidates.len();
            let mut best_enchant_dps = result.dps;

            for &e1 in &bar1_candidates {
                for &e2 in &bar2_candidates {
                    if e1 == winning_bar1 && e2 == winning_bar2 {
                        continue;
                    }
                    let sim = FightSimulator::new(
                        build.effective_stats(),
                        build.resolved_bonuses(),
                        suppressed.clone(),
                    )
                    .with_enchants(Some(e1), Some(e2))
                    .with_set_procs(proc_effects.clone())
                    .with_avg_resource_pct(options.avg_resource_pct);

                    let r = sim.simulate(&best_dist);
                    if r.dps > best_enchant_dps {
                        best_enchant_dps = r.dps;
                        winning_bar1 = e1;
                        winning_bar2 = e2;
                        result = r;
                    }
                }
            }

            logger::success(&std::format!(
                "Best enchants: Bar1={}, Bar2={} (optimized from {} combos)",
                winning_bar1,
                winning_bar2,
                combo_count
            ));
        }

        // Display simulation results inline
        {
            use crate::services::simulate_pipeline::SimulatePipelineResult;
            let display_result = SimulatePipelineResult {
                build_summary: String::new(),
                simulation: result.clone(),
                best_distribution: best_dist.clone(),
                distributions_tested: distributions.len(),
                set_names: builds[best_build_idx].set_names().to_vec(),
                buffed_stats: None,
                warnings: Vec::new(),
            };
            logger::info(&display_result.to_string());
        }
        logger::info(&std::format!("Simulation completed in {:.2?}", sim_elapsed));

        // Compute buffed stats for export metadata
        let build = &builds[best_build_idx];
        let mut suppressed_final = if options.trial {
            TRIAL_BUFF_NAMES.clone()
        } else {
            std::collections::HashSet::new()
        };
        let potion = options.baseline.potion.unwrap_or(Potion::WeaponPower);
        for bonus in potion.bonuses() {
            suppressed_final.insert(bonus.name.clone());
        }
        let proc_effects_final: Vec<SetProcEffect> = build
            .set_names()
            .iter()
            .flat_map(|(name, _)| {
                ALL_SETS
                    .iter()
                    .filter(move |s| s.name == *name)
                    .flat_map(|s| {
                        s.proc_effects_at(s.set_type.max_pieces())
                            .into_iter()
                            .cloned()
                    })
            })
            .collect();
        let final_sim = FightSimulator::new(
            build.effective_stats(),
            build.resolved_bonuses(),
            suppressed_final,
        )
        .with_enchants(Some(winning_bar1), Some(winning_bar2))
        .with_set_procs(proc_effects_final)
        .with_avg_resource_pct(options.avg_resource_pct);
        let buffed_stats = final_sim.compute_buffed_stats(&best_dist);

        return Some((
            best_build_idx,
            best_dist,
            result,
            winning_bar1,
            winning_bar2,
            buffed_stats,
        ));
    }

    None
}
