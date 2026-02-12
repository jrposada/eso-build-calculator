use crate::domain::formulas::crit_rating_to_chance;
use clap::{Args, Subcommand};

/// Convert game values between different representations
#[derive(Args, Debug)]
pub struct ConvertArgs {
    #[command(subcommand)]
    command: ConvertCommands,
}

#[derive(Subcommand, Debug)]
pub enum ConvertCommands {
    /// Convert critical rating to critical chance percentage
    CriticalRating(CriticalRatingArgs),
}

#[derive(Args, Debug)]
pub struct CriticalRatingArgs {
    /// Critical rating value to convert
    pub value: f64,
}

impl ConvertArgs {
    pub fn run(&self) {
        match &self.command {
            ConvertCommands::CriticalRating(args) => args.run(),
        }
    }
}

impl CriticalRatingArgs {
    pub fn run(&self) {
        let chance = crit_rating_to_chance(self.value);
        println!("Critical Rating: {}", self.value);
        println!("Critical Chance: {:.2}%", chance * 100.0);
    }
}
