use super::simulation_display::{display_simulation_result, format_buffed_stats};
use crate::domain::BuildConfig;
use crate::infrastructure::logger;
use crate::services::{SimulatePipeline, SimulatePipelineOptions, DEFAULT_AVG_RESOURCE_PCT};
use clap::Args;
use std::fs;
use std::path::PathBuf;

/// Run a discrete-event fight simulation for a specific build configuration
#[derive(Args, Debug)]
pub struct SimulateArgs {
    /// Path to build configuration file (exported from optimize)
    #[arg(short = 'f', long)]
    pub file: PathBuf,

    /// Show extra details (buffed character stats)
    #[arg(short = 'v', long)]
    pub verbose: bool,

    /// Disable trial dummy buffs/debuffs (enabled by default)
    #[arg(long = "no-trial")]
    pub no_trial: bool,

    /// Average resource percentage for resource-scaling sets like Bahsei's (0-100, default from file)
    #[arg(long)]
    pub avg_resource_pct: Option<f64>,
}

impl SimulateArgs {
    pub fn run(&self) {
        let config = self.load_config();
        let avg_resource_pct = self.avg_resource_pct.unwrap_or(DEFAULT_AVG_RESOURCE_PCT);

        let options = SimulatePipelineOptions {
            config,
            trial: !self.no_trial,
            verbose: self.verbose,
            avg_resource_pct,
        };

        logger::info("Calculating build damage...");

        match SimulatePipeline::run(options) {
            Ok(result) => self.display(result),
            Err(e) => {
                logger::error(&e);
                std::process::exit(1);
            }
        }
    }

    fn load_config(&self) -> BuildConfig {
        let content = fs::read_to_string(&self.file).unwrap_or_else(|e| {
            logger::error(&format!(
                "Failed to read file '{}': {}",
                self.file.display(),
                e
            ));
            std::process::exit(1);
        });

        serde_json::from_str(&content).unwrap_or_else(|e| {
            logger::error(&format!("Failed to parse build config: {}", e));
            std::process::exit(1);
        })
    }

    fn display(&self, result: crate::services::SimulatePipelineResult) {
        for warning in &result.warnings {
            logger::warn(warning);
        }

        logger::info(&result.build_summary);

        logger::info(&format!(
            "Running fight simulation (Bar1: {}, Bar2: {})...",
            result.best_distribution.bar1.weapon_type,
            result.best_distribution.bar2.weapon_type
        ));

        if let Some(ref stats) = result.buffed_stats {
            logger::trace(&format_buffed_stats(stats));
        }

        display_simulation_result(
            &result.simulation,
            &result.best_distribution,
            result.distributions_tested,
            &result.set_names,
        );
    }
}
