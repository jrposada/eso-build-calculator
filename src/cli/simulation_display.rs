use crate::domain::SimulationResult;
use crate::infrastructure::{format, logger, table};
use crate::services::bar_distribution::BarDistribution;

pub fn display_simulation_result(
    result: &SimulationResult,
    dist: &BarDistribution,
    total_distributions: usize,
    set_names: &[(String, u8)],
) {
    let divider = "-".repeat(73);

    let bar1_names: Vec<_> = dist.bar1.skills.iter().map(|s| s.name.as_str()).collect();
    let bar2_names: Vec<_> = dist.bar2.skills.iter().map(|s| s.name.as_str()).collect();

    let mut lines = vec![
        String::new(),
        "Fight Simulation Results".to_string(),
        divider.clone(),
        format!("Target:           21M HP Trial Dummy"),
        format!(
            "Fight Duration:   {}:{:05.2}",
            (result.fight_duration as u64) / 60,
            result.fight_duration % 60.0
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

    if !set_names.is_empty() {
        let formatted: Vec<String> = set_names
            .iter()
            .map(|(name, pieces)| format!("{} ({}pc)", name, pieces))
            .collect();
        lines.push(format!("Sets:          {}", formatted.join(", ")));
    }

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
