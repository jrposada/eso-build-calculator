use crate::data::bonuses::{TRIAL_BUFF_NAMES, TRIAL_DUMMY_BUFFS};
use crate::data::skill_trees::armor::armor_passives;
use crate::data::skill_trees::guild::undaunted::undaunted_passives::undaunted_mettle_bonuses;
use crate::domain::{
    BonusData, Build, BuildConfig, CharacterStats, Potion, SetData, SimulationResult, SkillData,
    SkillLineName, WeaponEnchant, BUILD_CONSTRAINTS,
};
use crate::infrastructure::format;
use crate::services::{
    generate_distributions, infer_weapons, BarDistribution, FightSimulator, PassivesService,
    PassivesServiceOptions,
};
use std::collections::HashSet;
use std::fmt;

use super::optimize_pipeline::resolve_set_bonuses;

pub struct SimulatePipelineOptions {
    pub config: BuildConfig,
    pub trial: bool,
    pub verbose: bool,
    pub avg_resource_pct: f64,
}

pub struct SimulatePipelineResult {
    pub build_summary: String,
    pub simulation: SimulationResult,
    pub best_distribution: BarDistribution,
    pub distributions_tested: usize,
    pub set_names: Vec<(String, u8)>,
    pub buffed_stats: Option<CharacterStats>,
    pub warnings: Vec<String>,
}

impl fmt::Display for SimulatePipelineResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let divider = "-".repeat(73);

        let bar1_names: Vec<_> = self
            .best_distribution
            .bar1
            .skills
            .iter()
            .map(|s| s.name.as_str())
            .collect();
        let bar2_names: Vec<_> = self
            .best_distribution
            .bar2
            .skills
            .iter()
            .map(|s| s.name.as_str())
            .collect();

        writeln!(f)?;
        writeln!(f, "Fight Simulation Results")?;
        writeln!(f, "{}", divider)?;
        writeln!(f, "Target:           21M HP Trial Dummy")?;
        writeln!(
            f,
            "Fight Duration:   {}:{:05.2}",
            (self.simulation.fight_duration as u64) / 60,
            self.simulation.fight_duration % 60.0
        )?;
        writeln!(
            f,
            "Total Damage:     {}",
            format::format_number(self.simulation.total_damage as u64)
        )?;
        writeln!(
            f,
            "DPS:              {}",
            format::format_number(self.simulation.dps as u64)
        )?;
        writeln!(f)?;
        writeln!(
            f,
            "Bar 1 ({}): {}",
            self.best_distribution.bar1.weapon_type,
            bar1_names.join(", ")
        )?;
        writeln!(
            f,
            "Bar 2 ({}): {}",
            self.best_distribution.bar2.weapon_type,
            bar2_names.join(", ")
        )?;

        if !self.set_names.is_empty() {
            let formatted: Vec<String> = self
                .set_names
                .iter()
                .map(|(name, pieces)| std::format!("{} ({}pc)", name, pieces))
                .collect();
            writeln!(f, "Sets:          {}", formatted.join(", "))?;
        }

        write!(f, "{}", self.simulation)?;
        writeln!(
            f,
            "\nTested {} bar distribution(s). Best shown above.",
            self.distributions_tested
        )
    }
}

pub struct SimulatePipeline;

impl SimulatePipeline {
    pub fn run(options: SimulatePipelineOptions) -> Result<SimulatePipelineResult, String> {
        let config = &options.config;
        let mut warnings: Vec<String> = Vec::new();

        // Parse skills
        let skills: Vec<&'static SkillData> = config
            .skills
            .iter()
            .map(|name| SkillData::parse(name).map_err(|e| e.to_string()))
            .collect::<Result<Vec<_>, _>>()?;

        // Parse champion points
        let champion_points: Vec<BonusData> = config
            .champion_points
            .iter()
            .map(|name| BonusData::parse_champion_point(name).map_err(|e| e.to_string()))
            .collect::<Result<Vec<_>, _>>()?;

        // Parse sets
        let sets: Vec<&'static SetData> = config
            .sets
            .iter()
            .map(|name| SetData::parse(name).map_err(|e| e.to_string()))
            .collect::<Result<Vec<_>, _>>()?;

        // Validate counts
        if skills.len() != BUILD_CONSTRAINTS.skill_count {
            return Err(format!(
                "Exactly {} skills required, got {}",
                BUILD_CONSTRAINTS.skill_count,
                skills.len()
            ));
        }
        if champion_points.len() != BUILD_CONSTRAINTS.champion_point_count {
            return Err(format!(
                "Exactly {} champion points required, got {}",
                BUILD_CONSTRAINTS.champion_point_count,
                champion_points.len()
            ));
        }

        // Warnings
        if !skills.iter().any(|s| s.spammable) {
            warnings.push(
                "This build has no spammable skill. Every rotation needs at least one instant-cast filler.".to_string(),
            );
        }
        let pure_spammable_count = skills
            .iter()
            .filter(|s| s.spammable && s.bonuses.is_none())
            .count();
        if pure_spammable_count > 1 {
            warnings.push(format!(
                "This build has {} pure spammable skills (spammable without bonuses). Only the highest-damage one would be used as filler; the rest waste a slot.",
                pure_spammable_count
            ));
        }

        // Resolve passives
        let skill_lines: HashSet<SkillLineName> = skills.iter().map(|s| s.skill_line).collect();
        let passives_service = PassivesService::new(PassivesServiceOptions::default());
        let mut passive_bonuses: Vec<BonusData> = skill_lines
            .iter()
            .flat_map(|sl| passives_service.get_passives_by_skill_line(*sl))
            .flat_map(|p| p.bonuses.iter().cloned())
            .collect();

        // Armor passives
        let armor_dist = config.armor;
        if let Some(dw) = armor_dist.dominant_weight() {
            passive_bonuses.extend(armor_passives(dw));
        }
        passive_bonuses.extend(undaunted_mettle_bonuses(armor_dist.type_count()));

        // Potion bonuses
        let potion = config.potion.unwrap_or(Potion::WeaponPower);
        passive_bonuses.extend(potion.bonuses());

        // Resolve set bonuses
        let (set_bonuses, set_names, set_proc_effects) = resolve_set_bonuses(&sets);

        // Trial buffs
        let extra_bonuses: Vec<BonusData> = if options.trial {
            TRIAL_DUMMY_BUFFS.clone()
        } else {
            Vec::new()
        };

        // Compute character stats
        let character_stats = config.character_stats.clone();

        // Build
        let build = Build::new_with_extra(
            skills.clone(),
            &champion_points,
            &passive_bonuses,
            &set_bonuses,
            set_names.clone(),
            character_stats,
            &extra_bonuses,
        );

        let build_summary = build.to_string();

        // Resolve weapons: Specific → infer from skills → skill-line default
        let pinned_bar1 = config.bar1_weapon.and_then(|wc| wc.weapon_type());
        let pinned_bar2 = config.bar2_weapon.and_then(|wc| wc.weapon_type());
        let inferred = infer_weapons(&skills).ok();

        let resolve = |pinned: Option<crate::domain::WeaponType>,
                       wc: Option<crate::domain::WeaponChoice>,
                       inferred_wt: Option<crate::domain::WeaponType>|
         -> Option<crate::domain::WeaponType> {
            pinned
                .or(inferred_wt)
                .or_else(|| wc.and_then(|wc| wc.skill_line().default_weapon_type()))
        };

        let (bar1_weapon, bar2_weapon) = match (pinned_bar1, pinned_bar2) {
            (Some(w1), Some(w2)) => (w1, w2),
            (Some(w1), None) => {
                let w2 =
                    resolve(None, config.bar2_weapon, inferred.map(|(_, w2)| w2)).unwrap_or(w1);
                (w1, w2)
            }
            (None, Some(w2)) => {
                let w1 =
                    resolve(None, config.bar1_weapon, inferred.map(|(w1, _)| w1)).unwrap_or(w2);
                (w1, w2)
            }
            (None, None) => {
                let w1 = resolve(None, config.bar1_weapon, inferred.map(|(w1, _)| w1));
                let w2 = resolve(None, config.bar2_weapon, inferred.map(|(_, w2)| w2));
                match (w1, w2) {
                    (Some(w1), Some(w2)) => (w1, w2),
                    _ => {
                        return Err(
                            "Could not resolve weapons. Specify --weapon or use a build with weapon skill lines."
                                .to_string(),
                        )
                    }
                }
            }
        };

        // Generate distributions and simulate
        let distributions = generate_distributions(&skills, bar1_weapon, bar2_weapon);
        if distributions.is_empty() {
            return Err(
                "No valid bar distributions found for this skill/weapon combination.".to_string(),
            );
        }

        let effective_stats = build.effective_stats();
        let resolved_bonuses = build.resolved_bonuses();

        let mut suppressed = if options.trial {
            TRIAL_BUFF_NAMES.clone()
        } else {
            HashSet::new()
        };
        for bonus in potion.bonuses() {
            suppressed.insert(bonus.name.clone());
        }

        let bar1_enchant = config.bar1_enchant.or(Some(WeaponEnchant::Flame));
        let bar2_enchant = config.bar2_enchant.or(Some(WeaponEnchant::Flame));

        let simulator = FightSimulator::new(effective_stats, resolved_bonuses, suppressed)
            .with_enchants(bar1_enchant, bar2_enchant)
            .with_set_procs(set_proc_effects)
            .with_avg_resource_pct(options.avg_resource_pct);

        // Compute buffed stats if verbose
        let buffed_stats = if options.verbose {
            Some(simulator.compute_buffed_stats(&distributions[0]))
        } else {
            None
        };

        // Run all distributions
        let mut results: Vec<(usize, SimulationResult)> = distributions
            .iter()
            .enumerate()
            .map(|(i, dist)| (i, simulator.simulate(dist)))
            .collect();

        results.sort_by(|a, b| b.1.dps.partial_cmp(&a.1.dps).unwrap());

        let (best_idx, best_result) = results.into_iter().next().ok_or("No simulation results")?;

        Ok(SimulatePipelineResult {
            build_summary,
            simulation: best_result,
            best_distribution: distributions[best_idx].clone(),
            distributions_tested: distributions.len(),
            set_names,
            buffed_stats,
            warnings,
        })
    }
}
