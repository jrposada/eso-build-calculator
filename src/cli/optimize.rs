use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;
use std::time::Instant;

use super::build_config::BuildConfig;
use super::parsers::{parse_champion_point, parse_class_name, parse_weapon_skill_line};
use crate::domain::BUILD_CONSTRAINTS;
use crate::domain::{ClassName, SkillLineName};
use crate::infrastructure::logger;
use crate::services::{BuildOptimizer, BuildOptimizerOptions};
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

    /// Force specific morph selections (comma-separated morph names)
    #[arg(short = 'm', long, value_delimiter = ',')]
    pub morphs: Option<Vec<String>>,

    /// Show optimization progress
    #[arg(short = 'v', long)]
    pub verbose: bool,

    /// Number of parallel threads to use (default: half of available CPUs)
    #[arg(short = 'p', long)]
    pub parallelism: Option<u8>,
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

        logger::info("Finding optimal build...");

        let optimizer = BuildOptimizer::new(BuildOptimizerOptions {
            verbose: self.verbose,
            pure_class: self.pure,
            required_class_names: self.classes.clone().unwrap_or_default(),
            required_weapon_skill_lines: self.weapons.clone().unwrap_or_default(),
            required_champion_points: self.champion_points.clone().unwrap_or_default(),
            forced_morphs: self.morphs.clone().unwrap_or_default(),
            parallelism: self
                .parallelism
                .unwrap_or_else(|| (num_cpus::get() / 2).max(1) as u8),
        });

        let start = Instant::now();
        let build = optimizer.find_optimal_build();
        let elapsed = start.elapsed();

        match build {
            Some(b) => {
                logger::info(&b.to_string());
                logger::info(&format!("Optimization completed in {:.2?}", elapsed));
                Self::prompt_export(&b);
            }
            None => {
                logger::error("No valid build found with the given constraints.");
                std::process::exit(1);
            }
        }
    }

    fn prompt_export(build: &crate::domain::Build) {
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
