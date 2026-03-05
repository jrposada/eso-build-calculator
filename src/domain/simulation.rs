use std::fmt;

use super::BonusTarget;
use super::DamageCoefficients;
use super::DamageFlags;
use crate::infrastructure::{format, table};

/// Global cooldown in seconds (1 GCD per action)
pub const GCD: f64 = 1.0;

/// Bar swap animation delay in seconds
pub const BAR_SWAP_DELAY: f64 = 0.3;

/// Trial dummy HP (21 million)
pub const TRIAL_DUMMY_HP: f64 = 21_000_000.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActiveBar {
    Bar1,
    Bar2,
}

impl ActiveBar {
    pub fn opposite(&self) -> Self {
        match self {
            ActiveBar::Bar1 => ActiveBar::Bar2,
            ActiveBar::Bar2 => ActiveBar::Bar1,
        }
    }
}

/// Tracks a running DoT effect on the target.
#[derive(Debug, Clone)]
pub struct ActiveEffect {
    pub source_skill_name: String,
    pub remaining_duration: f64,
    pub next_tick_in: f64,
    pub tick_interval: f64,
    pub tick_count: i32,
    pub total_ticks: i32,
    pub base_value: f64,
    pub flags: DamageFlags,
    pub coefficients: DamageCoefficients,
    pub increase_per_tick: f64,
    pub flat_increase_per_tick: f64,
    pub ignores_modifier: bool,
    /// Damage-done modifier sum snapshotted at cast time (DoT ticks use this, not current buffs)
    pub snapshotted_done_modifier: f64,
    /// EnemyDamageTaken modifier sum snapshotted at cast time (separate multiplicative layer)
    pub snapshotted_taken_modifier: f64,
    /// Armor damage factor snapshotted at cast time
    pub snapshotted_armor_factor: f64,
    /// Critical multiplier snapshotted at cast time
    pub snapshotted_crit_mult: f64,
}

/// Tracks an active buff granted by a skill.
#[derive(Debug, Clone)]
pub struct ActiveBuff {
    /// Dedup key (e.g., "Major Brutality")
    pub name: String,
    /// Which skill granted this buff (for rotation priority checks)
    pub source_skill_name: String,
    /// None = permanent (AbilitySlotted)
    pub remaining_duration: Option<f64>,
    pub target: BonusTarget,
    pub value: f64,
}

/// Buff uptime during a fight simulation.
#[derive(Debug, Clone)]
pub struct BuffUptime {
    pub name: String,
    /// Fraction of fight duration the buff was active (0.0–1.0)
    pub uptime: f64,
    /// Whether this buff is provided externally (e.g. trial dummy)
    pub external: bool,
}

/// Results of a fight simulation.
#[derive(Debug, Clone)]
pub struct SimulationResult {
    pub total_damage: f64,
    pub fight_duration: f64,
    pub dps: f64,
    pub skill_breakdown: Vec<SkillBreakdown>,
    pub la_damage: f64,
    pub la_count: u32,
    pub bar_swap_count: u32,
    pub buff_uptimes: Vec<BuffUptime>,
}

#[derive(Debug, Clone)]
pub struct SkillBreakdown {
    pub skill_name: String,
    pub damage: f64,
    pub cast_count: u32,
}

impl fmt::Display for SimulationResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Damage breakdown table
        let mut breakdown_data: Vec<Vec<String>> = Vec::new();

        for entry in &self.skill_breakdown {
            let pct = if self.total_damage > 0.0 {
                entry.damage / self.total_damage * 100.0
            } else {
                0.0
            };
            let dps = if self.fight_duration > 0.0 {
                entry.damage / self.fight_duration
            } else {
                0.0
            };
            breakdown_data.push(vec![
                String::new(), // rank placeholder
                entry.skill_name.clone(),
                format::format_number(entry.damage as u64),
                entry.cast_count.to_string(),
                format::format_number(dps as u64),
                std::format!("{:.1}%", pct),
            ]);
        }

        // Add light attack row
        if self.la_count > 0 {
            let la_pct = if self.total_damage > 0.0 {
                self.la_damage / self.total_damage * 100.0
            } else {
                0.0
            };
            let la_dps = if self.fight_duration > 0.0 {
                self.la_damage / self.fight_duration
            } else {
                0.0
            };
            breakdown_data.push(vec![
                String::new(),
                "Light Attack".to_string(),
                format::format_number(self.la_damage as u64),
                self.la_count.to_string(),
                format::format_number(la_dps as u64),
                std::format!("{:.1}%", la_pct),
            ]);
        }

        // Sort by damage descending
        breakdown_data.sort_by(|a, b| {
            let a_dmg: f64 = a[2].replace(',', "").parse().unwrap_or(0.0);
            let b_dmg: f64 = b[2].replace(',', "").parse().unwrap_or(0.0);
            b_dmg.partial_cmp(&a_dmg).unwrap()
        });

        // Assign ranks
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

        write!(f, "{}", breakdown_table)?;

        // Buff uptimes table
        let has_external = self
            .buff_uptimes
            .iter()
            .any(|b| b.external && b.uptime > 0.0);
        let uptime_data: Vec<Vec<String>> = self
            .buff_uptimes
            .iter()
            .filter(|b| b.uptime > 0.0)
            .map(|b| {
                let name = if b.external {
                    std::format!("{}*", b.name)
                } else {
                    b.name.clone()
                };
                vec![name, std::format!("{:.1}%", b.uptime * 100.0)]
            })
            .collect();

        if !uptime_data.is_empty() {
            let uptime_table = table::table(
                &uptime_data,
                table::TableOptions {
                    title: Some("Buff Uptimes".to_string()),
                    columns: vec![
                        table::ColumnDefinition::new("Name", 36),
                        table::ColumnDefinition::new("Uptime", 8).align_right(),
                    ],
                    footer: if has_external {
                        Some("* Provided by trial dummy".to_string())
                    } else {
                        None
                    },
                },
            );
            write!(f, "\n{}", uptime_table)?;
        }

        Ok(())
    }
}
