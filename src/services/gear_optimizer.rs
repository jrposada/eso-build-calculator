use crate::domain::{
    ArmorTrait, AttributeChoice, Build, CharacterStats, Food, GearConfig, JewelryTrait,
    MundusStone, Race, WeaponTrait, WeaponType, DPS_ARMOR_TRAITS, DPS_ATTRIBUTES, DPS_FOODS,
    DPS_JEWELRY_TRAITS, DPS_MUNDUS_STONES, DPS_RACES, DPS_WEAPON_TRAITS,
};
use crate::infrastructure::logger;

pub struct GearOptimizerOptions {
    pub pinned_race: Option<Race>,
    pub pinned_mundus: Option<MundusStone>,
    pub pinned_food: Option<Food>,
    pub pinned_armor_trait: Option<ArmorTrait>,
    pub pinned_jewelry_trait: Option<JewelryTrait>,
    pub pinned_weapon_trait: Option<WeaponTrait>,
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
            && self.pinned_armor_trait.is_some()
            && self.pinned_jewelry_trait.is_some()
            && self.pinned_weapon_trait.is_some()
            && self.pinned_attributes.is_some()
    }
}

pub struct GearOptimizerResult {
    pub gear_config: GearConfig,
    pub character_stats: CharacterStats,
}

pub struct GearOptimizer;

impl GearOptimizer {
    /// Two-phase gear optimization:
    /// Phase 1A: Greedy — score coupled/independent dimensions keeping others at baseline.
    /// Phase 1B: Refine — cross-product top-K from each dimension group.
    pub fn optimize(
        builds: &[Build],
        options: &GearOptimizerOptions,
        baseline: &GearConfig,
    ) -> GearOptimizerResult {
        let rep = &builds[0];
        let top_k = options.top_k;

        // Helper: evaluate a GearConfig by building a new Build and returning DPC
        let score = |gear: &GearConfig| -> f64 {
            let stats = gear.compute_stats(options.bar1_weapon);
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

        // Coupled group 1: (ArmorTrait, Mundus) — Divines amplifies Mundus
        let armor_candidates: Vec<ArmorTrait> = match options.pinned_armor_trait {
            Some(t) => vec![t],
            None => DPS_ARMOR_TRAITS.to_vec(),
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

        let mut armor_mundus_scores: Vec<(f64, ArmorTrait, Option<MundusStone>)> = Vec::new();
        for &armor in &armor_candidates {
            for &mundus in &mundus_candidates {
                let mut gear = baseline.clone();
                gear.armor_trait = armor;
                gear.mundus = mundus;
                let dpc = score(&gear);
                armor_mundus_scores.push((dpc, armor, mundus));
            }
        }
        armor_mundus_scores.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
        armor_mundus_scores.truncate(top_k);

        if options.verbose {
            logger::dim(&format!(
                "Gear Phase 1A: Scored {} armor+mundus combos, top-{}: {}",
                armor_candidates.len() * mundus_candidates.len(),
                top_k,
                armor_mundus_scores
                    .iter()
                    .map(|(dpc, a, m)| format!(
                        "{}+{} ({:.0})",
                        a,
                        m.map_or("None".to_string(), |m| m.to_string()),
                        dpc
                    ))
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
        }

        // Coupled group 2: (Attributes, Food) — both affect resource pools
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

        // Independent: JewelryTrait
        let jewelry_candidates: Vec<JewelryTrait> = match options.pinned_jewelry_trait {
            Some(t) => vec![t],
            None => DPS_JEWELRY_TRAITS.to_vec(),
        };

        let mut jewelry_scores: Vec<(f64, JewelryTrait)> = Vec::new();
        for &jt in &jewelry_candidates {
            let mut gear = baseline.clone();
            gear.jewelry_trait = jt;
            let dpc = score(&gear);
            jewelry_scores.push((dpc, jt));
        }
        jewelry_scores.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
        jewelry_scores.truncate(top_k);

        if options.verbose {
            logger::dim(&format!(
                "Gear Phase 1A: Scored {} jewelry traits, top-{}: {}",
                jewelry_candidates.len(),
                top_k,
                jewelry_scores
                    .iter()
                    .map(|(dpc, j)| format!("{} ({:.0})", j, dpc))
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
        }

        // Independent: WeaponTrait
        let weapon_candidates: Vec<WeaponTrait> = match options.pinned_weapon_trait {
            Some(t) => vec![t],
            None => DPS_WEAPON_TRAITS.to_vec(),
        };

        let mut weapon_scores: Vec<(f64, WeaponTrait)> = Vec::new();
        for &wt in &weapon_candidates {
            let mut gear = baseline.clone();
            gear.weapon_trait = wt;
            let dpc = score(&gear);
            weapon_scores.push((dpc, wt));
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
                    .map(|(dpc, w)| format!("{} ({:.0})", w, dpc))
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
                            let gear = GearConfig {
                                race,
                                mundus,
                                food,
                                armor_trait: armor,
                                jewelry_trait: jewelry,
                                weapon_trait: weapon,
                                attributes: attr,
                                armor_weight: baseline.armor_weight,
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

        let best_stats = best_gear.compute_stats(options.bar1_weapon);

        if options.verbose {
            logger::dim(&format!(
                "Gear optimization result: DPC={:.0}, Race={}, Mundus={}, Food={}, Armor={}, Jewelry={}, Weapon={}, Attributes={}",
                best_dpc,
                best_gear.race.map_or("None".to_string(), |r| r.to_string()),
                best_gear.mundus.map_or("None".to_string(), |m| m.to_string()),
                best_gear.food.map_or("None".to_string(), |f| f.to_string()),
                best_gear.armor_trait,
                best_gear.jewelry_trait,
                best_gear.weapon_trait,
                best_gear.attributes,
            ));
        }

        GearOptimizerResult {
            gear_config: best_gear,
            character_stats: best_stats,
        }
    }
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

    check(a.max_magicka.max(a.max_stamina), b.max_magicka.max(b.max_stamina))
        || check(
            a.weapon_damage.max(a.spell_damage),
            b.weapon_damage.max(b.spell_damage),
        )
        || check(a.critical_rating, b.critical_rating)
        || check(a.critical_damage, b.critical_damage)
        || check(a.penetration, b.penetration)
}
