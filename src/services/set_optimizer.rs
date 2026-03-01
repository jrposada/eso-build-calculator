use crate::data::sets::ALL_SETS;
use crate::domain::{BonusData, Build, SetData, SetType};
use crate::infrastructure::{format, logger};
use rayon::prelude::*;
use rayon::ThreadPoolBuilder;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Instant;

pub struct SetOptimizerOptions {
    pub top_k: usize,
    pub pinned_normal: Vec<&'static SetData>,
    pub pinned_monster: Vec<&'static SetData>,
    pub pinned_mythic: Option<&'static SetData>,
    pub parallelism: u8,
    pub verbose: bool,
}

pub struct SetOptimizerResult {
    pub build_idx: usize,
    pub set_bonuses: Vec<BonusData>,
    pub set_names: Vec<String>,
    pub damage: f64,
}

pub struct SetOptimizer;

impl SetOptimizer {
    pub fn optimize(builds: &[Build], options: &SetOptimizerOptions) -> Option<SetOptimizerResult> {
        let max_normal_slots = 2 - options.pinned_normal.len();
        let has_pinned_mythic = options.pinned_mythic.is_some();

        // Collect available sets per slot type (exclude pinned)
        let pinned_normal_names: Vec<&str> =
            options.pinned_normal.iter().map(|s| s.name.as_str()).collect();
        let pinned_monster_names: Vec<&str> = options
            .pinned_monster
            .iter()
            .map(|s| s.name.as_str())
            .collect();

        let available_normals: Vec<&'static SetData> = ALL_SETS
            .iter()
            .filter(|s| {
                (s.set_type == SetType::Normal || s.set_type == SetType::Arena)
                    && !pinned_normal_names.contains(&s.name.as_str())
            })
            .copied()
            .collect();
        let available_monsters: Vec<&'static SetData> = ALL_SETS
            .iter()
            .filter(|s| {
                s.set_type == SetType::Monster
                    && !pinned_monster_names.contains(&s.name.as_str())
            })
            .copied()
            .collect();
        let available_mythics: Vec<&'static SetData> = if has_pinned_mythic {
            Vec::new()
        } else {
            ALL_SETS
                .iter()
                .filter(|s| s.set_type == SetType::Mythic)
                .copied()
                .collect()
        };

        if options.verbose {
            logger::dim(&format!(
                "Set optimizer: {} normals, {} monsters, {} mythics available (pinned: {} normal, {} monster, {} mythic)",
                available_normals.len(),
                available_monsters.len(),
                available_mythics.len(),
                options.pinned_normal.len(),
                options.pinned_monster.len(),
                if has_pinned_mythic { 1 } else { 0 },
            ));
        }

        // Pre-resolve pinned set bonuses (always included)
        let pinned_bonuses: Vec<BonusData> = options
            .pinned_normal
            .iter()
            .chain(options.pinned_monster.iter())
            .chain(options.pinned_mythic.iter())
            .flat_map(|set| {
                set.bonuses_at(set.set_type.max_pieces())
                    .into_iter()
                    .cloned()
            })
            .collect();
        let pool = ThreadPoolBuilder::new()
            .num_threads(options.parallelism as usize)
            .build()
            .expect("Failed to create thread pool");

        let start_time = Instant::now();
        let top_k = options.top_k;

        // Phase 1: Score each set individually per build, keep top-K per slot
        logger::info("Scoring sets...");

        let scored_count = AtomicU64::new(0);

        // For each build, score all available sets and keep top-K per slot
        let per_build_topk: Vec<(
            Vec<(&'static SetData, f64)>,
            Vec<(&'static SetData, f64)>,
            Vec<(&'static SetData, f64)>,
        )> = pool.install(|| {
            builds
                .par_iter()
                .map(|build| {
                    let cp = build.cp_bonuses();
                    let passive = build.passive_bonuses();
                    let stats = build.character_stats();
                    let skills = build.skills().to_vec();

                    // Baseline: build with only pinned sets
                    let baseline = Build::new(
                        skills.clone(),
                        cp,
                        passive,
                        &pinned_bonuses,
                        Vec::new(),
                        stats.clone(),
                    );
                    let baseline_damage = baseline.total_damage_per_cast;

                    let score_set = |set: &'static SetData| -> f64 {
                        let mut bonuses = pinned_bonuses.clone();
                        bonuses.extend(
                            set.bonuses_at(set.set_type.max_pieces())
                                .into_iter()
                                .cloned(),
                        );
                        let b = Build::new(
                            skills.clone(),
                            cp,
                            passive,
                            &bonuses,
                            Vec::new(),
                            stats.clone(),
                        );
                        scored_count.fetch_add(1, Ordering::Relaxed);
                        b.total_damage_per_cast - baseline_damage
                    };

                    let mut normal_scores: Vec<(&'static SetData, f64)> = available_normals
                        .iter()
                        .map(|&s| (s, score_set(s)))
                        .collect();
                    normal_scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
                    normal_scores.truncate(top_k.min(max_normal_slots * top_k));

                    let mut monster_scores: Vec<(&'static SetData, f64)> = available_monsters
                        .iter()
                        .map(|&s| (s, score_set(s)))
                        .collect();
                    monster_scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
                    monster_scores.truncate(top_k);

                    let mut mythic_scores: Vec<(&'static SetData, f64)> = available_mythics
                        .iter()
                        .map(|&s| (s, score_set(s)))
                        .collect();
                    mythic_scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
                    mythic_scores.truncate(top_k);

                    (normal_scores, monster_scores, mythic_scores)
                })
                .collect()
        });

        let scoring_elapsed = start_time.elapsed();
        let total_scored = scored_count.load(Ordering::Relaxed);
        logger::info(&format!(
            "Scored {} sets in {:.2?}",
            format::format_number(total_scored),
            scoring_elapsed,
        ));

        // Phase 2: Cross-product top-K finalists per build
        logger::info("Evaluating set loadouts...");
        let phase2_start = Instant::now();
        let evaluated_count = AtomicU64::new(0);
        let best_damage_bits = AtomicU64::new(0);

        struct LoadoutCandidate {
            build_idx: usize,
            set_bonuses: Vec<BonusData>,
            set_names: Vec<String>,
            damage: f64,
        }

        let results: Vec<Option<LoadoutCandidate>> = pool.install(|| {
            builds
                .par_iter()
                .enumerate()
                .map(|(build_idx, build)| {
                    let cp = build.cp_bonuses();
                    let passive = build.passive_bonuses();
                    let stats = build.character_stats();
                    let skills = build.skills().to_vec();

                    let (ref top_normals, ref top_monsters, ref top_mythics) =
                        per_build_topk[build_idx];

                    let mut best: Option<LoadoutCandidate> = None;

                    // Pinned sets are always in every loadout
                    let pinned_normal = &options.pinned_normal;
                    let pinned_monster = &options.pinned_monster;
                    let pinned_mythic = &options.pinned_mythic;

                    let free_normal_slots = 2 - pinned_normal.len();

                    // Variable normal candidates (top-K from scoring phase)
                    let var_normals: Vec<&'static SetData> =
                        top_normals.iter().map(|(s, _)| *s).collect();

                    // Generate variable normal fills for remaining slots
                    let normal_fills: Vec<Vec<&'static SetData>> = match free_normal_slots {
                        0 => vec![vec![]], // all pinned, no variable slots
                        1 => {
                            let mut fills: Vec<Vec<&'static SetData>> = var_normals
                                .iter()
                                .map(|&s| vec![s])
                                .collect();
                            fills.push(vec![]); // allow empty
                            fills
                        }
                        2 => {
                            let mut fills = Vec::new();
                            // pairs
                            for i in 0..var_normals.len() {
                                for j in (i + 1)..var_normals.len() {
                                    fills.push(vec![var_normals[i], var_normals[j]]);
                                }
                            }
                            // singles
                            for &s in &var_normals {
                                fills.push(vec![s]);
                            }
                            // none
                            fills.push(vec![]);
                            fills
                        }
                        _ => vec![vec![]],
                    };

                    // Monster: if pinned, only pinned; else top-K + None
                    let monster_options: Vec<Option<&'static SetData>> = if !pinned_monster.is_empty() {
                        vec![None] // pinned monster always included, no variable choice
                    } else {
                        let mut opts: Vec<Option<&'static SetData>> = top_monsters
                            .iter()
                            .map(|(s, _)| Some(*s))
                            .collect();
                        opts.push(None);
                        opts
                    };

                    // Mythic: if pinned, only pinned; else top-K + None
                    let mythic_options: Vec<Option<&'static SetData>> = if pinned_mythic.is_some() {
                        vec![None] // pinned mythic always included, no variable choice
                    } else {
                        let mut opts: Vec<Option<&'static SetData>> = top_mythics
                            .iter()
                            .map(|(s, _)| Some(*s))
                            .collect();
                        opts.push(None);
                        opts
                    };

                    for normal_fill in &normal_fills {
                        for &var_monster in &monster_options {
                            for &var_mythic in &mythic_options {
                                let mut loadout_bonuses = Vec::new();
                                let mut loadout_names = Vec::new();

                                // Always include pinned sets
                                for &s in pinned_normal.iter() {
                                    loadout_bonuses.extend(
                                        s.bonuses_at(s.set_type.max_pieces())
                                            .into_iter()
                                            .cloned(),
                                    );
                                    loadout_names.push(s.name.clone());
                                }
                                for &s in pinned_monster.iter() {
                                    loadout_bonuses.extend(
                                        s.bonuses_at(s.set_type.max_pieces())
                                            .into_iter()
                                            .cloned(),
                                    );
                                    loadout_names.push(s.name.clone());
                                }
                                if let Some(s) = pinned_mythic {
                                    loadout_bonuses.extend(
                                        s.bonuses_at(s.set_type.max_pieces())
                                            .into_iter()
                                            .cloned(),
                                    );
                                    loadout_names.push(s.name.clone());
                                }

                                // Add variable sets
                                for &s in normal_fill {
                                    loadout_bonuses.extend(
                                        s.bonuses_at(s.set_type.max_pieces())
                                            .into_iter()
                                            .cloned(),
                                    );
                                    loadout_names.push(s.name.clone());
                                }
                                if let Some(s) = var_monster {
                                    loadout_bonuses.extend(
                                        s.bonuses_at(s.set_type.max_pieces())
                                            .into_iter()
                                            .cloned(),
                                    );
                                    loadout_names.push(s.name.clone());
                                }
                                if let Some(s) = var_mythic {
                                    loadout_bonuses.extend(
                                        s.bonuses_at(s.set_type.max_pieces())
                                            .into_iter()
                                            .cloned(),
                                    );
                                    loadout_names.push(s.name.clone());
                                }

                                let b = Build::new(
                                    skills.clone(),
                                    cp,
                                    passive,
                                    &loadout_bonuses,
                                    loadout_names.clone(),
                                    stats.clone(),
                                );
                                let damage = b.total_damage_per_cast;

                                evaluated_count.fetch_add(1, Ordering::Relaxed);
                                let _ = best_damage_bits
                                    .fetch_max(damage.to_bits(), Ordering::Relaxed);

                                if best.as_ref().map_or(true, |b| damage > b.damage) {
                                    best = Some(LoadoutCandidate {
                                        build_idx,
                                        set_bonuses: loadout_bonuses,
                                        set_names: loadout_names,
                                        damage,
                                    });
                                }
                            }
                        }
                    }

                    best
                })
                .collect()
        });

        let phase2_elapsed = phase2_start.elapsed();
        let total_evaluated = evaluated_count.load(Ordering::Relaxed);
        logger::info(&format!(
            "Evaluated {} set loadouts in {:.2?}",
            format::format_number(total_evaluated),
            phase2_elapsed,
        ));

        // Find global best across all builds
        results
            .into_iter()
            .flatten()
            .max_by(|a, b| a.damage.partial_cmp(&b.damage).unwrap())
            .map(|c| SetOptimizerResult {
                build_idx: c.build_idx,
                set_bonuses: c.set_bonuses,
                set_names: c.set_names,
                damage: c.damage,
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::bonuses::CHAMPION_POINTS;
    use crate::domain::{CharacterStats, ClassName, SkillLineName};
    use crate::infrastructure::logger;
    use crate::services::{BuildOptimizer, BuildOptimizerOptions};

    fn get_set(name: &str) -> &'static SetData {
        ALL_SETS
            .iter()
            .find(|s| s.name == name)
            .unwrap_or_else(|| panic!("Set '{}' not found in ALL_SETS", name))
    }

    fn get_champion_point(name: &str) -> BonusData {
        CHAMPION_POINTS
            .iter()
            .find(|cp| cp.name == name)
            .cloned()
            .unwrap_or_else(|| panic!("Champion point '{}' not found", name))
    }

    /// Helper: build a set of top builds for testing the set optimizer.
    /// Uses nightblade + bow/2h with all 4 CPs forced (fast: single CP combo).
    fn make_test_builds() -> Vec<Build> {
        logger::set_quiet(true);
        let optimizer = BuildOptimizer::new(BuildOptimizerOptions {
            character_stats: CharacterStats::default(),
            verbose: false,
            pure_class: Some(ClassName::Nightblade),
            required_class_names: vec![],
            required_weapon_skill_lines: vec![SkillLineName::Bow, SkillLineName::TwoHanded],
            required_champion_points: vec![
                get_champion_point("Deadly Aim"),
                get_champion_point("Master-at-Arms"),
                get_champion_point("Thaumaturge"),
                get_champion_point("Biting Aura"),
            ],
            required_skills: vec![],
            forced_morphs: vec![],
            parallelism: 4,
            max_pool_size: None,
            set_bonuses: vec![],
            set_names: vec![],
        });
        optimizer.find_optimal_build()
    }

    #[test]
    fn test_set_optimizer_returns_result() {
        let builds = make_test_builds();
        assert!(!builds.is_empty());

        let result = SetOptimizer::optimize(
            &builds[..1], // just the top build for speed
            &SetOptimizerOptions {
                top_k: 3,
                pinned_normal: vec![],
                pinned_monster: vec![],
                pinned_mythic: None,
                parallelism: 2,
                verbose: false,
            },
        );

        assert!(result.is_some(), "Set optimizer should find a loadout");
        let result = result.unwrap();
        assert!(result.damage > 0.0, "Damage should be positive");
        assert!(
            !result.set_names.is_empty(),
            "Should select at least one set"
        );
    }

    #[test]
    fn test_set_optimizer_with_pinned_normal() {
        let builds = make_test_builds();
        let mothers = get_set("Mother's Sorrow");

        let result = SetOptimizer::optimize(
            &builds[..1],
            &SetOptimizerOptions {
                top_k: 3,
                pinned_normal: vec![mothers],
                pinned_monster: vec![],
                pinned_mythic: None,
                parallelism: 2,
                verbose: false,
            },
        );

        let result = result.expect("Should find a loadout with pinned normal set");
        assert!(
            result.set_names.contains(&"Mother's Sorrow".to_string()),
            "Pinned set should appear in result: {:?}",
            result.set_names
        );
    }

    #[test]
    fn test_set_optimizer_with_pinned_mythic() {
        let builds = make_test_builds();
        let kilt = get_set("Harpooner's Wading Kilt");

        let result = SetOptimizer::optimize(
            &builds[..1],
            &SetOptimizerOptions {
                top_k: 3,
                pinned_normal: vec![],
                pinned_monster: vec![],
                pinned_mythic: Some(kilt),
                parallelism: 2,
                verbose: false,
            },
        );

        let result = result.expect("Should find a loadout with pinned mythic");
        assert!(
            result
                .set_names
                .contains(&"Harpooner's Wading Kilt".to_string()),
            "Pinned mythic should appear in result: {:?}",
            result.set_names
        );
    }

    #[test]
    fn test_set_optimizer_damage_exceeds_no_sets() {
        let builds = make_test_builds();
        let no_set_damage = builds[0].total_damage_per_cast;

        let result = SetOptimizer::optimize(
            &builds[..1],
            &SetOptimizerOptions {
                top_k: 5,
                pinned_normal: vec![],
                pinned_monster: vec![],
                pinned_mythic: None,
                parallelism: 2,
                verbose: false,
            },
        );

        let result = result.expect("Should find a loadout");
        assert!(
            result.damage > no_set_damage,
            "Optimized sets ({}) should beat no sets ({})",
            result.damage,
            no_set_damage
        );
    }

    #[test]
    fn test_set_optimizer_all_slots_pinned() {
        let builds = make_test_builds();
        let mothers = get_set("Mother's Sorrow");
        let julianos = get_set("Law of Julianos");
        let zaan = get_set("Zaan");
        let kilt = get_set("Harpooner's Wading Kilt");

        let result = SetOptimizer::optimize(
            &builds[..1],
            &SetOptimizerOptions {
                top_k: 3,
                pinned_normal: vec![mothers, julianos],
                pinned_monster: vec![zaan],
                pinned_mythic: Some(kilt),
                parallelism: 2,
                verbose: false,
            },
        );

        let result = result.expect("Should find a loadout with all pinned");
        // When all slots are pinned, the result should contain exactly those sets
        assert!(result.set_names.contains(&"Mother's Sorrow".to_string()));
        assert!(result.set_names.contains(&"Law of Julianos".to_string()));
        assert!(result.set_names.contains(&"Zaan".to_string()));
        assert!(result
            .set_names
            .contains(&"Harpooner's Wading Kilt".to_string()));
    }
}
