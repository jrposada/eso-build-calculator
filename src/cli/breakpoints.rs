use crate::domain::CharacterStats;
use crate::domain::formulas::crit_rating_to_chance;
use crate::infrastructure::{format as fmt, logger, table};
use crate::services::breakpoints_pipeline::{BreakpointGrid, BreakpointStat, BreakpointsPipeline};
use clap::Args;

/// Show stat breakpoints: what to maximize first given the DPS formula
#[derive(Args, Debug)]
pub struct BreakpointsArgs {
    /// Skill name (optional — validates skill exists)
    #[arg()]
    pub skill_name: Option<String>,

    /// Sets both max magicka and stamina
    #[arg(long)]
    pub max_stat: Option<f64>,

    /// Sets both weapon and spell damage
    #[arg(long)]
    pub max_power: Option<f64>,

    #[arg(long)]
    pub crit_rating: Option<f64>,

    /// As multiplier, e.g. 1.75
    #[arg(long)]
    pub crit_damage: Option<f64>,

    #[arg(long)]
    pub penetration: Option<f64>,

    #[arg(long)]
    pub target_armor: Option<f64>,
}

impl BreakpointsArgs {
    pub fn run(&self) {
        if let Some(name) = &self.skill_name {
            if let Err(e) = crate::domain::SkillData::parse(name) {
                logger::error(&e);
                std::process::exit(1);
            }
        }

        let mut stats = CharacterStats::default();
        if let Some(v) = self.max_stat {
            stats.max_magicka = v;
            stats.max_stamina = v;
        }
        if let Some(v) = self.max_power {
            stats.weapon_damage = v;
            stats.spell_damage = v;
        }
        if let Some(v) = self.crit_rating {
            stats.critical_rating = v;
        }
        if let Some(v) = self.crit_damage {
            stats.critical_damage = v;
        }
        if let Some(v) = self.penetration {
            stats.penetration = v;
        }
        if let Some(v) = self.target_armor {
            stats.target_armor = v;
        }

        let grid = BreakpointsPipeline::run(&stats);
        self.display(&stats, &grid);
    }

    fn display(&self, stats: &CharacterStats, grid: &BreakpointGrid) {
        // Section 1: Current stat effectiveness ranked
        let ranked = BreakpointsPipeline::current_edc_ranking(stats);
        let edc_data: Vec<Vec<String>> = ranked
            .iter()
            .enumerate()
            .map(|(i, (stat, edc))| {
                let unit_desc = match stat {
                    BreakpointStat::CritDamage => "per 1%".to_string(),
                    BreakpointStat::WeaponSpellDamage => "per 1 pt".to_string(),
                    _ => "per 1000".to_string(),
                };
                let edc_per_unit = match stat {
                    BreakpointStat::CritDamage => edc * 100.0,
                    BreakpointStat::WeaponSpellDamage => *edc,
                    _ => edc * 1000.0,
                };
                vec![
                    format!("{}", i + 1),
                    stat.label().to_string(),
                    format!("{:.4}%", edc_per_unit),
                    unit_desc,
                ]
            })
            .collect();

        logger::info(&table::table(
            &edc_data,
            table::TableOptions {
                title: Some("Stat Effectiveness (current build)".into()),
                columns: vec![
                    table::ColumnDefinition::new("Rank", 4).align_right(),
                    table::ColumnDefinition::new("Stat", 10),
                    table::ColumnDefinition::new("EDC/unit", 10).align_right(),
                    table::ColumnDefinition::new("Unit", 10),
                ],
                footer: None,
            },
        ));

        // Section 2: Investment sequence
        let sequence = BreakpointsPipeline::investment_sequence(stats);
        let mut seq_lines = vec![String::new(), "Investment Sequence".to_string(), "-".repeat(50)];

        // Simulate stats to track crit values after each step
        let mut sim_stats = stats.clone();
        for (i, step) in sequence.iter().enumerate() {
            let amount_str = match step.stat {
                BreakpointStat::CritDamage => format!("+{:.0}%", step.amount * 100.0),
                _ => format!("+{}", fmt::format_number(step.amount as u64)),
            };

            sim_stats = step.stat.apply_delta(&sim_stats, step.amount);

            let detail = match step.stat {
                BreakpointStat::CritRating => {
                    let new_chance = crit_rating_to_chance(sim_stats.critical_rating);
                    format!(" / to {:.0}% crit", new_chance * 100.0)
                }
                BreakpointStat::CritDamage => {
                    let bonus_pct = (sim_stats.critical_damage - 1.0) * 100.0;
                    format!(" / to {:.0}% bonus", bonus_pct)
                }
                _ => String::new(),
            };

            // Determine what becomes better next
            let next_label = if i + 1 < sequence.len() {
                format!(
                    "  (then {} becomes better)",
                    sequence[i + 1].stat.label()
                )
            } else {
                String::new()
            };

            seq_lines.push(format!(
                "  {}. Add {} {}{}{}",
                i + 1,
                amount_str,
                step.stat.label(),
                detail,
                next_label,
            ));
        }

        if sequence.is_empty() {
            seq_lines.push("  All stats at cap or no investment needed.".to_string());
        }

        logger::info(&seq_lines.join("\n"));

        // Section 3: Crossover grid
        let n = BreakpointStat::ALL.len();
        let headers: Vec<String> = BreakpointStat::ALL.iter().map(|s| s.label().to_string()).collect();

        let mut grid_data: Vec<Vec<String>> = Vec::new();
        for row in 0..n {
            let mut row_cells = vec![format!("{}...", BreakpointStat::ALL[row].label())];
            for col in 0..n {
                row_cells.push(grid.format_cell(row, col));
            }
            grid_data.push(row_cells);
        }

        let mut columns = vec![table::ColumnDefinition::new("Investing in", 12)];
        for h in &headers {
            columns.push(table::ColumnDefinition::new(h.as_str(), 10).align_right());
        }

        logger::info(&table::table(
            &grid_data,
            table::TableOptions {
                title: Some("Crossover Grid (+N more of row before col becomes better)".into()),
                columns,
                footer: None,
            },
        ));
    }
}
