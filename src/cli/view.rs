use crate::data::skills::ALL_SKILLS;
use crate::domain::Skill;
use crate::infrastructure::logger;
use clap::Args;

/// View skill command arguments
#[derive(Args, Debug)]
pub struct ViewArgs {
    /// Skill name to view
    pub name: String,
}

impl ViewArgs {
    pub fn run(&self) {
        let skill_data = ALL_SKILLS
            .iter()
            .find(|s| s.name.to_lowercase() == self.name.trim().to_lowercase());

        match skill_data {
            Some(data) => {
                let skill = Skill::new((*data).clone());
                println!("{}", skill.format_details());
                println!();
            }
            None => {
                logger::warn("Skill not found.");
            }
        }
    }
}
