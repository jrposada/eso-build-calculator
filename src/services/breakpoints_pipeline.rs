use crate::domain::{BonusTarget, CharacterStats};
use crate::domain::formulas::effective_damage_contribution;

/// Stats we compare in the breakpoint grid.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BreakpointStat {
    WeaponSpellDamage,
    Resource,
    CritRating,
    CritDamage,
    Penetration,
}

impl BreakpointStat {
    pub const ALL: [BreakpointStat; 5] = [
        BreakpointStat::WeaponSpellDamage,
        BreakpointStat::Resource,
        BreakpointStat::CritRating,
        BreakpointStat::CritDamage,
        BreakpointStat::Penetration,
    ];

    pub fn label(&self) -> &'static str {
        match self {
            BreakpointStat::WeaponSpellDamage => "WD/SD",
            BreakpointStat::Resource => "Resource",
            BreakpointStat::CritRating => "CritRat",
            BreakpointStat::CritDamage => "CritDmg",
            BreakpointStat::Penetration => "Pen",
        }
    }

    /// The BonusTarget used for edc evaluation.
    fn bonus_target(&self, stats: &CharacterStats) -> BonusTarget {
        match self {
            BreakpointStat::WeaponSpellDamage => BonusTarget::WeaponAndSpellDamageFlat,
            BreakpointStat::Resource => {
                if stats.max_magicka >= stats.max_stamina {
                    BonusTarget::MaxMagickaFlat
                } else {
                    BonusTarget::MaxStaminaFlat
                }
            }
            BreakpointStat::CritRating => BonusTarget::CriticalRating,
            BreakpointStat::CritDamage => BonusTarget::CriticalDamage,
            BreakpointStat::Penetration => BonusTarget::PhysicalAndSpellPenetration,
        }
    }

    /// Unit amount for one "point" of this stat in edc.
    fn unit(&self) -> f64 {
        match self {
            BreakpointStat::CritDamage => 0.01,
            _ => 1.0,
        }
    }

    /// Maximum additional amount of this stat that can be added.
    fn max_delta(&self, stats: &CharacterStats) -> f64 {
        match self {
            BreakpointStat::WeaponSpellDamage => 5000.0,
            BreakpointStat::Resource => 50000.0,
            BreakpointStat::CritRating => (21_912.0 - stats.critical_rating).max(0.0),
            BreakpointStat::CritDamage => (2.25 - stats.critical_damage).max(0.0),
            BreakpointStat::Penetration => (stats.target_armor - stats.penetration).max(0.0),
        }
    }

    /// Apply a delta of this stat to a copy of CharacterStats.
    pub(crate) fn apply_delta(&self, stats: &CharacterStats, delta: f64) -> CharacterStats {
        let mut s = stats.clone();
        match self {
            BreakpointStat::WeaponSpellDamage => {
                s.weapon_damage += delta;
                s.spell_damage += delta;
            }
            BreakpointStat::Resource => {
                if stats.max_magicka >= stats.max_stamina {
                    s.max_magicka += delta;
                } else {
                    s.max_stamina += delta;
                }
            }
            BreakpointStat::CritRating => {
                s.critical_rating += delta;
            }
            BreakpointStat::CritDamage => {
                s.critical_damage += delta;
            }
            BreakpointStat::Penetration => {
                s.penetration += delta;
            }
        }
        s
    }
}

/// Grid[row][col] = how much more of row stat before col stat becomes the better investment.
/// None = no crossover (row stat always dominates).
pub struct BreakpointGrid {
    pub cells: Vec<Vec<Option<f64>>>,
}

impl BreakpointGrid {
    pub fn format_cell(&self, row: usize, col: usize) -> String {
        if row == col {
            return "-".to_string();
        }
        match self.cells[row][col] {
            None => "N/A".to_string(),
            Some(v) if v <= 0.0 => "0".to_string(),
            Some(v) => {
                let stat = BreakpointStat::ALL[row];
                match stat {
                    BreakpointStat::CritDamage => format!("+{:.0}%", v * 100.0),
                    _ => format!("+{}", crate::infrastructure::format::format_number(v as u64)),
                }
            }
        }
    }
}

/// Binary search for the crossover point: how much of stat_a to add before stat_b becomes better.
///
/// Returns:
/// - Some(0) if stat_b already has higher edc
/// - Some(delta) if crossover found at delta
/// - None if stat_a always dominates within max_delta
fn find_crossover(
    stats: &CharacterStats,
    stat_a: BreakpointStat,
    stat_b: BreakpointStat,
    max_delta: f64,
) -> Option<f64> {
    let epsilon = match stat_a {
        BreakpointStat::CritDamage => 0.001,
        _ => 1.0,
    };

    if max_delta <= epsilon {
        return None;
    }

    let target_a = stat_a.bonus_target(stats);
    let target_b = stat_b.bonus_target(stats);
    let unit_a = stat_a.unit();
    let unit_b = stat_b.unit();

    // Check current state
    let edc_a = effective_damage_contribution(target_a, unit_a, stats);
    let edc_b = effective_damage_contribution(target_b, unit_b, stats);

    if edc_a <= edc_b {
        return Some(0.0);
    }

    // Check if crossover happens at all at max_delta
    let modified_max = stat_a.apply_delta(stats, max_delta);
    let target_a_max = stat_a.bonus_target(&modified_max);
    let target_b_max = stat_b.bonus_target(&modified_max);
    let edc_a_max = effective_damage_contribution(target_a_max, unit_a, &modified_max);
    let edc_b_max = effective_damage_contribution(target_b_max, unit_b, &modified_max);

    if edc_a_max > edc_b_max {
        return None;
    }

    // Binary search
    let mut lo = 0.0_f64;
    let mut hi = max_delta;

    while (hi - lo) > epsilon {
        let mid = (lo + hi) / 2.0;
        let modified = stat_a.apply_delta(stats, mid);
        let target_a_mid = stat_a.bonus_target(&modified);
        let target_b_mid = stat_b.bonus_target(&modified);
        let edc_a_mid = effective_damage_contribution(target_a_mid, unit_a, &modified);
        let edc_b_mid = effective_damage_contribution(target_b_mid, unit_b, &modified);

        if edc_a_mid > edc_b_mid {
            lo = mid;
        } else {
            hi = mid;
        }
    }

    Some(hi)
}

pub struct InvestmentStep {
    pub stat: BreakpointStat,
    pub amount: f64,
}

pub struct BreakpointsPipeline;

impl BreakpointsPipeline {
    /// Rank all stats by their current effective damage contribution, descending.
    pub fn current_edc_ranking(stats: &CharacterStats) -> Vec<(BreakpointStat, f64)> {
        let mut ranked: Vec<(BreakpointStat, f64)> = BreakpointStat::ALL
            .iter()
            .map(|&stat| {
                let edc = effective_damage_contribution(stat.bonus_target(stats), stat.unit(), stats);
                (stat, edc)
            })
            .collect();
        ranked.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        ranked
    }

    /// Return the sequence of investment steps that maximises DPS in order.
    pub fn investment_sequence(stats: &CharacterStats) -> Vec<InvestmentStep> {
        let mut current = stats.clone();
        let mut sequence = Vec::new();
        const MAX_ITERATIONS: usize = 15;

        for _ in 0..MAX_ITERATIONS {
            let ranked = Self::current_edc_ranking(&current);
            let best = ranked[0].0;

            let max_delta = best.max_delta(&current);
            if max_delta <= 0.0 {
                break;
            }

            // Find the minimum crossover among all other stats
            let step_epsilon = match best {
                BreakpointStat::CritDamage => 0.001,
                _ => 1.0,
            };
            let min_step = ranked[1..]
                .iter()
                .filter_map(|(other, _)| find_crossover(&current, best, *other, max_delta))
                .filter(|&v| v > step_epsilon)
                .reduce(f64::min);

            match min_step {
                None => {
                    // best always dominates up to cap
                    sequence.push(InvestmentStep { stat: best, amount: max_delta });
                    break;
                }
                Some(step) => {
                    sequence.push(InvestmentStep { stat: best, amount: step });
                    current = best.apply_delta(&current, step);
                }
            }
        }

        sequence
    }

    pub fn run(stats: &CharacterStats) -> BreakpointGrid {
        let stats_list = BreakpointStat::ALL;
        let n = stats_list.len();
        let mut cells = vec![vec![None; n]; n];

        for (row, &stat_a) in stats_list.iter().enumerate() {
            for (col, &stat_b) in stats_list.iter().enumerate() {
                if row == col {
                    continue;
                }
                let max_delta = stat_a.max_delta(stats);
                cells[row][col] = find_crossover(stats, stat_a, stat_b, max_delta);
            }
        }

        BreakpointGrid { cells }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_stats() -> CharacterStats {
        CharacterStats::default()
            .with_max_magicka(40_000.0)
            .with_max_stamina(12_000.0)
            .with_weapon_damage(6000.0)
            .with_spell_damage(6000.0)
            .with_critical_rating(10956.0)
            .with_critical_damage(1.75)
            .with_penetration(10000.0)
            .with_target_armor(18200.0)
    }

    #[test]
    fn test_find_crossover_b_already_better() {
        let stats = test_stats();
        // Same stat: edc_a == edc_b, so edc_a <= edc_b -> Some(0)
        let result = find_crossover(
            &stats,
            BreakpointStat::CritRating,
            BreakpointStat::CritRating,
            1000.0,
        );
        assert_eq!(result, Some(0.0));
    }

    #[test]
    fn test_find_crossover_returns_none_at_cap() {
        let stats = test_stats().with_penetration(18200.0);
        let result = find_crossover(
            &stats,
            BreakpointStat::Penetration,
            BreakpointStat::WeaponSpellDamage,
            BreakpointStat::Penetration.max_delta(&stats),
        );
        assert!(result.is_none());
    }

    #[test]
    fn test_apply_delta_wd_sd() {
        let stats = test_stats();
        let modified = BreakpointStat::WeaponSpellDamage.apply_delta(&stats, 100.0);
        assert_eq!(modified.weapon_damage, 6100.0);
        assert_eq!(modified.spell_damage, 6100.0);
    }

    #[test]
    fn test_apply_delta_resource_magicka_build() {
        let stats = test_stats();
        let modified = BreakpointStat::Resource.apply_delta(&stats, 1000.0);
        assert_eq!(modified.max_magicka, 41_000.0);
        assert_eq!(modified.max_stamina, 12_000.0);
    }

    #[test]
    fn test_grid_format_diagonal() {
        let grid = BreakpointGrid {
            cells: vec![vec![None; 5]; 5],
        };
        assert_eq!(grid.format_cell(0, 0), "-");
        assert_eq!(grid.format_cell(2, 2), "-");
    }

    #[test]
    fn test_grid_format_na() {
        let grid = BreakpointGrid {
            cells: vec![vec![None; 5]; 5],
        };
        assert_eq!(grid.format_cell(0, 1), "N/A");
    }

    #[test]
    fn test_grid_format_crit_damage_row_uses_percent() {
        let mut cells = vec![vec![None; 5]; 5];
        cells[3][0] = Some(0.15);
        let grid = BreakpointGrid { cells };
        assert_eq!(grid.format_cell(3, 0), "+15%");
    }

    #[test]
    fn test_grid_format_integer_row() {
        let mut cells = vec![vec![None; 5]; 5];
        cells[0][2] = Some(450.0);
        let grid = BreakpointGrid { cells };
        assert_eq!(grid.format_cell(0, 2), "+450");
    }

    #[test]
    fn test_run_produces_5x5_grid() {
        let stats = test_stats();
        let grid = BreakpointsPipeline::run(&stats);
        assert_eq!(grid.cells.len(), 5);
        for row in &grid.cells {
            assert_eq!(row.len(), 5);
        }
    }
}
