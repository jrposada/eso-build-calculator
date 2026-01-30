use crate::data::bonuses::CHAMPION_POINTS;
use crate::data::skills::ALL_SKILLS;
use crate::domain::Skill;
use crate::infrastructure::{logger, table};
use clap::Args;
use std::collections::HashMap;

/// Rank skills command arguments
#[derive(Args, Debug)]
pub struct RankArgs {
    /// Number of skills to show
    #[arg(short = 'l', long, default_value = "20")]
    pub limit: usize,

    /// Exclude ultimate abilities
    #[arg(long)]
    pub exclude_ultimates: bool,

    /// Only include skills from specified sources (comma-separated)
    #[arg(short = 's', long, value_delimiter = ',')]
    pub source: Option<Vec<String>>,

    /// Only include skills of specified mechanic (comma-separated: instant, dot, channeled)
    #[arg(short = 'm', long, value_delimiter = ',')]
    pub mechanic: Option<Vec<String>>,

    /// Apply list of champion bonuses (comma-separated)
    #[arg(long)]
    pub champion_bonus: Option<Vec<String>>,
}

impl RankArgs {
    pub fn run(&self) {
        logger::warn("Rank command is experimental and may produce inaccurate results.");

        if self.limit == 0 {
            logger::error("Error: Limit must be a positive number.");
            std::process::exit(1);
        }

        // Convert skill data to Skill objects
        let mut skills: Vec<Skill> = ALL_SKILLS
            .iter()
            .map(|data| Skill::new((*data).clone()))
            .collect();

        // Filter by ultimate
        if self.exclude_ultimates {
            skills.retain(|skill| !skill.is_ultimate());
        }

        // Filter by source
        if let Some(sources) = &self.source {
            let allowed: Vec<_> = sources.iter().map(|s| s.to_lowercase()).collect();
            skills.retain(|skill| allowed.contains(&skill.class_name().to_string().to_lowercase()));
        }

        // Filter by mechanic
        if let Some(mechanics) = &self.mechanic {
            let allowed: Vec<_> = mechanics.iter().map(|m| m.to_lowercase()).collect();
            skills.retain(|skill| allowed.contains(&skill.mechanic().to_string().to_lowercase()));
        }

        // Get champion bonuses
        let champion_bonuses: Vec<_> = if let Some(bonus_names) = &self.champion_bonus {
            let allowed: Vec<_> = bonus_names.iter().map(|b| b.to_lowercase()).collect();
            CHAMPION_POINTS
                .iter()
                .filter(|b| allowed.contains(&b.name.to_lowercase()))
                .map(|b| b.to_bonus_data())
                .collect()
        } else {
            vec![]
        };

        // Calculate damage for each skill
        let mut damage_map: HashMap<String, f64> = HashMap::new();
        for skill in &skills {
            let damage = skill.calculate_damage_per_cast(&champion_bonuses);
            damage_map.insert(skill.name().to_string(), damage);
        }

        // Filter out skills with no damage
        skills.retain(|skill| damage_map.get(skill.name()).copied().unwrap_or(0.0) > 0.0);

        // Group by base skill name and pick highest damage
        let mut skills_by_base: HashMap<String, Skill> = HashMap::new();
        for skill in skills {
            let key = format!("{}-{}", skill.class_name(), skill.base_skill_name());
            let skill_damage = damage_map.get(skill.name()).copied().unwrap_or(0.0);

            if let Some(existing) = skills_by_base.get(&key) {
                let existing_damage = damage_map.get(existing.name()).copied().unwrap_or(0.0);
                if skill_damage > existing_damage {
                    skills_by_base.insert(key, skill);
                }
            } else {
                skills_by_base.insert(key, skill);
            }
        }

        // Sort by damage
        let mut ranked_skills: Vec<_> = skills_by_base.into_values().collect();
        ranked_skills.sort_by(|a, b| {
            let a_dmg = damage_map.get(a.name()).copied().unwrap_or(0.0);
            let b_dmg = damage_map.get(b.name()).copied().unwrap_or(0.0);
            b_dmg
                .partial_cmp(&a_dmg)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        if ranked_skills.is_empty() {
            logger::warn("No damaging skills found.");
            return;
        }

        // Format and display table
        let display_skills: Vec<_> = ranked_skills.iter().take(self.limit).collect();
        let total_skills = ranked_skills.len();

        let data: Vec<Vec<String>> = display_skills
            .iter()
            .enumerate()
            .map(|(i, skill)| {
                let damage = damage_map.get(skill.name()).copied().unwrap_or(0.0);
                let duration = skill.duration();
                let duration_str = if duration > 0.0 {
                    format!("{}s", duration)
                } else {
                    "instant".to_string()
                };

                vec![
                    (i + 1).to_string(),
                    skill.name().to_string(),
                    skill.class_name().to_string(),
                    skill.skill_line().to_string(),
                    format!("{:.0}", damage),
                    duration_str,
                ]
            })
            .collect();

        let table_output = table::table(
            &data,
            table::TableOptions {
                title: Some("Skills Ranked by Damage Per Cast".to_string()),
                columns: vec![
                    table::ColumnDefinition::new("#", 4).align_right(),
                    table::ColumnDefinition::new("Name", 25),
                    table::ColumnDefinition::new("Source", 12),
                    table::ColumnDefinition::new("Skill Line", 18),
                    table::ColumnDefinition::new("Damage", 10).align_right(),
                    table::ColumnDefinition::new("Duration", 10).align_right(),
                ],
                footer: Some(format!(
                    "Showing {} of {} skills",
                    display_skills.len(),
                    total_skills
                )),
            },
        );

        println!("{}", table_output);
    }
}
