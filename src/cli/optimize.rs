use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;
use std::time::Instant;

use super::build_config::BuildConfig;
use super::parsers::{parse_champion_point, parse_class_name, parse_skill, parse_weapon_skill_line};
use super::simulation_display::display_simulation_result;
use crate::domain::{CharacterStats, SkillData, WeaponType, ATTRIBUTE_POINTS_BONUS, BUILD_CONSTRAINTS};
use crate::domain::{ClassName, SkillLineName};
use crate::infrastructure::logger;
use crate::services::{
    generate_distributions, infer_weapons, BuildOptimizer, BuildOptimizerOptions, FightSimulator,
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
        self.run_simulation(&builds);

        Self::prompt_export(best_build, self.bar1_weapon, self.bar2_weapon);
    }

    fn run_simulation(&self, builds: &[crate::domain::Build]) {
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
                        return;
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

        let mut best_dps = f64::NEG_INFINITY;
        let mut best_build_idx = 0;
        let mut best_dist_idx = 0;
        let mut best_result = None;
        let mut best_distributions = None;

        for (build_idx, build) in builds.iter().enumerate() {
            let skills = build.skills();
            let distributions = generate_distributions(skills, bar1_weapon, bar2_weapon);

            if distributions.is_empty() {
                continue;
            }

            let effective_stats = build.effective_stats();
            let resolved_bonuses = build.resolved_bonuses();
            let simulator = FightSimulator::new(effective_stats, resolved_bonuses);

            for (dist_idx, dist) in distributions.iter().enumerate() {
                let result = simulator.simulate(dist);
                if result.dps > best_dps {
                    best_dps = result.dps;
                    best_build_idx = build_idx;
                    best_dist_idx = dist_idx;
                    best_result = Some(result);
                    best_distributions = Some(distributions.clone());
                }
            }
        }

        let sim_elapsed = sim_start.elapsed();

        if let (Some(result), Some(distributions)) = (best_result, best_distributions) {
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
        }
    }

    fn prompt_export(
        build: &crate::domain::Build,
        bar1_weapon: Option<WeaponType>,
        bar2_weapon: Option<WeaponType>,
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

        let path = PathBuf::from(input);
        let config = BuildConfig {
            skills: build.skill_names(),
            champion_points: build.champion_point_names(),
            bar1_weapon: bar1_weapon.map(|w| w.to_string()),
            bar2_weapon: bar2_weapon.map(|w| w.to_string()),
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
