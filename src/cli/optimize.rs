use super::build_config::BuildConfig;
use super::parsers::{
    parse_champion_point, parse_class_name, parse_set, parse_skill, parse_weapon,
};
use super::simulation_display::display_simulation_result;
use crate::data::bonuses::{TRIAL_BUFF_NAMES, TRIAL_DUMMY_BUFFS};
use crate::data::passives::armor::armor_passives;
use crate::data::passives::undaunted_mettle_bonuses;
use crate::data::sets::ALL_SETS;
use crate::domain::{
    ArmorTrait, ArmorWeight, AttributeChoice, BonusData, Build, CharacterStats, ClassName, Food,
    GearConfig, JewelryTrait, MundusStone, Potion, Race, SetData, SetProcEffect, SetType,
    SimulationResult, SkillData, WeaponEnchant, WeaponTrait, WeaponType, BUILD_CONSTRAINTS,
};
use crate::infrastructure::{format, logger};
use crate::services::{
    generate_distributions, infer_weapons, stats_differ_significantly, BarDistribution,
    BuildOptimizer, BuildOptimizerOptions, FightSimulator, GearOptimizer, GearOptimizerOptions,
    SetOptimizer, SetOptimizerOptions,
};
use clap::Args;
use rayon::prelude::*;
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use std::time::Instant;

/// Optimize build command arguments
#[derive(Args, Debug)]
pub struct OptimizeArgs {
    /// Show optimization progress
    #[arg(short = 'v', long)]
    pub verbose: bool,

    /// Pin character race (dark-elf, khajiit, orc, etc.) - optimized if omitted
    #[arg(long, value_parser = Race::parse)]
    pub race: Option<Race>,

    /// Pin at least 1 skill line from these class (comma-separated) -
    #[arg(short = 'c', long, value_delimiter = ',', value_parser = parse_class_name)]
    pub class: Option<Vec<ClassName>>,

    /// Restrict build to only the classes specified by --class
    #[arg(long, requires = "class")]
    pub pure: bool,

    /// Pin weapon (comma-separated). Accepts skill lines (bow, destruction-staff, dual-wield,
    /// two-handed) or specific types (inferno-staff, lightning-staff, dual-wield-dagger, etc.).
    /// First value = bar1, second = bar2. One value = bar1 only (bar2 optimized).
    #[arg(short = 'w', long, value_delimiter = ',', value_parser = parse_weapon)]
    pub weapon: Option<Vec<WeaponType>>,

    /// Require these champion points (comma-separated)
    #[arg(long = "cp", value_delimiter = ',', value_parser = parse_champion_point)]
    pub champion_point: Option<Vec<crate::domain::BonusData>>,

    /// Require these skills in every build (comma-separated skill names)
    #[arg(short = 's', long, value_delimiter = ',', value_parser = parse_skill)]
    pub skill: Option<Vec<&'static SkillData>>,

    /// Pin attribute points to magicka (optimized if omitted)
    #[arg(long, conflicts_with = "stamina")]
    pub magicka: bool,

    /// Pin attribute points to stamina (optimized if omitted)
    #[arg(long, conflicts_with = "magicka")]
    pub stamina: bool,

    /// Pin gear sets (comma-separated). Auto-grouped by type: max 2 normal/arena, 2 monster, 1 mythic.
    #[arg(long, value_delimiter = ',', value_parser = parse_set)]
    pub set: Option<Vec<&'static SetData>>,

    /// Pin mundus stone (thief, shadow, warrior, etc.) - optimized if omitted
    #[arg(long, value_parser = MundusStone::parse)]
    pub mundus: Option<MundusStone>,

    /// Pin food buff (lava-foot, ghastly-eye, sugar-skulls) - optimized if omitted
    #[arg(long, value_parser = Food::parse)]
    pub food: Option<Food>,

    /// Pin armor trait for all 7 pieces - optimized if omitted
    #[arg(long, value_parser = ArmorTrait::parse)]
    pub armor_trait: Option<ArmorTrait>,

    /// Pin jewelry trait for all 3 pieces - optimized if omitted
    #[arg(long, value_parser = JewelryTrait::parse)]
    pub jewelry_trait: Option<JewelryTrait>,

    /// Pin weapon trait - optimized if omitted
    #[arg(long, value_parser = WeaponTrait::parse)]
    pub weapon_trait: Option<WeaponTrait>,

    /// Armor weight for armor passives (medium, light, heavy; defaults to medium)
    #[arg(long, value_parser = ArmorWeight::parse)]
    pub armor_weight: Option<ArmorWeight>,

    /// Potion buff (weapon-power, spell-power; defaults to weapon-power)
    #[arg(long, value_parser = Potion::parse)]
    pub potion: Option<Potion>,

    /// Bar 1 weapon enchant (flame, poison, shock, berserker; defaults to flame)
    #[arg(long, value_parser = WeaponEnchant::parse)]
    pub bar1_enchant: Option<WeaponEnchant>,

    /// Bar 2 weapon enchant (flame, poison, shock, berserker; defaults to flame)
    #[arg(long, value_parser = WeaponEnchant::parse)]
    pub bar2_enchant: Option<WeaponEnchant>,

    /// Number of distinct armor weights worn (1-3, defaults to 3 for 5/1/1 builds)
    #[arg(long, default_value = "3")]
    pub armor_types: u8,

    /// Average resource percentage for resource-scaling sets like Bahsei's (0-100, default 50)
    #[arg(long, default_value = "50")]
    pub avg_resource_pct: f64,

    /// Override computed max stamina
    #[arg(long)]
    pub max_stamina: Option<f64>,

    /// Override computed max magicka
    #[arg(long)]
    pub max_magicka: Option<f64>,

    /// Override computed weapon damage
    #[arg(long)]
    pub weapon_damage: Option<f64>,

    /// Override computed spell damage
    #[arg(long)]
    pub spell_damage: Option<f64>,

    /// Override computed critical rating
    #[arg(long)]
    pub critical_rating: Option<f64>,

    /// Override computed penetration
    #[arg(long)]
    pub penetration: Option<f64>,

    /// Disable trial dummy buffs/debuffs (enabled by default)
    #[arg(long = "no-trial")]
    pub no_trial: bool,

    /// Export build to this file without prompting
    #[arg(short = 'o', long)]
    pub output: Option<PathBuf>,

    /// Number of parallel threads to use (default: half of available CPUs)
    #[arg(short = 'p', long)]
    pub parallelism: Option<u8>,

    /// Cap non-spammable skill pool per skill-line combo (prune lowest-damage skills)
    #[arg(long)]
    pub max_pool_size: Option<usize>,
}

/// Bundled export options for writing build config JSON.
struct ExportOptions {
    bar1_weapon: Option<WeaponType>,
    bar2_weapon: Option<WeaponType>,
    bar1_enchant: Option<WeaponEnchant>,
    bar2_enchant: Option<WeaponEnchant>,
    armor_weight: Option<ArmorWeight>,
    potion: Option<Potion>,
    armor_types: u8,
    avg_resource_pct: f64,
    trial: bool,
}

impl OptimizeArgs {
    pub fn run(&self) {
        self.validate();

        let parallelism = self
            .parallelism
            .unwrap_or_else(|| (num_cpus::get() / 2).max(1) as u8);

        // Determine pinned attributes
        let pinned_attributes = if self.magicka {
            Some(AttributeChoice::Magicka)
        } else if self.stamina {
            Some(AttributeChoice::Stamina)
        } else {
            None
        };

        // ── Build baseline GearConfig for Phase 0 ──
        // Pinned dimensions use the pinned value; unpinned use sensible defaults.
        let baseline_gear = GearConfig {
            race: self.race,     // None if unpinned (naked baseline)
            mundus: self.mundus, // None if unpinned
            food: self.food,     // None if unpinned
            armor_trait: self.armor_trait.unwrap_or(ArmorTrait::Divines),
            jewelry_trait: self.jewelry_trait.unwrap_or(JewelryTrait::Bloodthirsty),
            weapon_trait: self.weapon_trait.unwrap_or(WeaponTrait::Nirnhoned),
            attributes: pinned_attributes.unwrap_or(AttributeChoice::Stamina),
            armor_weight: self.armor_weight.unwrap_or(ArmorWeight::Medium),
        };

        // Derive bar weapons from positional --weapon values
        let (bar1_weapon, bar2_weapon) = match self.weapon.as_deref() {
            Some([w1, w2, ..]) => (Some(*w1), Some(*w2)),
            Some([w1]) => (Some(*w1), None),
            _ => (None, None),
        };

        let mut character_stats = baseline_gear.compute_stats(bar1_weapon);

        // Apply stat overrides
        if let Some(v) = self.max_stamina {
            character_stats.max_stamina = v;
        }
        if let Some(v) = self.max_magicka {
            character_stats.max_magicka = v;
        }
        if let Some(v) = self.weapon_damage {
            character_stats.weapon_damage = v;
        }
        if let Some(v) = self.spell_damage {
            character_stats.spell_damage = v;
        }
        if let Some(v) = self.critical_rating {
            character_stats.critical_rating = v;
        }
        if let Some(v) = self.penetration {
            character_stats.penetration = v;
        }

        let baseline_stats = character_stats.clone();

        // Resolve pinned set bonuses for Phase 0
        let (set_bonuses, set_names, _set_proc_effects) = self.resolve_set_bonuses();

        // Resolve trial dummy buffs
        let extra_bonuses = if self.no_trial {
            Vec::new()
        } else {
            TRIAL_DUMMY_BUFFS.clone()
        };

        // Resolve armor passives and potion bonuses
        let armor_weight = self.armor_weight.unwrap_or(ArmorWeight::Medium);
        let potion = self.potion.unwrap_or(Potion::WeaponPower);
        let mut armor_passive_bonuses = armor_passives(armor_weight);
        armor_passive_bonuses.extend(undaunted_mettle_bonuses(self.armor_types));
        armor_passive_bonuses.extend(potion.bonuses());

        // ── Phase 0: BuildOptimizer with baseline stats ──
        logger::info("Phase 0: Finding optimal skill/CP build...");

        let optimizer = BuildOptimizer::new(BuildOptimizerOptions {
            character_stats,
            verbose: self.verbose,
            pure: self.pure,
            required_class_names: self.class.clone().unwrap_or_default(),
            required_weapon_skill_lines: self
                .weapon
                .as_ref()
                .map(|ws| ws.iter().map(|w| w.skill_line()).collect())
                .unwrap_or_default(),
            required_champion_points: self.champion_point.clone().unwrap_or_default(),
            required_skills: self.skill.clone().unwrap_or_default(),
            parallelism,
            max_pool_size: self.max_pool_size,
            set_bonuses,
            set_names,
            extra_bonuses: extra_bonuses.clone(),
            armor_passive_bonuses: armor_passive_bonuses.clone(),
        });

        let start = Instant::now();
        let mut builds = optimizer.find_optimal_build();
        let elapsed = start.elapsed();

        if builds.is_empty() {
            logger::error("No valid build found with the given constraints.");
            std::process::exit(1);
        }

        logger::info(&builds[0].to_string());
        logger::info(&format!("Phase 0 completed in {:.2?}", elapsed));

        // ── Phase 1: Gear Optimization ──
        let gear_options = GearOptimizerOptions {
            pinned_race: self.race,
            pinned_mundus: self.mundus,
            pinned_food: self.food,
            pinned_armor_trait: self.armor_trait,
            pinned_jewelry_trait: self.jewelry_trait,
            pinned_weapon_trait: self.weapon_trait,
            pinned_attributes,
            bar1_weapon,
            top_k: 3,
            verbose: self.verbose,
        };

        let winning_gear = if gear_options.all_pinned() {
            if self.verbose {
                logger::dim("Phase 1: All gear dimensions pinned, skipping gear optimization.");
            }
            None
        } else {
            logger::info("Phase 1: Optimizing gear (race, mundus, food, traits)...");
            let gear_start = Instant::now();
            let result = GearOptimizer::optimize(&builds, &gear_options, &baseline_gear);
            let gear_elapsed = gear_start.elapsed();

            // Display winning gear
            let g = &result.gear_config;
            logger::success(&format!(
                "Best gear: Race={}, Mundus={}, Food={}, Armor={}, Jewelry={}, Weapon={}, Attributes={}",
                g.race.map_or("None".to_string(), |r| r.to_string()),
                g.mundus.map_or("None".to_string(), |m| m.to_string()),
                g.food.map_or("None".to_string(), |f| f.to_string()),
                g.armor_trait,
                g.jewelry_trait,
                g.weapon_trait,
                g.attributes,
            ));
            logger::info(&format!("Phase 1 completed in {:.2?}", gear_elapsed));
            Some(result)
        };

        // ── Phase 2: Conditional BuildOptimizer re-run ──
        if let Some(ref gear_result) = winning_gear {
            let mut new_stats = gear_result.character_stats.clone();

            // Re-apply stat overrides on top of winning gear stats
            if let Some(v) = self.max_stamina {
                new_stats.max_stamina = v;
            }
            if let Some(v) = self.max_magicka {
                new_stats.max_magicka = v;
            }
            if let Some(v) = self.weapon_damage {
                new_stats.weapon_damage = v;
            }
            if let Some(v) = self.spell_damage {
                new_stats.spell_damage = v;
            }
            if let Some(v) = self.critical_rating {
                new_stats.critical_rating = v;
            }
            if let Some(v) = self.penetration {
                new_stats.penetration = v;
            }

            if stats_differ_significantly(&baseline_stats, &new_stats, 0.05) {
                logger::info("Phase 2: Gear stats changed >5%, re-running build optimizer...");

                let (set_bonuses, set_names, _set_proc_effects) = self.resolve_set_bonuses();
                let rerun_optimizer = BuildOptimizer::new(BuildOptimizerOptions {
                    character_stats: new_stats,
                    verbose: self.verbose,
                    pure: self.pure,
                    required_class_names: self.class.clone().unwrap_or_default(),
                    required_weapon_skill_lines: self
                        .weapon
                        .as_ref()
                        .map(|ws| ws.iter().map(|w| w.skill_line()).collect())
                        .unwrap_or_default(),
                    required_champion_points: self.champion_point.clone().unwrap_or_default(),
                    required_skills: self.skill.clone().unwrap_or_default(),
                    parallelism,
                    max_pool_size: self.max_pool_size,
                    set_bonuses,
                    set_names,
                    extra_bonuses: extra_bonuses.clone(),
                    armor_passive_bonuses: armor_passive_bonuses.clone(),
                });

                let rerun_start = Instant::now();
                let new_builds = rerun_optimizer.find_optimal_build();
                let rerun_elapsed = rerun_start.elapsed();

                if !new_builds.is_empty() {
                    logger::info(&new_builds[0].to_string());
                    logger::info(&format!("Phase 2 completed in {:.2?}", rerun_elapsed));
                    builds = new_builds;
                } else {
                    logger::warn("Phase 2 re-run found no valid builds, keeping Phase 0 results.");
                }
            } else if self.verbose {
                logger::dim("Phase 2: Stats within 5% of baseline, skipping re-run.");
            }
        }

        // ── Phase 3: Set Optimization (always runs) ──
        logger::info("Phase 3: Optimizing gear sets...");
        let pinned_sets = self.set.clone().unwrap_or_default();
        let (pinned_normal, pinned_monster, pinned_mythic_vec) =
            Self::split_sets_by_type(&pinned_sets);
        let set_result = SetOptimizer::optimize(
            &builds,
            &SetOptimizerOptions {
                top_k: 10,
                pinned_normal,
                pinned_monster,
                pinned_mythic: pinned_mythic_vec.into_iter().next(),
                parallelism,
                verbose: self.verbose,
            },
        );
        let builds = if let Some(result) = set_result {
            let source = &builds[result.build_idx];
            let best_with_sets = Build::new_with_extra(
                source.skills().to_vec(),
                source.cp_bonuses(),
                source.passive_bonuses(),
                &result.set_bonuses,
                result.set_names,
                source.character_stats().clone(),
                &extra_bonuses,
            );
            logger::info(&best_with_sets.to_string());
            vec![best_with_sets]
        } else {
            logger::warn("Set optimization found no valid loadout.");
            builds
        };

        // ── Phase 4: Fight Simulation ──
        let sim_result = self.run_simulation(&builds);

        // Use the build selected by simulation (if any), otherwise fall back to DPC-best
        let best_build = &builds[0];
        let export_build = sim_result
            .as_ref()
            .map(|(build_idx, _, _, _, _, _)| &builds[*build_idx])
            .unwrap_or(best_build);
        let sim_data = sim_result
            .as_ref()
            .map(|(_, dist, result, _, _, _)| (dist, result));
        let buffed_stats = sim_result.as_ref().map(|(_, _, _, _, _, stats)| stats);

        // Export with gear info (use winning enchants from simulation if available)
        let (winning_bar1, winning_bar2) = sim_result
            .as_ref()
            .map(|(_, _, _, e1, e2, _)| (*e1, *e2))
            .unwrap_or((
                self.bar1_enchant.unwrap_or(WeaponEnchant::Flame),
                self.bar2_enchant.unwrap_or(WeaponEnchant::Flame),
            ));
        let gear_config = winning_gear.as_ref().map(|g| &g.gear_config);
        let export_opts = ExportOptions {
            bar1_weapon,
            bar2_weapon,
            bar1_enchant: Some(winning_bar1),
            bar2_enchant: Some(winning_bar2),
            armor_weight: Some(self.armor_weight.unwrap_or(ArmorWeight::Medium)),
            potion: Some(self.potion.unwrap_or(Potion::WeaponPower)),
            armor_types: self.armor_types,
            avg_resource_pct: self.avg_resource_pct,
            trial: !self.no_trial,
        };
        if let Some(path) = &self.output {
            Self::export_to_file(
                export_build,
                sim_data,
                buffed_stats,
                gear_config,
                &export_opts,
                path,
            );
        } else {
            Self::prompt_export(
                export_build,
                sim_data,
                buffed_stats,
                gear_config,
                &export_opts,
            );
        }
    }

    fn validate(&self) {
        // Validate class count
        if let Some(classes) = &self.class {
            if classes.len() > BUILD_CONSTRAINTS.class_skill_line_count {
                logger::error(&format!(
                    "Maximum {} classes allowed",
                    BUILD_CONSTRAINTS.class_skill_line_count
                ));
                std::process::exit(1);
            }
        }

        // Validate weapon count
        if let Some(weapons) = &self.weapon {
            if weapons.len() > BUILD_CONSTRAINTS.weapon_skill_line_count {
                logger::error(&format!(
                    "Maximum {} weapons allowed",
                    BUILD_CONSTRAINTS.weapon_skill_line_count
                ));
                std::process::exit(1);
            }
        }

        // Validate required skills count
        if let Some(skills) = &self.skill {
            if skills.len() > BUILD_CONSTRAINTS.skill_count {
                logger::error(&format!(
                    "Maximum {} required skills allowed",
                    BUILD_CONSTRAINTS.skill_count
                ));
                std::process::exit(1);
            }
        }

        // Validate champion point count
        if let Some(cp) = &self.champion_point {
            if cp.len() > BUILD_CONSTRAINTS.champion_point_count {
                logger::error(&format!(
                    "Maximum {} champion points allowed",
                    BUILD_CONSTRAINTS.champion_point_count
                ));
                std::process::exit(1);
            }
        }

        // Validate set counts by type
        if let Some(sets) = &self.set {
            let (normals, monsters, mythics) = Self::split_sets_by_type(sets);
            if normals.len() > 2 {
                logger::error("Maximum 2 normal/arena sets allowed");
                std::process::exit(1);
            }
            if monsters.len() > 2 {
                logger::error("Maximum 2 monster sets allowed");
                std::process::exit(1);
            }
            if mythics.len() > 1 {
                logger::error("Maximum 1 mythic set allowed");
                std::process::exit(1);
            }
        }
    }

    /// Split a flat list of sets into (normal/arena, monster, mythic) groups.
    fn split_sets_by_type(
        sets: &[&'static SetData],
    ) -> (
        Vec<&'static SetData>,
        Vec<&'static SetData>,
        Vec<&'static SetData>,
    ) {
        let mut normals = Vec::new();
        let mut monsters = Vec::new();
        let mut mythics = Vec::new();
        for &set in sets {
            match set.set_type {
                SetType::Normal | SetType::Arena => normals.push(set),
                SetType::Monster => monsters.push(set),
                SetType::Mythic => mythics.push(set),
            }
        }
        (normals, monsters, mythics)
    }

    fn resolve_set_bonuses(&self) -> (Vec<BonusData>, Vec<(String, u8)>, Vec<SetProcEffect>) {
        let active_sets = self.set.clone().unwrap_or_default();

        let mut set_bonuses: Vec<BonusData> = Vec::new();
        let mut set_names: Vec<(String, u8)> = Vec::new();
        let mut set_proc_effects: Vec<SetProcEffect> = Vec::new();
        for set in &active_sets {
            let piece_count = set.set_type.max_pieces();
            let bonuses = set.bonuses_at(piece_count);
            set_bonuses.extend(bonuses.into_iter().cloned());
            set_proc_effects.extend(set.proc_effects_at(piece_count).into_iter().cloned());
            set_names.push((set.name.clone(), piece_count));
        }

        (set_bonuses, set_names, set_proc_effects)
    }

    fn run_simulation(
        &self,
        builds: &[crate::domain::Build],
    ) -> Option<(
        usize,
        BarDistribution,
        SimulationResult,
        WeaponEnchant,
        WeaponEnchant,
        CharacterStats,
    )> {
        // Derive bar weapons from --weapon positional args, or infer from skills
        let inferred = {
            let top_skills = builds[0].skills();
            match infer_weapons(top_skills) {
                Ok(weapons) => Some(weapons),
                Err(e) => {
                    if self.weapon.is_none() {
                        logger::warn(&format!(
                            "Could not infer weapons for simulation: {}. Skipping fight simulation.",
                            e
                        ));
                        return None;
                    }
                    None
                }
            }
        };
        let (bar1_weapon, bar2_weapon) = match self.weapon.as_deref() {
            Some([w1, w2, ..]) => (*w1, *w2),
            Some([w1]) => (*w1, inferred.map(|(_, w2)| w2).unwrap_or(*w1)),
            _ => inferred.unwrap(),
        };

        logger::info(&format!(
            "Phase 4: Running fight simulation on top {} candidates (Bar1: {}, Bar2: {})...",
            builds.len(),
            bar1_weapon,
            bar2_weapon
        ));

        let sim_start = Instant::now();

        // Pre-compute work items: (build_idx, simulator, distributions)
        let work: Vec<(usize, FightSimulator, Vec<BarDistribution>)> = builds
            .iter()
            .enumerate()
            .filter_map(|(build_idx, build)| {
                let distributions =
                    generate_distributions(build.skills(), bar1_weapon, bar2_weapon);
                if distributions.is_empty() {
                    return None;
                }
                let mut suppressed = if self.no_trial {
                    std::collections::HashSet::new()
                } else {
                    TRIAL_BUFF_NAMES.clone()
                };
                // Suppress potion buff names to prevent double-counting with skill buffs
                let potion = self.potion.unwrap_or(Potion::WeaponPower);
                for bonus in potion.bonuses() {
                    suppressed.insert(bonus.name.clone());
                }
                let bar1_enchant = self.bar1_enchant.or(Some(WeaponEnchant::Flame));
                let bar2_enchant = self.bar2_enchant.or(Some(WeaponEnchant::Flame));
                // Collect proc effects from the build's equipped sets
                let proc_effects: Vec<SetProcEffect> = build
                    .set_names()
                    .iter()
                    .flat_map(|(name, _)| {
                        ALL_SETS
                            .iter()
                            .filter(move |s| s.name == *name)
                            .flat_map(|s| {
                                s.proc_effects_at(s.set_type.max_pieces())
                                    .into_iter()
                                    .cloned()
                            })
                    })
                    .collect();
                let simulator = FightSimulator::new(
                    build.effective_stats(),
                    build.resolved_bonuses(),
                    suppressed,
                )
                .with_enchants(bar1_enchant, bar2_enchant)
                .with_set_procs(proc_effects)
                .with_avg_resource_pct(self.avg_resource_pct);
                Some((build_idx, simulator, distributions))
            })
            .collect();

        let total_sims: usize = work.iter().map(|(_, _, d)| d.len()).sum();
        let completed = AtomicUsize::new(0);
        let best_dps_bits = AtomicU64::new(f64::NEG_INFINITY.to_bits());

        let results: Vec<_> = work
            .par_iter()
            .filter_map(|(build_idx, simulator, distributions)| {
                let mut local_best: Option<(usize, SimulationResult)> = None;
                for (dist_idx, dist) in distributions.iter().enumerate() {
                    let result = simulator.simulate(dist);
                    if local_best
                        .as_ref()
                        .map_or(true, |(_, r)| result.dps > r.dps)
                    {
                        local_best = Some((dist_idx, result));
                    }

                    let done = completed.fetch_add(1, Ordering::Relaxed) + 1;

                    // Update shared best DPS via CAS loop
                    if let Some((_, ref best)) = local_best {
                        let new_bits = best.dps.to_bits();
                        let mut current = best_dps_bits.load(Ordering::Relaxed);
                        loop {
                            if f64::from_bits(current) >= best.dps {
                                break;
                            }
                            match best_dps_bits.compare_exchange_weak(
                                current,
                                new_bits,
                                Ordering::Relaxed,
                                Ordering::Relaxed,
                            ) {
                                Ok(_) => break,
                                Err(actual) => current = actual,
                            }
                        }
                    }

                    if done % 10 == 0 || done == total_sims {
                        let best = f64::from_bits(best_dps_bits.load(Ordering::Relaxed));
                        let best_str = if best > 0.0 {
                            format::format_number(best as u64)
                        } else {
                            "---".to_string()
                        };
                        logger::progress(&format!(
                            "Simulating: {}/{} | Best DPS: {}",
                            done, total_sims, best_str
                        ));
                    }
                }
                local_best
                    .map(|(dist_idx, result)| (*build_idx, dist_idx, distributions.clone(), result))
            })
            .collect();

        let sim_elapsed = sim_start.elapsed();

        // Find global best from parallel results
        if let Some((best_build_idx, best_dist_idx, distributions, mut result)) = results
            .into_iter()
            .max_by(|(_, _, _, a), (_, _, _, b)| a.dps.partial_cmp(&b.dps).unwrap())
        {
            if best_build_idx > 0 {
                logger::info(&format!(
                    "Simulation selected build #{} (of {} candidates) as best DPS.",
                    best_build_idx + 1,
                    builds.len()
                ));
                logger::info(&builds[best_build_idx].to_string());
            }
            let best_dist = distributions[best_dist_idx].clone();

            // ── Enchant optimization sweep ──
            let bar1_pinned = self.bar1_enchant.is_some();
            let bar2_pinned = self.bar2_enchant.is_some();
            let mut winning_bar1 = self.bar1_enchant.unwrap_or(WeaponEnchant::Flame);
            let mut winning_bar2 = self.bar2_enchant.unwrap_or(WeaponEnchant::Flame);

            if !bar1_pinned || !bar2_pinned {
                let all_enchants = [
                    WeaponEnchant::Flame,
                    WeaponEnchant::Poison,
                    WeaponEnchant::Shock,
                    WeaponEnchant::Berserker,
                ];
                let bar1_candidates: Vec<WeaponEnchant> = if bar1_pinned {
                    vec![winning_bar1]
                } else {
                    all_enchants.to_vec()
                };
                let bar2_candidates: Vec<WeaponEnchant> = if bar2_pinned {
                    vec![winning_bar2]
                } else {
                    all_enchants.to_vec()
                };

                let build = &builds[best_build_idx];
                let mut suppressed = if self.no_trial {
                    std::collections::HashSet::new()
                } else {
                    TRIAL_BUFF_NAMES.clone()
                };
                let potion = self.potion.unwrap_or(Potion::WeaponPower);
                for bonus in potion.bonuses() {
                    suppressed.insert(bonus.name.clone());
                }
                let proc_effects: Vec<SetProcEffect> = build
                    .set_names()
                    .iter()
                    .flat_map(|(name, _)| {
                        ALL_SETS
                            .iter()
                            .filter(move |s| s.name == *name)
                            .flat_map(|s| {
                                s.proc_effects_at(s.set_type.max_pieces())
                                    .into_iter()
                                    .cloned()
                            })
                    })
                    .collect();

                let combo_count = bar1_candidates.len() * bar2_candidates.len();
                let mut best_enchant_dps = result.dps;

                for &e1 in &bar1_candidates {
                    for &e2 in &bar2_candidates {
                        if e1 == winning_bar1 && e2 == winning_bar2 {
                            continue; // already tested
                        }
                        let sim = FightSimulator::new(
                            build.effective_stats(),
                            build.resolved_bonuses(),
                            suppressed.clone(),
                        )
                        .with_enchants(Some(e1), Some(e2))
                        .with_set_procs(proc_effects.clone())
                        .with_avg_resource_pct(self.avg_resource_pct);

                        let r = sim.simulate(&best_dist);
                        if r.dps > best_enchant_dps {
                            best_enchant_dps = r.dps;
                            winning_bar1 = e1;
                            winning_bar2 = e2;
                            result = r;
                        }
                    }
                }

                logger::success(&format!(
                    "Best enchants: Bar1={}, Bar2={} (optimized from {} combos)",
                    winning_bar1, winning_bar2, combo_count
                ));
            }

            display_simulation_result(
                &result,
                &best_dist,
                distributions.len(),
                builds[best_build_idx].set_names(),
            );
            logger::info(&format!("Simulation completed in {:.2?}", sim_elapsed));

            // Compute buffed stats for export metadata
            let build = &builds[best_build_idx];
            let mut suppressed_final = if self.no_trial {
                std::collections::HashSet::new()
            } else {
                TRIAL_BUFF_NAMES.clone()
            };
            let potion_final = self.potion.unwrap_or(Potion::WeaponPower);
            for bonus in potion_final.bonuses() {
                suppressed_final.insert(bonus.name.clone());
            }
            let proc_effects_final: Vec<SetProcEffect> = build
                .set_names()
                .iter()
                .flat_map(|(name, _)| {
                    ALL_SETS
                        .iter()
                        .filter(move |s| s.name == *name)
                        .flat_map(|s| {
                            s.proc_effects_at(s.set_type.max_pieces())
                                .into_iter()
                                .cloned()
                        })
                })
                .collect();
            let final_sim = FightSimulator::new(
                build.effective_stats(),
                build.resolved_bonuses(),
                suppressed_final,
            )
            .with_enchants(Some(winning_bar1), Some(winning_bar2))
            .with_set_procs(proc_effects_final)
            .with_avg_resource_pct(self.avg_resource_pct);
            let buffed_stats = final_sim.compute_buffed_stats(&best_dist);

            return Some((
                best_build_idx,
                best_dist,
                result,
                winning_bar1,
                winning_bar2,
                buffed_stats,
            ));
        }

        None
    }

    fn prompt_export(
        build: &crate::domain::Build,
        simulation: Option<(&BarDistribution, &SimulationResult)>,
        buffed_stats: Option<&CharacterStats>,
        gear_config: Option<&GearConfig>,
        opts: &ExportOptions,
    ) {
        // Show prompt with greyed-out default value "no"
        print!("\nExport build to file? [path/no]: \x1b[90mn\x1b[0m");
        // Move cursor back over the default value so user input overwrites it
        print!("\x1b[1D");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            return;
        }

        let input = input.trim();
        if input.is_empty() || input.eq_ignore_ascii_case("no") || input.eq_ignore_ascii_case("n") {
            return;
        }

        let path = PathBuf::from(input);
        Self::export_to_file(build, simulation, buffed_stats, gear_config, opts, &path);
    }

    fn export_to_file(
        build: &crate::domain::Build,
        simulation: Option<(&BarDistribution, &SimulationResult)>,
        buffed_stats: Option<&CharacterStats>,
        gear_config: Option<&GearConfig>,
        opts: &ExportOptions,
        path: &PathBuf,
    ) {
        let metadata = simulation.map(|(dist, result)| super::build_config::BuildMetadata {
            dps: result.dps,
            total_damage: result.total_damage,
            fight_duration: result.fight_duration,
            bar1_skills: dist
                .bar1
                .skills
                .iter()
                .map(|s| s.name.to_string())
                .collect(),
            bar2_skills: dist
                .bar2
                .skills
                .iter()
                .map(|s| s.name.to_string())
                .collect(),
            buffed_stats: buffed_stats.cloned(),
        });

        let config = BuildConfig {
            skills: build.skill_names(),
            champion_points: build.champion_point_names(),
            sets: build
                .set_names()
                .iter()
                .map(|(name, _)| name.clone())
                .collect(),
            bar1_weapon: opts.bar1_weapon.map(|w| w.to_string()),
            bar2_weapon: opts.bar2_weapon.map(|w| w.to_string()),
            character_stats: build.character_stats().clone(),
            race: gear_config.and_then(|g| g.race.map(|r| r.to_string())),
            mundus: gear_config.and_then(|g| g.mundus.map(|m| m.to_string())),
            food: gear_config.and_then(|g| g.food.map(|f| f.to_string())),
            armor_trait: gear_config.map(|g| g.armor_trait.to_string()),
            jewelry_trait: gear_config.map(|g| g.jewelry_trait.to_string()),
            weapon_trait: gear_config.map(|g| g.weapon_trait.to_string()),
            bar1_enchant: opts.bar1_enchant.map(|e| e.to_string()),
            bar2_enchant: opts.bar2_enchant.map(|e| e.to_string()),
            armor_weight: opts.armor_weight.map(|w| w.to_string()),
            potion: opts.potion.map(|p| p.to_string()),
            armor_types: opts.armor_types,
            avg_resource_pct: opts.avg_resource_pct,
            trial: opts.trial,
            metadata,
        };

        match serde_json::to_string_pretty(&config) {
            Ok(json) => match fs::write(path, json) {
                Ok(_) => logger::info(&format!("Build exported to {}", path.display())),
                Err(e) => logger::error(&format!("Failed to write file: {}", e)),
            },
            Err(e) => logger::error(&format!("Failed to serialize build: {}", e)),
        }
    }
}
