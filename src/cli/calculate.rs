use super::build_config::BuildConfig;
use super::parsers::{parse_champion_point, parse_skill};
use crate::domain::{
    BonusData, Build, CharacterStats, SkillData, SkillLineName, WeaponType, ATTRIBUTE_POINTS_BONUS,
    BUILD_CONSTRAINTS,
};
use crate::infrastructure::{format, logger, table};
use crate::services::{
    generate_distributions, infer_weapons, FightSimulator, PassivesService, PassivesServiceOptions,
};
use clap::Args;
use std::collections::HashSet;
use std::fs;
use std::path::PathBuf;

/// Calculate total damage for a specific build configuration
#[derive(Args, Debug)]
pub struct CalculateArgs {
    /// 10 skills (comma-separated skill names)
    #[arg(short = 's', long, value_delimiter = ',', value_parser = parse_skill, conflicts_with = "file")]
    pub skills: Option<Vec<&'static SkillData>>,

    /// 4 champion points (comma-separated)
    #[arg(long = "cp", value_delimiter = ',', value_parser = parse_champion_point, conflicts_with = "file")]
    pub champion_points: Option<Vec<BonusData>>,

    /// Path to build configuration file (exported from optimize)
    #[arg(short = 'f', long, conflicts_with_all = ["skills", "champion_points"])]
    pub file: Option<PathBuf>,

    /// Allocate 64 attribute points to magicka
    #[arg(long, conflicts_with = "stamina")]
    pub magicka: bool,

    /// Allocate 64 attribute points to stamina
    #[arg(long, conflicts_with = "magicka")]
    pub stamina: bool,

    /// Bar 1 weapon type (e.g., inferno-staff, bow)
    #[arg(long, value_parser = WeaponType::parse)]
    pub bar1_weapon: Option<WeaponType>,

    /// Bar 2 weapon type (e.g., inferno-staff, bow)
    #[arg(long, value_parser = WeaponType::parse)]
    pub bar2_weapon: Option<WeaponType>,
}

impl CalculateArgs {
    pub fn run(&self) {
        let (skills, champion_points, file_weapons) = if let Some(path) = &self.file {
            self.load_from_file(path)
        } else {
            let (s, cp) = self.get_from_args();
            (s, cp, (None, None))
        };

        // Validate skill count
        if skills.len() != BUILD_CONSTRAINTS.skill_count {
            logger::error(&format!(
                "Exactly {} skills required, got {}",
                BUILD_CONSTRAINTS.skill_count,
                skills.len()
            ));
            std::process::exit(1);
        }

        // Validate champion point count
        if champion_points.len() != BUILD_CONSTRAINTS.champion_point_count {
            logger::error(&format!(
                "Exactly {} champion points required, got {}",
                BUILD_CONSTRAINTS.champion_point_count,
                champion_points.len()
            ));
            std::process::exit(1);
        }

        logger::info("Calculating build damage...");

        // Collect unique skill lines from the selected skills
        let skill_lines: HashSet<SkillLineName> = skills.iter().map(|s| s.skill_line).collect();

        // Get passives from the skill lines
        let passives_service = PassivesService::new(PassivesServiceOptions::default());
        let passive_bonuses: Vec<BonusData> = skill_lines
            .iter()
            .flat_map(|sl| passives_service.get_passives_by_skill_line(*sl))
            .flat_map(|p| p.bonuses.iter().cloned())
            .collect();

        let mut stats = CharacterStats::default();
        if self.magicka {
            stats.max_magicka += ATTRIBUTE_POINTS_BONUS;
        } else if self.stamina {
            stats.max_stamina += ATTRIBUTE_POINTS_BONUS;
        }

        // Create the build (for stats resolution and display)
        let build = Build::new(skills.clone(), &champion_points, &passive_bonuses, stats);

        if !skills.iter().any(|s| s.spammable) {
            logger::warn("This build has no spammable skill. Every rotation needs at least one instant-cast filler.");
        }

        let pure_spammable_count = skills
            .iter()
            .filter(|s| s.spammable && s.bonuses.is_none())
            .count();
        if pure_spammable_count > 1 {
            logger::warn(&format!(
                "This build has {} pure spammable skills (spammable without bonuses). Only the highest-damage one would be used as filler; the rest waste a slot.",
                pure_spammable_count
            ));
        }

        // Display the build summary (existing behavior)
        logger::info(&build.to_string());

        // --- Fight simulation ---
        // Determine weapon types
        let bar1_weapon = self
            .bar1_weapon
            .or(file_weapons.0);
        let bar2_weapon = self
            .bar2_weapon
            .or(file_weapons.1);

        let (bar1_weapon, bar2_weapon) = match (bar1_weapon, bar2_weapon) {
            (Some(w1), Some(w2)) => (w1, w2),
            (Some(w1), None) => (w1, w1),
            (None, Some(w2)) => (w2, w2),
            (None, None) => match infer_weapons(&skills) {
                Ok(weapons) => weapons,
                Err(e) => {
                    logger::warn(&format!(
                        "Could not infer weapons for simulation: {}. Skipping fight simulation.",
                        e
                    ));
                    return;
                }
            },
        };

        logger::info(&format!(
            "Running fight simulation (Bar1: {}, Bar2: {})...",
            bar1_weapon, bar2_weapon
        ));

        let distributions = generate_distributions(&skills, bar1_weapon, bar2_weapon);

        // Use the build's effective stats and resolved bonuses for the simulator
        let effective_stats = build.effective_stats();
        let resolved_bonuses = build.resolved_bonuses();

        let simulator = FightSimulator::new(effective_stats, resolved_bonuses);

        let mut results: Vec<(usize, crate::domain::SimulationResult)> = distributions
            .iter()
            .enumerate()
            .map(|(i, dist)| (i, simulator.simulate(dist)))
            .collect();

        results.sort_by(|a, b| b.1.dps.partial_cmp(&a.1.dps).unwrap());

        if let Some((best_idx, best_result)) = results.first() {
            let best_dist = &distributions[*best_idx];
            self.display_simulation_result(best_result, best_dist, distributions.len());
        }
    }

    fn display_simulation_result(
        &self,
        result: &crate::domain::SimulationResult,
        dist: &crate::services::bar_distribution::BarDistribution,
        total_distributions: usize,
    ) {
        let divider = "-".repeat(73);

        let bar1_names: Vec<_> = dist.bar1.skills.iter().map(|s| s.name.as_str()).collect();
        let bar2_names: Vec<_> = dist.bar2.skills.iter().map(|s| s.name.as_str()).collect();

        let mut lines = vec![
            String::new(),
            "Fight Simulation Results".to_string(),
            divider.clone(),
            format!(
                "Target:           21M HP Trial Dummy"
            ),
            format!(
                "Fight Duration:   {:.1}s",
                result.fight_duration
            ),
            format!(
                "Total Damage:     {}",
                format::format_number(result.total_damage as u64)
            ),
            format!(
                "DPS:              {}",
                format::format_number(result.dps as u64)
            ),
            String::new(),
            format!(
                "Bar 1 ({}): {}",
                dist.bar1.weapon_type,
                bar1_names.join(", ")
            ),
            format!(
                "Bar 2 ({}): {}",
                dist.bar2.weapon_type,
                bar2_names.join(", ")
            ),
        ];

        // Damage breakdown table
        let mut breakdown_data: Vec<Vec<String>> = Vec::new();
        let mut rank = 1;

        // Add skill breakdowns
        for entry in &result.skill_breakdown {
            let pct = if result.total_damage > 0.0 {
                entry.damage / result.total_damage * 100.0
            } else {
                0.0
            };
            let dps = if result.fight_duration > 0.0 {
                entry.damage / result.fight_duration
            } else {
                0.0
            };
            breakdown_data.push(vec![
                rank.to_string(),
                entry.skill_name.clone(),
                format::format_number(entry.damage as u64),
                entry.cast_count.to_string(),
                format::format_number(dps as u64),
                format!("{:.1}%", pct),
            ]);
            rank += 1;
        }

        // Add light attack row
        if result.la_count > 0 {
            let la_pct = if result.total_damage > 0.0 {
                result.la_damage / result.total_damage * 100.0
            } else {
                0.0
            };
            let la_dps = if result.fight_duration > 0.0 {
                result.la_damage / result.fight_duration
            } else {
                0.0
            };
            breakdown_data.push(vec![
                rank.to_string(),
                "Light Attack".to_string(),
                format::format_number(result.la_damage as u64),
                result.la_count.to_string(),
                format::format_number(la_dps as u64),
                format!("{:.1}%", la_pct),
            ]);
        }

        // Sort by damage descending
        breakdown_data.sort_by(|a, b| {
            let a_dmg: f64 = a[2].replace(',', "").parse().unwrap_or(0.0);
            let b_dmg: f64 = b[2].replace(',', "").parse().unwrap_or(0.0);
            b_dmg.partial_cmp(&a_dmg).unwrap()
        });

        // Re-number ranks
        for (i, row) in breakdown_data.iter_mut().enumerate() {
            row[0] = (i + 1).to_string();
        }

        let breakdown_table = table::table(
            &breakdown_data,
            table::TableOptions {
                title: Some("Damage Breakdown".to_string()),
                columns: vec![
                    table::ColumnDefinition::new("#", 4).align_right(),
                    table::ColumnDefinition::new("Source", 28),
                    table::ColumnDefinition::new("Damage", 12).align_right(),
                    table::ColumnDefinition::new("Casts", 6).align_right(),
                    table::ColumnDefinition::new("DPS", 10).align_right(),
                    table::ColumnDefinition::new("%", 7).align_right(),
                ],
                footer: None,
            },
        );

        lines.push(breakdown_table);
        lines.push(format!(
            "Tested {} bar distribution(s). Best shown above.",
            total_distributions
        ));

        logger::info(&lines.join("\n"));
    }

    fn get_from_args(&self) -> (Vec<&'static SkillData>, Vec<BonusData>) {
        let skills = self.skills.clone().unwrap_or_else(|| {
            logger::error("Either --skills/-s or --file/-f must be provided");
            std::process::exit(1);
        });

        let champion_points = self.champion_points.clone().unwrap_or_else(|| {
            logger::error("Either --champion-points/-p or --file/-f must be provided");
            std::process::exit(1);
        });

        (skills, champion_points)
    }

    fn load_from_file(
        &self,
        path: &PathBuf,
    ) -> (
        Vec<&'static SkillData>,
        Vec<BonusData>,
        (Option<WeaponType>, Option<WeaponType>),
    ) {
        let content = fs::read_to_string(path).unwrap_or_else(|e| {
            logger::error(&format!("Failed to read file '{}': {}", path.display(), e));
            std::process::exit(1);
        });

        let config: BuildConfig = serde_json::from_str(&content).unwrap_or_else(|e| {
            logger::error(&format!("Failed to parse build config: {}", e));
            std::process::exit(1);
        });

        let skills: Vec<&'static SkillData> = config
            .skills
            .iter()
            .map(|name| {
                parse_skill(name).unwrap_or_else(|e| {
                    logger::error(&e);
                    std::process::exit(1);
                })
            })
            .collect();

        let champion_points: Vec<BonusData> = config
            .champion_points
            .iter()
            .map(|name| {
                parse_champion_point(name).unwrap_or_else(|e| {
                    logger::error(&e);
                    std::process::exit(1);
                })
            })
            .collect();

        let bar1 = config
            .bar1_weapon
            .as_ref()
            .and_then(|w| WeaponType::parse(w).ok());
        let bar2 = config
            .bar2_weapon
            .as_ref()
            .and_then(|w| WeaponType::parse(w).ok());

        (skills, champion_points, (bar1, bar2))
    }
}
