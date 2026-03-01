use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use std::time::Instant;

use rayon::prelude::*;

use super::build_config::BuildConfig;
use super::parsers::{parse_champion_point, parse_class_name, parse_skill, parse_weapon_skill_line};
use super::simulation_display::display_simulation_result;
use crate::domain::{
    CharacterStats, ClassName, SimulationResult, SkillData, SkillLineName, WeaponType,
    ATTRIBUTE_POINTS_BONUS, BUILD_CONSTRAINTS,
};
use crate::infrastructure::{format, logger};
use crate::services::{
    generate_distributions, infer_weapons, BarDistribution, BuildOptimizer, BuildOptimizerOptions,
    FightSimulator,
};
use clap::Args;

/// Optimize build command arguments
#[derive(Args, Debug)]
pub struct OptimizeArgs {
    /// Require at least 1 skill line from these classes (comma-separated)
    #[arg(short = 'c', long, value_delimiter = ',', value_parser = parse_class_name, conflicts_with = "pure")]
    pub classes: Option<Vec<ClassName>>,

    /// Use only skills from a single class (pure build)
    #[arg(long, value_parser = parse_class_name, conflicts_with = "classes")]
    pub pure: Option<ClassName>,

    /// Require at least 1 skill line from these weapons (comma-separated)
    #[arg(short = 'w', long, value_delimiter = ',', value_parser = parse_weapon_skill_line)]
    pub weapons: Option<Vec<SkillLineName>>,

    /// Require these champion points (comma-separated)
    #[arg(long = "cp", value_delimiter = ',', value_parser = parse_champion_point)]
    pub champion_points: Option<Vec<crate::domain::BonusData>>,

    /// Require these skills in every build (comma-separated skill names)
    #[arg(short = 's', long, value_delimiter = ',', value_parser = parse_skill)]
    pub skills: Option<Vec<&'static SkillData>>,

    /// Force specific morph selections (comma-separated morph names)
    #[arg(short = 'm', long, value_delimiter = ',')]
    pub morphs: Option<Vec<String>>,

    /// Show optimization progress
    #[arg(short = 'v', long)]
    pub verbose: bool,

    /// Number of parallel threads to use (default: half of available CPUs)
    #[arg(short = 'p', long)]
    pub parallelism: Option<u8>,

    /// Cap non-spammable skill pool per skill-line combo (prune lowest-damage skills)
    #[arg(long)]
    pub max_pool_size: Option<usize>,

    /// Allocate 64 attribute points to magicka
    #[arg(long, conflicts_with = "stamina")]
    pub magicka: bool,

    /// Allocate 64 attribute points to stamina
    #[arg(long, conflicts_with = "magicka")]
    pub stamina: bool,

    /// Bar 1 weapon type for fight simulation (e.g., inferno-staff, bow)
    #[arg(long, value_parser = WeaponType::parse)]
    pub bar1_weapon: Option<WeaponType>,

    /// Bar 2 weapon type for fight simulation (e.g., inferno-staff, bow)
    #[arg(long, value_parser = WeaponType::parse)]
    pub bar2_weapon: Option<WeaponType>,
}

impl OptimizeArgs {
    pub fn run(&self) {
        // Validate class count
        if let Some(classes) = &self.classes {
            if classes.len() > BUILD_CONSTRAINTS.class_skill_line_count {
                logger::error(&format!(
                    "Maximum {} classes allowed",
                    BUILD_CONSTRAINTS.class_skill_line_count
                ));
                std::process::exit(1);
            }
        }

        // Validate weapon count
        if let Some(weapons) = &self.weapons {
            if weapons.len() > BUILD_CONSTRAINTS.weapon_skill_line_count {
                logger::error(&format!(
                    "Maximum {} weapons allowed",
                    BUILD_CONSTRAINTS.weapon_skill_line_count
                ));
                std::process::exit(1);
            }
        }

        // Validate required skills count
        if let Some(skills) = &self.skills {
            if skills.len() > BUILD_CONSTRAINTS.skill_count {
                logger::error(&format!(
                    "Maximum {} required skills allowed",
                    BUILD_CONSTRAINTS.skill_count
                ));
                std::process::exit(1);
            }
        }

        // Validate champion point count
        if let Some(cp) = &self.champion_points {
            if cp.len() > BUILD_CONSTRAINTS.champion_point_count {
                logger::error(&format!(
                    "Maximum {} champion points allowed",
                    BUILD_CONSTRAINTS.champion_point_count
                ));
                std::process::exit(1);
            }
        }

        let mut character_stats = CharacterStats::default();
        if self.magicka {
            character_stats.max_magicka += ATTRIBUTE_POINTS_BONUS;
        } else if self.stamina {
            character_stats.max_stamina += ATTRIBUTE_POINTS_BONUS;
        }

        logger::info("Finding optimal build...");

        let optimizer = BuildOptimizer::new(BuildOptimizerOptions {
            character_stats,
            verbose: self.verbose,
            pure_class: self.pure,
            required_class_names: self.classes.clone().unwrap_or_default(),
            required_weapon_skill_lines: self.weapons.clone().unwrap_or_default(),
            required_champion_points: self.champion_points.clone().unwrap_or_default(),
            required_skills: self.skills.clone().unwrap_or_default(),
            forced_morphs: self.morphs.clone().unwrap_or_default(),
            parallelism: self
                .parallelism
                .unwrap_or_else(|| (num_cpus::get() / 2).max(1) as u8),
            max_pool_size: self.max_pool_size,
        });

        let start = Instant::now();
        let builds = optimizer.find_optimal_build();
        let elapsed = start.elapsed();

        if builds.is_empty() {
            logger::error("No valid build found with the given constraints.");
            std::process::exit(1);
        }

        // Display top-1 build by damage-per-cast
        let best_build = &builds[0];
        logger::info(&best_build.to_string());
        logger::info(&format!("Optimization completed in {:.2?}", elapsed));

        // --- Fight simulation on top candidates ---
        let sim_result = self.run_simulation(&builds);

        // Use the build selected by simulation (if any), otherwise fall back to DPC-best
        let export_build = sim_result
            .as_ref()
            .map(|(build_idx, _, _)| &builds[*build_idx])
            .unwrap_or(best_build);
        let sim_data = sim_result.as_ref().map(|(_, dist, result)| (dist, result));
        Self::prompt_export(export_build, self.bar1_weapon, self.bar2_weapon, sim_data);
    }

    fn run_simulation(
        &self,
        builds: &[crate::domain::Build],
    ) -> Option<(usize, BarDistribution, SimulationResult)> {
        // Determine weapon types from CLI args or infer from the top build
        let (bar1_weapon, bar2_weapon) = match (self.bar1_weapon, self.bar2_weapon) {
            (Some(w1), Some(w2)) => (w1, w2),
            (Some(w1), None) => (w1, w1),
            (None, Some(w2)) => (w2, w2),
            (None, None) => {
                let top_skills = builds[0].skills();
                match infer_weapons(top_skills) {
                    Ok(weapons) => weapons,
                    Err(e) => {
                        logger::warn(&format!(
                            "Could not infer weapons for simulation: {}. Skipping fight simulation.",
                            e
                        ));
                        return None;
                    }
                }
            }
        };

        logger::info(&format!(
            "Running fight simulation on top {} candidates (Bar1: {}, Bar2: {})...",
            builds.len(),
            bar1_weapon,
            bar2_weapon
        ));

        let sim_start = Instant::now();

        // Pre-compute work items: (build_idx, simulator, distributions)
        let work: Vec<(usize, FightSimulator, Vec<BarDistribution>)> = builds
            .iter()
            .enumerate()
            .filter_map(|(build_idx, build)| {
                let distributions =
                    generate_distributions(build.skills(), bar1_weapon, bar2_weapon);
                if distributions.is_empty() {
                    return None;
                }
                let simulator =
                    FightSimulator::new(build.effective_stats(), build.resolved_bonuses());
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
                    if local_best.as_ref().map_or(true, |(_, r)| result.dps > r.dps) {
                        local_best = Some((dist_idx, result));
                    }

                    let done = completed.fetch_add(1, Ordering::Relaxed) + 1;

                    // Update shared best DPS via CAS loop
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
                        logger::progress(&format!(
                            "Simulating: {}/{} | Best DPS: {}",
                            done, total_sims, best_str
                        ));
                    }
                }
                local_best.map(|(dist_idx, result)| {
                    (*build_idx, dist_idx, distributions.clone(), result)
                })
            })
            .collect();

        let sim_elapsed = sim_start.elapsed();

        // Find global best from parallel results
        if let Some((best_build_idx, best_dist_idx, distributions, result)) = results
            .into_iter()
            .max_by(|(_, _, _, a), (_, _, _, b)| a.dps.partial_cmp(&b.dps).unwrap())
        {
            if best_build_idx > 0 {
                logger::info(&format!(
                    "Simulation selected build #{} (of {} candidates) as best DPS.",
                    best_build_idx + 1,
                    builds.len()
                ));
                logger::info(&builds[best_build_idx].to_string());
            }
            let best_dist = &distributions[best_dist_idx];
            display_simulation_result(&result, best_dist, distributions.len());
            logger::info(&format!("Simulation completed in {:.2?}", sim_elapsed));
            return Some((best_build_idx, best_dist.clone(), result));
        }

        None
    }

    fn prompt_export(
        build: &crate::domain::Build,
        bar1_weapon: Option<WeaponType>,
        bar2_weapon: Option<WeaponType>,
        simulation: Option<(&BarDistribution, &SimulationResult)>,
    ) {
        // Show prompt with greyed-out default value "no"
        print!("\nExport build to file? [path/no]: \x1b[90mn\x1b[0m");
        // Move cursor back over the default value so user input overwrites it
        print!("\x1b[1D");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            return;
        }

        let input = input.trim();
        if input.is_empty() || input.eq_ignore_ascii_case("no") || input.eq_ignore_ascii_case("n") {
            return;
        }

        let metadata = simulation.map(|(dist, result)| {
            super::build_config::BuildMetadata {
                dps: result.dps,
                total_damage: result.total_damage,
                fight_duration: result.fight_duration,
                bar1_skills: dist.bar1.skills.iter().map(|s| s.name.to_string()).collect(),
                bar2_skills: dist.bar2.skills.iter().map(|s| s.name.to_string()).collect(),
            }
        });

        let path = PathBuf::from(input);
        let config = BuildConfig {
            skills: build.skill_names(),
            champion_points: build.champion_point_names(),
            bar1_weapon: bar1_weapon.map(|w| w.to_string()),
            bar2_weapon: bar2_weapon.map(|w| w.to_string()),
            metadata,
        };

        match serde_json::to_string_pretty(&config) {
            Ok(json) => match fs::write(&path, json) {
                Ok(_) => logger::info(&format!("Build exported to {}", path.display())),
                Err(e) => logger::error(&format!("Failed to write file: {}", e)),
            },
            Err(e) => logger::error(&format!("Failed to serialize build: {}", e)),
        }
    }
}
