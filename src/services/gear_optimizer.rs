use crate::domain::{
    ArmorTrait, AttributeChoice, Build, BuildConfig, CharacterStats, Food, JewelryTrait,
    MundusStone, Race, WeaponTrait, WeaponType, DPS_ARMOR_TRAITS, DPS_ATTRIBUTES, DPS_FOODS,
    DPS_JEWELRY_TRAITS, DPS_MUNDUS_STONES, DPS_RACES, DPS_WEAPON_TRAITS,
};
use crate::infrastructure::logger;

pub struct GearOptimizerOptions {
    pub pinned_race: Option<Race>,
    pub pinned_mundus: Option<MundusStone>,
    pub pinned_food: Option<Food>,
    /// Per-slot pinned armor traits. len 0..=7; pins first N slots, rest optimized.
    pub pinned_armor_traits: Vec<ArmorTrait>,
    /// Per-slot pinned jewelry traits. len 0..=3; pins first N slots, rest optimized.
    pub pinned_jewelry_traits: Vec<JewelryTrait>,
    /// Per-slot pinned weapon traits. len 0..=2; index 0=bar1, 1=bar2.
    pub pinned_weapon_traits: Vec<WeaponTrait>,
    pub pinned_attributes: Option<AttributeChoice>,
    pub bar1_weapon: Option<WeaponType>,
    pub top_k: usize,
    pub verbose: bool,
}

impl GearOptimizerOptions {
    /// Returns true if all gear dimensions are pinned (nothing to optimize).
    pub fn all_pinned(&self) -> bool {
        self.pinned_race.is_some()
            && self.pinned_mundus.is_some()
            && self.pinned_food.is_some()
            && self.pinned_armor_traits.len() == 7
            && self.pinned_jewelry_traits.len() == 3
            && self.pinned_weapon_traits.len() >= 1
            && self.pinned_attributes.is_some()
    }
}

pub struct GearOptimizerResult {
    pub build_config: BuildConfig,
    pub character_stats: CharacterStats,
}

pub struct GearOptimizer;

impl GearOptimizer {
    /// Two-phase gear optimization:
    /// Phase 1A: Greedy - score coupled/independent dimensions keeping others at baseline.
    /// Phase 1B: Refine - cross-product top-K from each dimension group.
    pub fn optimize(
        builds: &[Build],
        options: &GearOptimizerOptions,
        baseline: &BuildConfig,
    ) -> GearOptimizerResult {
        let rep = &builds[0];
        let top_k = options.top_k;

        // Helper: evaluate a BuildConfig by building a new Build and returning DPC
        let score = |gear: &BuildConfig| -> f64 {
            let stats = gear.compute_stats();
            let build = Build::new_with_extra(
                rep.skills().to_vec(),
                rep.cp_bonuses(),
                rep.passive_bonuses(),
                &[], // no set bonuses during gear optimization
                Vec::new(),
                stats,
                rep.extra_bonuses(),
            );
            build.total_damage_per_cast
        };

        // ── Phase 1A: Greedy scoring ──

        // Coupled group 1: (ArmorTraits, Mundus) - Divines amplifies Mundus
        // Generate all armor trait array combinations for free slots (not pinned)
        let pinned_armor = &options.pinned_armor_traits;
        let free_armor_slots = 7 - pinned_armor.len();
        let armor_trait_arrays: Vec<[ArmorTrait; 7]> = if free_armor_slots == 0 {
            // All 7 slots pinned
            let mut arr = [ArmorTrait::Divines; 7];
            for (i, t) in pinned_armor.iter().enumerate() {
                arr[i] = *t;
            }
            vec![arr]
        } else {
            // Generate all DPS trait combos for free slots
            let mut combos: Vec<[ArmorTrait; 7]> = Vec::new();
            let free_count = free_armor_slots;
            let trait_count = DPS_ARMOR_TRAITS.len();
            let total = trait_count.pow(free_count as u32);
            for i in 0..total {
                let mut arr = [ArmorTrait::Divines; 7];
                // Fill pinned prefix
                for (j, t) in pinned_armor.iter().enumerate() {
                    arr[j] = *t;
                }
                // Fill free slots
                let mut idx = i;
                for slot in pinned_armor.len()..7 {
                    arr[slot] = DPS_ARMOR_TRAITS[idx % trait_count];
                    idx /= trait_count;
                }
                combos.push(arr);
            }
            combos
        };

        let mundus_candidates: Vec<Option<MundusStone>> = match options.pinned_mundus {
            Some(m) => vec![Some(m)],
            None => {
                let mut v: Vec<Option<MundusStone>> =
                    DPS_MUNDUS_STONES.iter().copied().map(Some).collect();
                v.push(None); // no mundus is also a candidate
                v
            }
        };

        let mut armor_mundus_scores: Vec<(f64, [ArmorTrait; 7], Option<MundusStone>)> = Vec::new();
        for armor in &armor_trait_arrays {
            for &mundus in &mundus_candidates {
                let mut gear = baseline.clone();
                gear.armor_traits = *armor;
                gear.mundus = mundus;
                let dpc = score(&gear);
                armor_mundus_scores.push((dpc, *armor, mundus));
            }
        }
        armor_mundus_scores.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
        armor_mundus_scores.truncate(top_k);

        if options.verbose {
            logger::dim(&format!(
                "Gear Phase 1A: Scored {} armor+mundus combos, top-{}: {}",
                armor_trait_arrays.len() * mundus_candidates.len(),
                top_k,
                armor_mundus_scores
                    .iter()
                    .map(|(dpc, a, m)| format!(
                        "{}+{} ({:.0})",
                        format_armor_traits(a),
                        m.map_or("None".to_string(), |m| m.to_string()),
                        dpc
                    ))
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
        }

        // Coupled group 2: (Attributes, Food) - both affect resource pools
        let attr_candidates: Vec<AttributeChoice> = match options.pinned_attributes {
            Some(a) => vec![a],
            None => DPS_ATTRIBUTES.to_vec(),
        };
        let food_candidates: Vec<Option<Food>> = match options.pinned_food {
            Some(f) => vec![Some(f)],
            None => {
                let mut v: Vec<Option<Food>> = DPS_FOODS.iter().copied().map(Some).collect();
                v.push(None);
                v
            }
        };

        let mut attr_food_scores: Vec<(f64, AttributeChoice, Option<Food>)> = Vec::new();
        for &attr in &attr_candidates {
            for &food in &food_candidates {
                let mut gear = baseline.clone();
                gear.attributes = attr;
                gear.food = food;
                let dpc = score(&gear);
                attr_food_scores.push((dpc, attr, food));
            }
        }
        attr_food_scores.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
        attr_food_scores.truncate(top_k);

        if options.verbose {
            logger::dim(&format!(
                "Gear Phase 1A: Scored {} attr+food combos, top-{}: {}",
                attr_candidates.len() * food_candidates.len(),
                top_k,
                attr_food_scores
                    .iter()
                    .map(|(dpc, a, f)| format!(
                        "{}+{} ({:.0})",
                        a,
                        f.map_or("None".to_string(), |f| f.to_string()),
                        dpc
                    ))
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
        }

        // Independent: Race
        let race_candidates: Vec<Option<Race>> = match options.pinned_race {
            Some(r) => vec![Some(r)],
            None => {
                let mut v: Vec<Option<Race>> = DPS_RACES.iter().copied().map(Some).collect();
                v.push(None);
                v
            }
        };

        let mut race_scores: Vec<(f64, Option<Race>)> = Vec::new();
        for &race in &race_candidates {
            let mut gear = baseline.clone();
            gear.race = race;
            let dpc = score(&gear);
            race_scores.push((dpc, race));
        }
        race_scores.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
        race_scores.truncate(top_k);

        if options.verbose {
            logger::dim(&format!(
                "Gear Phase 1A: Scored {} races, top-{}: {}",
                race_candidates.len(),
                top_k,
                race_scores
                    .iter()
                    .map(|(dpc, r)| format!(
                        "{} ({:.0})",
                        r.map_or("None".to_string(), |r| r.to_string()),
                        dpc
                    ))
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
        }

        // Independent: JewelryTraits (per-slot)
        let pinned_jewelry = &options.pinned_jewelry_traits;
        let free_jewelry_slots = 3 - pinned_jewelry.len();
        let jewelry_trait_arrays: Vec<[JewelryTrait; 3]> = if free_jewelry_slots == 0 {
            let mut arr = [JewelryTrait::Bloodthirsty; 3];
            for (i, t) in pinned_jewelry.iter().enumerate() {
                arr[i] = *t;
            }
            vec![arr]
        } else {
            let mut combos: Vec<[JewelryTrait; 3]> = Vec::new();
            let trait_count = DPS_JEWELRY_TRAITS.len();
            let total = trait_count.pow(free_jewelry_slots as u32);
            for i in 0..total {
                let mut arr = [JewelryTrait::Bloodthirsty; 3];
                for (j, t) in pinned_jewelry.iter().enumerate() {
                    arr[j] = *t;
                }
                let mut idx = i;
                for slot in pinned_jewelry.len()..3 {
                    arr[slot] = DPS_JEWELRY_TRAITS[idx % trait_count];
                    idx /= trait_count;
                }
                combos.push(arr);
            }
            combos
        };

        let mut jewelry_scores: Vec<(f64, [JewelryTrait; 3])> = Vec::new();
        for jewelry in &jewelry_trait_arrays {
            let mut gear = baseline.clone();
            gear.jewelry_traits = *jewelry;
            let dpc = score(&gear);
            jewelry_scores.push((dpc, *jewelry));
        }
        jewelry_scores.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
        jewelry_scores.truncate(top_k);

        if options.verbose {
            logger::dim(&format!(
                "Gear Phase 1A: Scored {} jewelry trait combos, top-{}: {}",
                jewelry_trait_arrays.len(),
                top_k,
                jewelry_scores
                    .iter()
                    .map(|(dpc, j)| format!("{} ({:.0})", format_jewelry_traits(j), dpc))
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
        }

        // Independent: WeaponTraits (per-slot, but only bar1 affects DPS calc)
        let pinned_weapon = &options.pinned_weapon_traits;
        let weapon_candidates: Vec<[WeaponTrait; 2]> = if !pinned_weapon.is_empty() {
            // At least bar1 is pinned
            let bar1 = pinned_weapon[0];
            let bar2 = pinned_weapon.get(1).copied().unwrap_or(bar1);
            vec![[bar1, bar2]]
        } else {
            DPS_WEAPON_TRAITS
                .iter()
                .map(|&t| [t, t])
                .collect()
        };

        let mut weapon_scores: Vec<(f64, [WeaponTrait; 2])> = Vec::new();
        for wt in &weapon_candidates {
            let mut gear = baseline.clone();
            gear.weapon_traits = *wt;
            let dpc = score(&gear);
            weapon_scores.push((dpc, *wt));
        }
        weapon_scores.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
        weapon_scores.truncate(top_k);

        if options.verbose {
            logger::dim(&format!(
                "Gear Phase 1A: Scored {} weapon traits, top-{}: {}",
                weapon_candidates.len(),
                top_k,
                weapon_scores
                    .iter()
                    .map(|(dpc, w)| format!("{} ({:.0})", w[0], dpc))
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
        }

        // ── Phase 1B: Cross-product refinement ──
        let total_combos = armor_mundus_scores.len()
            * attr_food_scores.len()
            * race_scores.len()
            * jewelry_scores.len()
            * weapon_scores.len();

        if options.verbose {
            logger::dim(&format!(
                "Gear Phase 1B: Cross-product {} combos ({}x{}x{}x{}x{})",
                total_combos,
                armor_mundus_scores.len(),
                attr_food_scores.len(),
                race_scores.len(),
                jewelry_scores.len(),
                weapon_scores.len(),
            ));
        }

        let mut best_dpc = f64::NEG_INFINITY;
        let mut best_gear = baseline.clone();

        for &(_, armor, mundus) in &armor_mundus_scores {
            for &(_, attr, food) in &attr_food_scores {
                for &(_, race) in &race_scores {
                    for &(_, jewelry) in &jewelry_scores {
                        for &(_, weapon) in &weapon_scores {
                            let gear = BuildConfig {
                                race,
                                mundus,
                                food,
                                armor_traits: armor,
                                jewelry_traits: jewelry,
                                weapon_traits: weapon,
                                attributes: attr,
                                armor: baseline.armor,
                                bar1_weapon: baseline.bar1_weapon,
                                ..baseline.clone()
                            };
                            let dpc = score(&gear);
                            if dpc > best_dpc {
                                best_dpc = dpc;
                                best_gear = gear;
                            }
                        }
                    }
                }
            }
        }

        let best_stats = best_gear.compute_stats();

        if options.verbose {
            logger::dim(&format!(
                "Gear optimization result: DPC={:.0}, Race={}, Mundus={}, Food={}, Armor={}, Jewelry={}, Weapon={}, Attributes={}",
                best_dpc,
                best_gear.race.map_or("None".to_string(), |r| r.to_string()),
                best_gear.mundus.map_or("None".to_string(), |m| m.to_string()),
                best_gear.food.map_or("None".to_string(), |f| f.to_string()),
                format_armor_traits(&best_gear.armor_traits),
                format_jewelry_traits(&best_gear.jewelry_traits),
                best_gear.weapon_traits[0],
                best_gear.attributes,
            ));
        }

        GearOptimizerResult {
            build_config: best_gear,
            character_stats: best_stats,
        }
    }
}

/// Format armor trait array as compact string like "5×Divines,2×Infused".
pub fn format_armor_traits(traits: &[ArmorTrait; 7]) -> String {
    format_trait_counts(traits)
}

/// Format jewelry trait array as compact string like "2×Bloodthirsty,1×Infused".
pub fn format_jewelry_traits(traits: &[JewelryTrait; 3]) -> String {
    format_trait_counts(traits)
}

/// Format weapon trait array as compact string.
pub fn format_weapon_traits(traits: &[WeaponTrait; 2]) -> String {
    format_trait_counts(traits)
}

fn format_trait_counts<T: std::fmt::Display + Eq + std::hash::Hash + Copy>(traits: &[T]) -> String {
    // Preserve order of first appearance
    let mut seen: Vec<T> = Vec::new();
    let mut counts: std::collections::HashMap<&T, usize> = std::collections::HashMap::new();
    for t in traits {
        *counts.entry(t).or_insert(0) += 1;
        if !seen.contains(t) {
            seen.push(*t);
        }
    }
    seen.iter()
        .map(|t| {
            let c = counts[t];
            if c == 1 {
                t.to_string()
            } else {
                format!("{}×{}", c, t)
            }
        })
        .collect::<Vec<_>>()
        .join(",")
}

/// Check if character stats differ by more than a threshold percentage on any key stat.
pub fn stats_differ_significantly(a: &CharacterStats, b: &CharacterStats, threshold: f64) -> bool {
    let check = |va: f64, vb: f64| -> bool {
        if va == 0.0 && vb == 0.0 {
            return false;
        }
        let max = va.abs().max(vb.abs());
        if max == 0.0 {
            return false;
        }
        (va - vb).abs() / max > threshold
    };

    check(
        a.max_magicka.max(a.max_stamina),
        b.max_magicka.max(b.max_stamina),
    ) || check(
        a.weapon_damage.max(a.spell_damage),
        b.weapon_damage.max(b.spell_damage),
    ) || check(a.critical_rating, b.critical_rating)
        || check(a.critical_damage, b.critical_damage)
        || check(a.penetration, b.penetration)
}
