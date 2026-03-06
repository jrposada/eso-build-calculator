use crate::domain::BuildConfig;
use crate::infrastructure::logger;
use crate::services::breakpoints_pipeline::{BreakpointStat, BreakpointsPipeline};
use clap::Args;
use std::fs;
use std::path::PathBuf;

/// Show stat breakpoints: what to maximize first given the DPS formula
#[derive(Args, Debug)]
pub struct BreakpointsArgs {
    /// Path to build configuration file (exported from optimize)
    #[arg(short = 'f', long)]
    pub file: PathBuf,
}

impl BreakpointsArgs {
    pub fn run(&self) {
        let config = self.load_config();
        let stats = &config.character_stats;

        logger::info("Computing stat breakpoints...");

        let grid = BreakpointsPipeline::run(stats);
        self.display(&grid);
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

    fn display(&self, grid: &crate::services::breakpoints_pipeline::BreakpointGrid) {
        let stats = BreakpointStat::ALL;
        let n = stats.len();

        let label_width = 10;
        let col_width = 10;
        let total_width = label_width + 2 + n * (col_width + 1);
        let divider = "-".repeat(total_width);

        println!();
        println!("Stat Breakpoints (crossover points)");
        println!("{}", divider);

        // Header row
        print!("{:<width$}  ", "", width = label_width);
        for stat in &stats {
            print!("{:>width$} ", stat.label(), width = col_width);
        }
        println!();

        // Data rows
        for (row, stat_a) in stats.iter().enumerate() {
            print!("{:<width$}  ", stat_a.label(), width = label_width);
            for col in 0..n {
                let cell = grid.format_cell(row, col);
                print!("{:>width$} ", cell, width = col_width);
            }
            println!();
        }

        println!("{}", divider);
        println!("Row = stat you're adding. Cell = how much more of row stat");
        println!("before column stat becomes the better investment.");
        println!("N/A = no crossover (row stat always dominates).");
        println!("  - = same stat comparison.");
    }
}
