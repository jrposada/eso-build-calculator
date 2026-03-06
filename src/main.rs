use clap::{Parser, Subcommand};
use eso_build_calculator::cli::{BreakpointsArgs, OptimizeArgs, SimulateArgs};

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
    /// Run a discrete-event fight simulation for a specific build configuration
    Simulate(SimulateArgs),
    /// Find the optimal build to maximize total damage per cast
    Optimize(OptimizeArgs),
    /// Show stat breakpoints: how much of one stat before another becomes the better investment
    Breakpoints(BreakpointsArgs),
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Simulate(args) => args.run(),
        Commands::Optimize(args) => args.run(),
        Commands::Breakpoints(args) => args.run(),
    }
}
