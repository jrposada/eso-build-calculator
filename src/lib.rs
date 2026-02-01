pub mod cli;
pub mod data;
pub mod domain;
pub mod infrastructure;
pub mod services;

// Re-export commonly used types
pub use data::{ClassName, DamageType, Resource, SkillLineName, TargetType};
pub use domain::{Build, SkillDamage, SkillData};
pub use services::{BuildOptimizer, BuildOptimizerOptions, SkillsService};
