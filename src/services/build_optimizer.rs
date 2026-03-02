use crate::data::bonuses::CHAMPION_POINTS;
use crate::data::skills::ALL_SKILLS;
use crate::domain::{
    BonusData, BonusTrigger, Build, CharacterStats, DamageFlags, ResolvedBonus, SkillData,
    BUILD_CONSTRAINTS,
};
use crate::domain::{ClassName, ResolveContext, SkillLineName};
use crate::infrastructure::{combinatorics, format, logger, table};
use crate::services::passives_service::{PassivesFilter, PassivesServiceOptions};
use crate::services::skills_service::{MorphSelectionOptions, SkillsFilter, SkillsServiceOptions};
use crate::services::{PassivesService, SkillsService};
use rayon::prelude::*;
use rayon::ThreadPoolBuilder;
use smallvec::SmallVec;
use std::collections::{HashMap, HashSet};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Instant;

#[derive(Debug, Clone)]
pub struct BuildOptimizerOptions {
    pub character_stats: CharacterStats,
    pub verbose: bool,
    pub pure_class: Option<ClassName>,
    pub required_class_names: Vec<ClassName>,
    pub required_weapon_skill_lines: Vec<SkillLineName>,
    pub required_champion_points: Vec<BonusData>,
    pub required_skills: Vec<&'static SkillData>,
    pub forced_morphs: Vec<String>,
    pub parallelism: u8,
    pub max_pool_size: Option<usize>,
    pub set_bonuses: Vec<BonusData>,
    pub set_names: Vec<(String, u8)>,
    pub extra_bonuses: Vec<BonusData>,
}

/// Three-way split of bonuses for the optimizer fast path:
/// - `pre_resolved`: simple non-AbilitySlottedCount bonuses pre-resolved to ResolvedBonus
/// - `ability_count`: simple AbilitySlottedCount bonuses (multiplier depends on skills)
/// - `alt`: alternative bonuses (need per-eval resolution)
type PreSplitBonuses = (Vec<ResolvedBonus>, Vec<BonusData>, Vec<BonusData>);

pub struct BuildOptimizer {
    character_stats: CharacterStats,
    required_class_names: Vec<ClassName>,
    class_names: HashSet<ClassName>,
    required_weapon_skill_lines: Vec<SkillLineName>,
    weapon_skill_line_names: HashSet<SkillLineName>,
    required_champion_points: Vec<String>,
    champion_point_names: HashSet<String>,
    required_skill_names: Vec<String>,
    skill_names: HashSet<String>,
    parallelism: u8,

    /// Required non-spammable skills prepended to every combination
    required_non_spammable: Vec<&'static SkillData>,
    /// Required finisher skill (if any) — always included in builds
    required_finisher: Option<&'static SkillData>,

    /// Pre-split champion point combinations: (pre_resolved, ability_count, alt)
    champion_point_combinations: Vec<PreSplitBonuses>,
    /// Original champion point BonusData for final Build reconstruction
    champion_point_original: Vec<Vec<BonusData>>,
    spammable_skills: Vec<Vec<&'static SkillData>>,
    finisher_skills: Vec<Vec<&'static SkillData>>,
    non_spammable_skills: Vec<Vec<&'static SkillData>>,
    /// Pre-split passive bonus lists: (pre_resolved, ability_count, alt)
    passive_bonuses_list: Vec<PreSplitBonuses>,
    /// Original passive BonusData for final Build reconstruction
    passive_original: Vec<Vec<BonusData>>,
    total_possible_build_count: u64,
    /// Original set BonusData for final Build reconstruction
    set_bonuses: Vec<BonusData>,
    /// Set names and piece counts for display/export
    set_names: Vec<(String, u8)>,
    /// Extra bonuses (e.g. trial dummy buffs) for final Build reconstruction
    extra_bonuses: Vec<BonusData>,
}

// Constructor
impl BuildOptimizer {
    pub fn new(options: BuildOptimizerOptions) -> Self {
        let verbose = options.verbose;
        let parallelism = options.parallelism;
        let mut required_class_names = options.required_class_names;
        let mut required_weapon_skill_lines = options.required_weapon_skill_lines;
        let required_champion_points: Vec<String> = options
            .required_champion_points
            .iter()
            .map(|cp| cp.name.clone())
            .collect();
        let pure_class = options.pure_class;

        // Auto-infer constraints from required skills
        let mut forced_morphs = options.forced_morphs.clone();
        for skill in &options.required_skills {
            // Auto-force morph so morph selection picks the correct morph
            if !forced_morphs.contains(&skill.name) {
                forced_morphs.push(skill.name.clone());
            }

            let sl = skill.skill_line;
            if sl.is_weapon() {
                if !required_weapon_skill_lines.contains(&sl) {
                    required_weapon_skill_lines.push(sl);
                }
            } else if !sl.is_guild() {
                // Class skill line
                let class = sl.get_class();
                if let Some(pure) = pure_class {
                    if class != pure {
                        logger::error(&format!(
                            "Required skill '{}' is from class {} but --pure {} was specified",
                            skill.name, class, pure
                        ));
                        std::process::exit(1);
                    }
                }
                if !required_class_names.contains(&class) {
                    required_class_names.push(class);
                }
            }
        }

        // Classify required skills into spammable / finisher / non-spammable
        let mut required_spammable: Option<&'static SkillData> = None;
        let mut required_finisher: Option<&'static SkillData> = None;
        let mut required_non_spammable: Vec<&'static SkillData> = Vec::new();
        let required_skill_names: Vec<String> = options
            .required_skills
            .iter()
            .map(|s| s.name.clone())
            .collect();

        for skill in &options.required_skills {
            if skill.spammable && skill.bonuses.is_none() && skill.execute.is_none() {
                if required_spammable.is_some() {
                    logger::error(
                        "At most 1 pure spammable skill can be required (spammable with no bonuses and no execute)",
                    );
                    std::process::exit(1);
                }
                required_spammable = Some(skill);
            } else if skill.spammable && skill.bonuses.is_none() && skill.execute.is_some() {
                if required_finisher.is_some() {
                    logger::error(
                        "At most 1 finisher skill can be required (spammable with execute and no bonuses)",
                    );
                    std::process::exit(1);
                }
                required_finisher = Some(skill);
            } else {
                required_non_spammable.push(skill);
            }
        }

        if verbose {
            logger::dim(&format!(
                "Total skills before morph selection: {}",
                ALL_SKILLS.len()
            ));
        }

        let skills_service = SkillsService::new(SkillsServiceOptions::default())
            .with_morph_selection(MorphSelectionOptions { forced_morphs })
            .with_filter(SkillsFilter {
                exclude_ultimates: true,
                exclude_non_damaging: true, // TODO: only exclude if no buff
            });

        let (class_names, class_skill_line_combinations) =
            Self::generate_class_skill_line_combinations(
                pure_class,
                &required_class_names,
                verbose,
            );

        let (weapon_skill_line_names, weapon_skill_line_combinations) =
            Self::generate_weapon_skill_line_combinations(&required_weapon_skill_lines, verbose);

        let guild_skill_lines = SkillLineName::GUILD.to_vec();
        let skill_line_combinations: Vec<Vec<SkillLineName>> = combinatorics::cartesian_product(
            &class_skill_line_combinations,
            &weapon_skill_line_combinations,
        )
        .into_iter()
        .map(|mut combo| {
            combo.extend_from_slice(&guild_skill_lines);
            combo
        })
        .collect();

        if verbose {
            logger::dim(&format!(
                "Generated {} total skill line combinations (class + weapon)",
                skill_line_combinations.len()
            ));
        }

        let mut skill_names: HashSet<String> = HashSet::new();
        let mut spammable_skills = Self::generate_spammable_skills(
            &skill_line_combinations,
            &skills_service,
            &mut skill_names,
        );
        let mut finisher_skills = Self::generate_finisher_skills(
            &skill_line_combinations,
            &skills_service,
            &mut skill_names,
        );
        let mut non_spammable_skills = Self::generate_non_spammable_skills(
            &skill_line_combinations,
            &skills_service,
            &mut skill_names,
            options.max_pool_size,
            &options.character_stats,
            verbose,
        );

        // Filter pools based on required skills
        if required_spammable.is_some() {
            let req = required_spammable.unwrap();
            for pool in &mut spammable_skills {
                if pool.iter().any(|s| s.name == req.name) {
                    *pool = vec![req];
                } else {
                    pool.clear();
                }
            }
        }

        if required_finisher.is_some() {
            let req = required_finisher.unwrap();
            for pool in &mut finisher_skills {
                if pool.iter().any(|s| s.name == req.name) {
                    *pool = vec![req];
                } else {
                    pool.clear();
                }
            }
        }

        // Remove required non-spammable skills from the variable pool
        if !required_non_spammable.is_empty() {
            let req_names: HashSet<&str> = required_non_spammable
                .iter()
                .map(|s| s.name.as_str())
                .collect();
            for pool in &mut non_spammable_skills {
                pool.retain(|s| !req_names.contains(s.name.as_str()));
            }
        }

        let has_any_spammable = spammable_skills.iter().any(|skills| !skills.is_empty());
        if !has_any_spammable {
            logger::error(
                "No spammable skills found in skill pool. Cannot optimize without a spammable.",
            );
            std::process::exit(1);
        }

        // Collect extra bonus names to suppress duplicate passives
        let extra_bonus_names: HashSet<String> = options.extra_bonuses.iter().map(|b| b.name.clone()).collect();

        let (passive_bonuses_list, passive_original) =
            Self::generate_passive_bonuses(&skill_line_combinations, &extra_bonus_names, verbose);

        let (champion_point_names, mut champion_point_combinations, champion_point_original) =
            Self::generate_champion_point_combinations(&required_champion_points, verbose);

        // Merge fixed set bonuses into CP combo pre_resolved buckets
        // (set bonuses are always BonusTrigger::Passive → always pre_resolved)
        let set_bonuses = options.set_bonuses;
        let set_names = options.set_names;
        if !set_bonuses.is_empty() {
            let (set_pre, set_ability, set_alt) = Self::three_way_split(set_bonuses.clone());
            for (pre_resolved, ability_count, alt) in &mut champion_point_combinations {
                pre_resolved.extend_from_slice(&set_pre);
                ability_count.extend_from_slice(&set_ability);
                alt.extend_from_slice(&set_alt);
            }
            if verbose {
                logger::dim(&format!(
                    "Merged {} set bonuses ({}) into {} CP combinations",
                    set_bonuses.len(),
                    set_names.iter().map(|(n, p)| format!("{} ({}pc)", n, p)).collect::<Vec<_>>().join(", "),
                    champion_point_combinations.len()
                ));
            }
        }

        // Merge extra bonuses (e.g. trial dummy buffs) into CP combo buckets
        let extra_bonuses = options.extra_bonuses;
        if !extra_bonuses.is_empty() {
            let (extra_pre, extra_ability, extra_alt) = Self::three_way_split(extra_bonuses.clone());
            for (pre_resolved, ability_count, alt) in &mut champion_point_combinations {
                pre_resolved.extend_from_slice(&extra_pre);
                ability_count.extend_from_slice(&extra_ability);
                alt.extend_from_slice(&extra_alt);
            }
            if verbose {
                logger::dim(&format!(
                    "Merged {} extra bonuses (trial buffs) into {} CP combinations",
                    extra_bonuses.len(),
                    champion_point_combinations.len()
                ));
            }
        }

        let total_possible_build_count = Self::calculate_total_build_count(
            &champion_point_combinations,
            &spammable_skills,
            &finisher_skills,
            &non_spammable_skills,
            required_non_spammable.len(),
            required_finisher.is_some(),
        );

        let optimizer = Self {
            character_stats: options.character_stats,
            required_class_names,
            class_names,
            required_weapon_skill_lines,
            weapon_skill_line_names,
            required_champion_points,
            champion_point_names,
            required_skill_names,
            skill_names,
            parallelism,
            required_non_spammable,
            required_finisher,
            champion_point_combinations,
            champion_point_original,
            spammable_skills,
            finisher_skills,
            non_spammable_skills,
            passive_bonuses_list,
            passive_original,
            total_possible_build_count,
            set_bonuses,
            set_names,
            extra_bonuses,
        };

        logger::log(&optimizer.to_string());

        optimizer
    }

    /// Three-way split a list of bonuses into:
    /// - pre_resolved: simple non-AbilitySlottedCount → ResolvedBonus (multiplier always 1.0)
    /// - ability_count: simple AbilitySlottedCount → BonusData (multiplier depends on skills)
    /// - alt: alternative bonuses → BonusData (need per-eval resolution)
    fn three_way_split(bonuses: Vec<BonusData>) -> PreSplitBonuses {
        let default_ctx = ResolveContext::default();
        let mut pre_resolved = Vec::new();
        let mut ability_count = Vec::new();
        let mut alt = Vec::new();

        for bonus in bonuses {
            if bonus.has_alternative() {
                alt.push(bonus);
            } else if bonus.trigger == BonusTrigger::AbilitySlottedCount {
                ability_count.push(bonus);
            } else {
                let bv = bonus.resolve_ref(&default_ctx);
                pre_resolved.push(ResolvedBonus {
                    target: bv.target,
                    value: bv.value,
                    skill_line_filter: bonus.skill_line_filter,
                    execute_threshold: bonus.execute_threshold,
                });
            }
        }

        (pre_resolved, ability_count, alt)
    }

    fn generate_champion_point_combinations(
        required_champion_points: &[String],
        verbose: bool,
    ) -> (HashSet<String>, Vec<PreSplitBonuses>, Vec<Vec<BonusData>>) {
        let mut champion_point_names: HashSet<String> = HashSet::new();
        let cp_vec: Vec<_> = CHAMPION_POINTS.iter().cloned().collect();

        let filtered: Vec<Vec<BonusData>> =
            combinatorics::generate_combinations(&cp_vec, BUILD_CONSTRAINTS.champion_point_count)
                .into_iter()
                .filter(|combination| {
                    let has_required = required_champion_points.is_empty()
                        || required_champion_points
                            .iter()
                            .all(|required| combination.iter().any(|cp| &cp.name == required));

                    if has_required {
                        for cp in combination {
                            champion_point_names.insert(cp.name.clone());
                        }
                    }

                    has_required
                })
                .collect();

        let split: Vec<PreSplitBonuses> = filtered
            .iter()
            .map(|combo| Self::three_way_split(combo.clone()))
            .collect();

        if verbose {
            let required_str = if required_champion_points.is_empty() {
                "none".to_string()
            } else {
                required_champion_points.join(", ")
            };
            logger::dim(&format!(
                "Generated {} champion point combinations using {} CPs (required: {})",
                split.len(),
                champion_point_names.len(),
                required_str
            ));
        }

        (champion_point_names, split, filtered)
    }

    fn generate_class_skill_line_combinations(
        pure_class: Option<ClassName>,
        required_class_names: &[ClassName],
        verbose: bool,
    ) -> (HashSet<ClassName>, Vec<Vec<SkillLineName>>) {
        let mut class_names: HashSet<ClassName> = HashSet::new();

        let class_name_combinations: Vec<Vec<ClassName>> = if let Some(class) = pure_class {
            class_names.insert(class);
            vec![vec![class]]
        } else {
            combinatorics::generate_combinations(
                &ClassName::CLASS_ONLY.to_vec(),
                BUILD_CONSTRAINTS.class_skill_line_count,
            )
            .into_iter()
            .filter(|combination| {
                let has_required = required_class_names.is_empty()
                    || required_class_names
                        .iter()
                        .all(|required| combination.contains(required));

                if has_required {
                    for class in combination {
                        class_names.insert(*class);
                    }
                }

                has_required
            })
            .collect()
        };

        if verbose {
            let required_str = if required_class_names.is_empty() {
                "none".to_string()
            } else {
                required_class_names
                    .iter()
                    .map(|c| c.to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            };
            logger::dim(&format!(
                "Generated {} class combinations using {} classes (required: {})",
                class_name_combinations.len(),
                class_names.len(),
                required_str
            ));
        }

        let skill_line_combinations: Vec<Vec<SkillLineName>> = class_name_combinations
            .iter()
            .map(|class_combination| {
                class_combination
                    .iter()
                    .flat_map(|class_name| SkillLineName::for_class(*class_name))
                    .collect()
            })
            .collect();

        (class_names, skill_line_combinations)
    }

    fn generate_weapon_skill_line_combinations(
        required_weapon_skill_lines: &[SkillLineName],
        verbose: bool,
    ) -> (HashSet<SkillLineName>, Vec<Vec<SkillLineName>>) {
        let mut weapon_skill_line_names: HashSet<SkillLineName> = HashSet::new();

        let combinations: Vec<Vec<SkillLineName>> = combinatorics::generate_combinations(
            &SkillLineName::WEAPON.to_vec(),
            BUILD_CONSTRAINTS.weapon_skill_line_count,
        )
        .into_iter()
        .filter(|combination| {
            let has_required = required_weapon_skill_lines.is_empty()
                || required_weapon_skill_lines
                    .iter()
                    .all(|required| combination.contains(required));

            if has_required {
                for weapon in combination {
                    weapon_skill_line_names.insert(*weapon);
                }
            }

            has_required
        })
        .collect();

        if verbose {
            let required_str = if required_weapon_skill_lines.is_empty() {
                "none".to_string()
            } else {
                required_weapon_skill_lines
                    .iter()
                    .map(|w| w.to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            };
            logger::dim(&format!(
                "Generated {} weapon skill line combinations using {} weapons (required: {})",
                combinations.len(),
                weapon_skill_line_names.len(),
                required_str
            ));
        }

        (weapon_skill_line_names, combinations)
    }

    fn generate_spammable_skills(
        skill_line_combinations: &[Vec<SkillLineName>],
        skills_service: &SkillsService,
        skill_names: &mut HashSet<String>,
    ) -> Vec<Vec<&'static SkillData>> {
        skill_line_combinations
            .iter()
            .map(|skill_line_combination| {
                skill_line_combination
                    .iter()
                    .flat_map(|skill_line| {
                        let skills: Vec<_> = skills_service
                            .get_skills_by_skill_line(*skill_line)
                            .into_iter()
                            .filter(|s| s.spammable && s.bonuses.is_none() && s.execute.is_none())
                            .collect();
                        for skill in &skills {
                            skill_names.insert(skill.name.clone());
                        }
                        skills
                    })
                    .collect()
            })
            .collect()
    }

    fn generate_finisher_skills(
        skill_line_combinations: &[Vec<SkillLineName>],
        skills_service: &SkillsService,
        skill_names: &mut HashSet<String>,
    ) -> Vec<Vec<&'static SkillData>> {
        skill_line_combinations
            .iter()
            .map(|skill_line_combination| {
                skill_line_combination
                    .iter()
                    .flat_map(|skill_line| {
                        let skills: Vec<_> = skills_service
                            .get_skills_by_skill_line(*skill_line)
                            .into_iter()
                            .filter(|s| s.spammable && s.bonuses.is_none() && s.execute.is_some())
                            .collect();
                        for skill in &skills {
                            skill_names.insert(skill.name.clone());
                        }
                        skills
                    })
                    .collect()
            })
            .collect()
    }

    fn generate_non_spammable_skills(
        skill_line_combinations: &[Vec<SkillLineName>],
        skills_service: &SkillsService,
        skill_names: &mut HashSet<String>,
        max_pool_size: Option<usize>,
        character_stats: &CharacterStats,
        verbose: bool,
    ) -> Vec<Vec<&'static SkillData>> {
        skill_line_combinations
            .iter()
            .map(|skill_line_combination| {
                let mut skills: Vec<&'static SkillData> = skill_line_combination
                    .iter()
                    .flat_map(|skill_line| {
                        skills_service
                            .get_skills_by_skill_line(*skill_line)
                            .into_iter()
                            .filter(|s| !(s.spammable && s.bonuses.is_none()))
                    })
                    .collect();

                // Prune dominated skills per skill line before pool capping
                let before = skills.len();
                skills = Self::prune_dominated_skills(skills, character_stats);
                if verbose && skills.len() < before {
                    logger::dim(&format!(
                        "Pruned {} dominated skills ({} → {})",
                        before - skills.len(),
                        before,
                        skills.len()
                    ));
                }

                // Sort by standalone damage (descending) and cap to max_pool_size
                if let Some(cap) = max_pool_size {
                    if skills.len() > cap {
                        skills.sort_by(|a, b| {
                            let da = a.calculate_damage_per_cast(&[], character_stats, None);
                            let db = b.calculate_damage_per_cast(&[], character_stats, None);
                            db.partial_cmp(&da).unwrap_or(std::cmp::Ordering::Equal)
                        });
                        if verbose {
                            logger::dim(&format!(
                                "Capping non-spammable pool from {} to {} skills",
                                skills.len(),
                                cap
                            ));
                        }
                        skills.truncate(cap);
                    }
                }

                for skill in &skills {
                    skill_names.insert(skill.name.clone());
                }
                skills
            })
            .collect()
    }

    /// Signature capturing how a skill interacts with bonus modifiers.
    /// Two skills with the same signature respond to the same bonuses in the
    /// same proportional way, so higher standalone damage ⇒ strict dominance.
    ///
    /// Components: sorted list of `(DamageFlags, is_hit)` tuples.
    /// Sorting by `(flags.bits(), is_hit)` gives a canonical order.
    fn damage_signature(skill: &SkillData) -> Vec<(DamageFlags, bool)> {
        let mut sig = Vec::new();
        if let Some(damage) = &skill.damage {
            if let Some(hits) = &damage.hits {
                for hit in hits {
                    sig.push((hit.flags, true));
                }
            }
            if let Some(dots) = &damage.dots {
                for dot in dots {
                    sig.push((dot.flags, false));
                }
            }
        }
        sig.sort_by_key(|(flags, is_hit)| (flags.bits(), *is_hit));
        sig
    }

    /// Remove skills that are strictly outclassed by another skill from the
    /// same skill line with the same damage signature. Pruning is per skill
    /// line: a skill from line A never prunes a skill from line B.
    ///
    /// Exempt from pruning:
    /// - Skills with bonuses (provide utility beyond raw damage)
    /// - Skills with execute mechanics (relative value varies by enemy HP)
    fn prune_dominated_skills(
        skills: Vec<&'static SkillData>,
        character_stats: &CharacterStats,
    ) -> Vec<&'static SkillData> {
        // Group by (skill_line, damage_signature)
        let mut groups: HashMap<
            (SkillLineName, Vec<(DamageFlags, bool)>),
            Vec<&'static SkillData>,
        > = HashMap::new();
        let mut exempt: Vec<&'static SkillData> = Vec::new();

        for skill in &skills {
            if skill.bonuses.is_some() || skill.execute.is_some() {
                exempt.push(skill);
                continue;
            }
            let sig = Self::damage_signature(skill);
            groups
                .entry((skill.skill_line, sig))
                .or_default()
                .push(skill);
        }

        let mut result = Vec::with_capacity(skills.len());
        result.extend(exempt);

        for (_key, group) in groups {
            if group.len() <= 1 {
                result.extend(group);
                continue;
            }
            // Keep only the skill with the highest standalone damage
            let best = group
                .iter()
                .max_by(|a, b| {
                    let da = a.calculate_damage_per_cast(&[], character_stats, None);
                    let db = b.calculate_damage_per_cast(&[], character_stats, None);
                    da.partial_cmp(&db).unwrap_or(std::cmp::Ordering::Equal)
                })
                .unwrap();
            result.push(best);
        }

        result
    }

    fn generate_passive_bonuses(
        skill_line_combinations: &[Vec<SkillLineName>],
        suppressed_names: &HashSet<String>,
        verbose: bool,
    ) -> (Vec<PreSplitBonuses>, Vec<Vec<BonusData>>) {
        let all_used_skill_lines: HashSet<SkillLineName> =
            skill_line_combinations.iter().flatten().copied().collect();

        let passives_service =
            PassivesService::new(PassivesServiceOptions::default()).with_filter(PassivesFilter {
                skill_lines: Some(all_used_skill_lines),
            });

        let originals: Vec<Vec<BonusData>> = skill_line_combinations
            .iter()
            .map(|skill_lines| {
                skill_lines
                    .iter()
                    .flat_map(|sl| passives_service.get_passives_by_skill_line(*sl))
                    .flat_map(|passive| passive.bonuses.iter().cloned())
                    .filter(|b| !suppressed_names.contains(&b.name))
                    .collect()
            })
            .collect();

        let split: Vec<PreSplitBonuses> = originals
            .iter()
            .map(|all| Self::three_way_split(all.clone()))
            .collect();

        if verbose {
            logger::dim(&format!(
                "Pre-computed passive bonuses for {} skill line combinations",
                split.len()
            ));
        }

        (split, originals)
    }

    fn calculate_total_build_count(
        champion_point_combinations: &[PreSplitBonuses],
        spammable_skills: &[Vec<&'static SkillData>],
        finisher_skills: &[Vec<&'static SkillData>],
        non_spammable_skills: &[Vec<&'static SkillData>],
        required_non_spammable_count: usize,
        has_required_finisher: bool,
    ) -> u64 {
        let base_free_slots = BUILD_CONSTRAINTS.skill_count - 1 - required_non_spammable_count;
        let skill_combinations_count: u64 = spammable_skills
            .iter()
            .zip(finisher_skills.iter())
            .zip(non_spammable_skills.iter())
            .map(|((spammable, finishers), non_spammable)| {
                let mut count = 0u64;
                let spam_count = spammable.len() as u64;
                if has_required_finisher {
                    // Only with-finisher builds (required finisher uses 1 slot)
                    let free = base_free_slots - 1;
                    count += spam_count
                        * combinatorics::count_combinations(non_spammable.len(), free);
                } else {
                    // Without finisher: full free slots
                    count += spam_count
                        * combinatorics::count_combinations(non_spammable.len(), base_free_slots);
                    // With each finisher: 1 fewer free slot
                    if base_free_slots > 0 {
                        let free = base_free_slots - 1;
                        count += spam_count
                            * finishers.len() as u64
                            * combinatorics::count_combinations(non_spammable.len(), free);
                    }
                }
                count
            })
            .sum();

        champion_point_combinations.len() as u64 * skill_combinations_count
    }
}

// Optimize
impl BuildOptimizer {
    pub fn find_optimal_build(&self) -> Vec<Build> {
        logger::log(&format!("Using {} threads...", self.parallelism));

        let start_time = Instant::now();

        let evaluated_count = AtomicU64::new(0);
        let last_progress_update = AtomicU64::new(0);
        let best_damage = AtomicU64::new(0);

        let pool = ThreadPoolBuilder::new()
            .num_threads(self.parallelism as usize)
            .build()
            .expect("Failed to create thread pool");

        // Lightweight candidate — stores indices for final Build reconstruction
        #[derive(Clone)]
        struct Candidate {
            damage: f64,
            skills: SmallVec<[&'static SkillData; 10]>,
            cp_idx: usize,
            sl_idx: usize,
        }

        const TOP_N_CAPACITY: usize = 100;

        struct TopN {
            candidates: Vec<Candidate>,
            capacity: usize,
            min_damage: f64,
        }

        impl TopN {
            fn new(capacity: usize) -> Self {
                Self {
                    candidates: Vec::with_capacity(capacity + 1),
                    capacity,
                    min_damage: f64::NEG_INFINITY,
                }
            }

            fn try_insert(&mut self, candidate: Candidate) {
                if self.candidates.len() >= self.capacity
                    && candidate.damage <= self.min_damage
                {
                    return;
                }
                // Binary search for insertion point (descending order)
                let pos = self
                    .candidates
                    .partition_point(|c| c.damage > candidate.damage);
                self.candidates.insert(pos, candidate);
                if self.candidates.len() > self.capacity {
                    self.candidates.pop();
                }
                if self.candidates.len() >= self.capacity {
                    self.min_damage = self.candidates.last().unwrap().damage;
                }
            }

            fn merge(mut self, other: TopN) -> TopN {
                for c in other.candidates {
                    self.try_insert(c);
                }
                self
            }
        }

        // Pre-collect lightweight work units to avoid par_bridge() mutex contention.
        // Each work unit is (cp_idx, skill_line_idx, spammable_idx) — a few KB total.
        let work_units = self.collect_work_units();

        let best_candidates: Option<TopN> = pool.install(|| {
            work_units
                .par_iter()
                .map(|&(sl_idx, spam_idx, fin_opt)| {
                    let (passive_pre_resolved, passive_ability_count, passive_alt) =
                        &self.passive_bonuses_list[sl_idx];
                    let spammable_skill = self.spammable_skills[sl_idx][spam_idx];
                    let finisher_skill: Option<&'static SkillData> =
                        fin_opt.map(|fi| self.finisher_skills[sl_idx][fi]);
                    let non_spammable = &self.non_spammable_skills[sl_idx];
                    let has_finisher = finisher_skill.is_some();

                    let mut top_n = TopN::new(TOP_N_CAPACITY);

                    // Track progress and update top-N for a single evaluation
                    let mut track = |damage: f64,
                                     combo: &SmallVec<[&'static SkillData; 10]>,
                                     cp_idx: usize| {
                        let count = evaluated_count.fetch_add(1, Ordering::Relaxed) + 1;
                        top_n.try_insert(Candidate {
                            damage,
                            skills: combo.clone(),
                            cp_idx,
                            sl_idx,
                        });
                        let _ = best_damage.fetch_max(damage.to_bits(), Ordering::Relaxed);
                        if count % 1_000_000 == 0 {
                            let last = last_progress_update.swap(count, Ordering::Relaxed);
                            if count > last {
                                let progress = (count as f64
                                    / self.total_possible_build_count as f64)
                                    * 100.0;
                                let elapsed = start_time.elapsed().as_secs_f64();
                                let eta = if progress > 0.0 {
                                    elapsed * (100.0 - progress) / progress
                                } else {
                                    0.0
                                };
                                let best =
                                    f64::from_bits(best_damage.load(Ordering::Relaxed));
                                logger::progress(&format!(
                                    "Progress: {:.1}% ({}) | Best: {} | ETA: {}",
                                    progress,
                                    format::format_number(count),
                                    format::format_number(best as u64),
                                    format::format_duration((eta * 1000.0) as u64)
                                ));
                            }
                        }
                    };

                    let req_count = self.required_non_spammable.len();
                    let non_spam_count = BUILD_CONSTRAINTS.skill_count
                        - 1
                        - has_finisher as usize;
                    let free_slots = non_spam_count - req_count;

                    if self.champion_point_combinations.len() > 1 {
                        // Multiple CP combos: incremental evaluation with cached passive context
                        let num_cp = self.champion_point_combinations.len();

                        // Passive lookup is constant for this work unit
                        let (passive_lookup, passive_filtered) =
                            Build::compute_passive_lookup(
                                passive_pre_resolved,
                                passive_ability_count,
                            );

                        // Incremental state
                        let mut passive_ctx = None;
                        let mut cp_ctxs = Vec::with_capacity(num_cp);
                        let mut cp_raw_totals = vec![0.0f64; num_cp];
                        let mut cp_skill_damages = vec![[0.0f64; 10]; num_cp];
                        let mut prev_combo: SmallVec<[&'static SkillData; 10]> =
                            SmallVec::new();

                        let mut combo_iter = combinatorics::CombinationIterator::new(
                            non_spammable,
                            free_slots,
                        );

                        while let Some(variable) = combo_iter.next() {
                            // Build full combo: [required..., variable..., finisher?, spammable]
                            let mut combo: SmallVec<[&'static SkillData; 10]> =
                                SmallVec::new();
                            combo.extend_from_slice(&self.required_non_spammable);
                            combo.extend_from_slice(&variable);
                            if let Some(fin) = finisher_skill {
                                combo.push(fin);
                            }
                            combo.push(spammable_skill);

                            let first_changed =
                                req_count + combo_iter.first_changed();

                            let can_incr = passive_ctx.is_some()
                                && (first_changed..non_spam_count).all(|i| {
                                    combo[i].skill_line == prev_combo[i].skill_line
                                });

                            if can_incr {
                                let pctx = passive_ctx.as_mut().unwrap();

                                // Update passive mods only for changed positions
                                for i in first_changed..non_spam_count {
                                    Build::update_passive_mod(
                                        combo[i],
                                        i,
                                        &passive_lookup,
                                        &passive_filtered,
                                        pctx,
                                    );
                                }

                                // Update per-CP-combo damages for changed positions
                                for cp_idx in 0..num_cp {
                                    for i in first_changed..non_spam_count {
                                        cp_raw_totals[cp_idx] -=
                                            cp_skill_damages[cp_idx][i];
                                        cp_skill_damages[cp_idx][i] =
                                            Build::single_skill_damage_cached(
                                                combo[i],
                                                i,
                                                pctx,
                                                &cp_ctxs[cp_idx],
                                            );
                                        cp_raw_totals[cp_idx] +=
                                            cp_skill_damages[cp_idx][i];
                                    }
                                    let damage = cp_raw_totals[cp_idx]
                                        * cp_ctxs[cp_idx].armor_factor
                                        * cp_ctxs[cp_idx].crit_mult;
                                    track(damage, &combo, cp_idx);
                                }
                            } else {
                                // Full recompute
                                let pctx = Build::cache_passive_context(
                                    &combo,
                                    passive_pre_resolved,
                                    passive_ability_count,
                                    &self.character_stats,
                                );

                                cp_ctxs.clear();
                                for (
                                    cp_idx,
                                    (cp_pre_resolved, cp_ability_count, cp_alt),
                                ) in self
                                    .champion_point_combinations
                                    .iter()
                                    .enumerate()
                                {
                                    let cp_ctx =
                                        Build::compute_cp_eval_context(
                                            &combo,
                                            &pctx,
                                            passive_alt,
                                            cp_pre_resolved,
                                            cp_ability_count,
                                            cp_alt,
                                        );
                                    let mut raw = 0.0;
                                    for (i, skill) in combo.iter().enumerate() {
                                        let d =
                                            Build::single_skill_damage_cached(
                                                skill, i, &pctx, &cp_ctx,
                                            );
                                        cp_skill_damages[cp_idx][i] = d;
                                        raw += d;
                                    }
                                    cp_raw_totals[cp_idx] = raw;
                                    let damage =
                                        raw * cp_ctx.armor_factor * cp_ctx.crit_mult;
                                    track(damage, &combo, cp_idx);
                                    cp_ctxs.push(cp_ctx);
                                }

                                passive_ctx = Some(pctx);
                            }

                            prev_combo = combo;
                        }
                    } else {
                        // Single CP combo: direct path with incremental evaluation
                        let (cp_pre_resolved, cp_ability_count, cp_alt) =
                            &self.champion_point_combinations[0];

                        // Incremental state
                        let mut eval_ctx = None;
                        let mut per_skill_damages = [0.0f64; 10];
                        let mut raw_total = 0.0f64;
                        let mut prev_combo: SmallVec<[&'static SkillData; 10]> =
                            SmallVec::new();

                        let mut combo_iter = combinatorics::CombinationIterator::new(
                            non_spammable,
                            free_slots,
                        );

                        while let Some(variable) = combo_iter.next() {
                            // Build full combo: [required..., variable..., finisher?, spammable]
                            let mut combo: SmallVec<[&'static SkillData; 10]> =
                                SmallVec::new();
                            combo.extend_from_slice(&self.required_non_spammable);
                            combo.extend_from_slice(&variable);
                            if let Some(fin) = finisher_skill {
                                combo.push(fin);
                            }
                            combo.push(spammable_skill);

                            let first_changed =
                                req_count + combo_iter.first_changed();

                            let can_incr = eval_ctx.is_some()
                                && (first_changed..non_spam_count).all(|i| {
                                    combo[i].skill_line == prev_combo[i].skill_line
                                });

                            if can_incr {
                                let ctx = eval_ctx.as_ref().unwrap();
                                for i in first_changed..non_spam_count {
                                    raw_total -= per_skill_damages[i];
                                    per_skill_damages[i] =
                                        Build::single_skill_damage(combo[i], ctx);
                                    raw_total += per_skill_damages[i];
                                }
                                let damage =
                                    raw_total * ctx.armor_factor * ctx.crit_mult;
                                track(damage, &combo, 0);
                            } else {
                                let ctx = Build::compute_eval_context(
                                    &combo,
                                    cp_pre_resolved,
                                    cp_ability_count,
                                    cp_alt,
                                    passive_pre_resolved,
                                    passive_ability_count,
                                    passive_alt,
                                    &self.character_stats,
                                );
                                raw_total = 0.0;
                                for (i, skill) in combo.iter().enumerate() {
                                    per_skill_damages[i] =
                                        Build::single_skill_damage(skill, &ctx);
                                    raw_total += per_skill_damages[i];
                                }
                                let damage =
                                    raw_total * ctx.armor_factor * ctx.crit_mult;
                                track(damage, &combo, 0);
                                eval_ctx = Some(ctx);
                            }

                            prev_combo = combo;
                        }
                    }

                    top_n
                })
                .reduce_with(|a, b| a.merge(b))
        });

        let total_evaluated = evaluated_count.load(Ordering::Relaxed);
        let elapsed = start_time.elapsed();

        logger::log(&format!(
            "Completed: {} builds evaluated in {:.1}s",
            format::format_number(total_evaluated),
            elapsed.as_secs_f64()
        ));

        // Construct full Builds for the top-N candidates using original BonusData
        match best_candidates {
            Some(top) => top
                .candidates
                .into_iter()
                .map(|c| {
                    let cp_bonuses = &self.champion_point_original[c.cp_idx];
                    let passive_bonuses = &self.passive_original[c.sl_idx];

                    Build::new_with_extra(
                        c.skills.to_vec(),
                        cp_bonuses,
                        passive_bonuses,
                        &self.set_bonuses,
                        self.set_names.clone(),
                        self.character_stats.clone(),
                        &self.extra_bonuses,
                    )
                })
                .collect(),
            None => Vec::new(),
        }
    }

    /// Pre-collect lightweight work unit indices: (skill_line_idx, spammable_idx, Option<finisher_idx>).
    /// Skills are outermost; each work unit iterates all CP combos internally.
    fn collect_work_units(&self) -> Vec<(usize, usize, Option<usize>)> {
        let mut units = Vec::new();
        for sl_idx in 0..self.spammable_skills.len() {
            for spam_idx in 0..self.spammable_skills[sl_idx].len() {
                if self.required_finisher.is_some() {
                    // Required finisher: only generate with-finisher work units
                    units.push((sl_idx, spam_idx, Some(0)));
                } else {
                    // No finisher: 9 non-spammable slots
                    units.push((sl_idx, spam_idx, None));
                    // With each finisher: 8 non-spammable slots
                    for fin_idx in 0..self.finisher_skills[sl_idx].len() {
                        units.push((sl_idx, spam_idx, Some(fin_idx)));
                    }
                }
            }
        }
        units
    }
}

// Format
impl BuildOptimizer {
    fn fmt_configuration_table(&self) -> String {
        let config_data = vec![
            vec![
                "Skills".to_string(),
                BUILD_CONSTRAINTS.skill_count.to_string(),
            ],
            vec![
                "Champion Points".to_string(),
                BUILD_CONSTRAINTS.champion_point_count.to_string(),
            ],
            vec![
                "Class Skill Lines".to_string(),
                BUILD_CONSTRAINTS.class_skill_line_count.to_string(),
            ],
            vec![
                "Weapon Skill Lines".to_string(),
                BUILD_CONSTRAINTS.weapon_skill_line_count.to_string(),
            ],
            vec!["Workers".to_string(), self.parallelism.to_string()],
        ];

        table::table(
            &config_data,
            table::TableOptions {
                title: Some("Configuration".to_string()),
                columns: vec![
                    table::ColumnDefinition::new("Constraint", 25),
                    table::ColumnDefinition::new("Value", 10).align_right(),
                ],
                footer: None,
            },
        )
    }

    fn fmt_sorted_list<T: ToString>(items: impl IntoIterator<Item = T>) -> String {
        let mut sorted: Vec<_> = items.into_iter().map(|i| i.to_string()).collect();
        if sorted.is_empty() {
            "None".to_string()
        } else {
            sorted.sort();
            sorted.join(", ")
        }
    }

    fn fmt_skill_line_configuration_table(&self) -> String {
        let used_classes_str = Self::fmt_sorted_list(self.class_names.iter());
        let required_classes_str = Self::fmt_sorted_list(self.required_class_names.iter());
        let used_weapons_str = Self::fmt_sorted_list(self.weapon_skill_line_names.iter());
        let required_weapons_str = Self::fmt_sorted_list(self.required_weapon_skill_lines.iter());
        let used_cp_str = Self::fmt_sorted_list(self.champion_point_names.iter());
        let required_cp_str = Self::fmt_sorted_list(self.required_champion_points.iter());
        let required_skills_str = Self::fmt_sorted_list(self.required_skill_names.iter());

        let name_width = [
            &used_classes_str,
            &required_classes_str,
            &used_weapons_str,
            &required_weapons_str,
            &used_cp_str,
            &required_cp_str,
            &required_skills_str,
        ]
        .iter()
        .map(|s| s.len())
        .max()
        .unwrap_or(20)
        .max(20);

        let skill_config_data = vec![
            vec!["Used Classes".to_string(), used_classes_str],
            vec!["Required Classes".to_string(), required_classes_str],
            vec!["Used Weapons".to_string(), used_weapons_str],
            vec!["Required Weapons".to_string(), required_weapons_str],
            vec!["Used Champion Points".to_string(), used_cp_str],
            vec!["Required Champion Points".to_string(), required_cp_str],
            vec!["Required Skills".to_string(), required_skills_str],
            vec![
                "Used Skills".to_string(),
                self.skill_names.len().to_string(),
            ],
        ];

        table::table(
            &skill_config_data,
            table::TableOptions {
                title: Some("Skill Line Configuration".to_string()),
                columns: vec![
                    table::ColumnDefinition::new("Setting", 25),
                    table::ColumnDefinition::new("Name", name_width),
                ],
                footer: None,
            },
        )
    }
}

impl std::fmt::Display for BuildOptimizer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut lines = Vec::new();
        lines.push(self.fmt_configuration_table());
        lines.push(self.fmt_skill_line_configuration_table());
        lines.push(format!(
            "Total builds to be evaluated: {}",
            format::format_number(self.total_possible_build_count)
        ));
        write!(f, "{}", lines.join("\n"))
    }
}
