use clap::{Parser, Subcommand};
use eso_build_calculator::cli::{CalculateArgs, OptimizeArgs, ViewArgs};

#[derive(Parser)]
#[command(name = "eso-build-calculator")]
#[command(author = "JR Posada")]
#[command(version = "1.0.0")]
#[command(about = "ESO Build Calculator - Find optimal skill builds", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Calculate total damage for a specific build configuration
    Calculate(CalculateArgs),
    /// Find the optimal build to maximize total damage per cast
    Optimize(OptimizeArgs),
    /// View skill data
    View(ViewArgs),
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Calculate(args) => args.run(),
        Commands::Optimize(args) => args.run(),
        Commands::View(args) => args.run(),
    }
}
