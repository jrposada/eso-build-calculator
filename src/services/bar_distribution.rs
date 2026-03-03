use crate::domain::{SkillData, SkillLineName, WeaponType};

#[derive(Debug, Clone)]
pub struct WeaponBar {
    pub weapon_type: WeaponType,
    pub skills: Vec<&'static SkillData>,
}

#[derive(Debug, Clone)]
pub struct BarDistribution {
    pub bar1: WeaponBar,
    pub bar2: WeaponBar,
}

/// Infer weapon types from the weapon skill lines present in the build.
/// Returns (bar1_weapon, bar2_weapon).
pub fn infer_weapons(skills: &[&'static SkillData]) -> Result<(WeaponType, WeaponType), String> {
    let weapon_lines: Vec<SkillLineName> = skills
        .iter()
        .map(|s| s.skill_line)
        .filter(|sl| sl.is_weapon())
        .collect::<std::collections::HashSet<_>>()
        .into_iter()
        .collect();

    match weapon_lines.len() {
        0 => Err(
            "No weapon skills found. Use --bar1-weapon and --bar2-weapon to specify weapons."
                .to_string(),
        ),
        1 => {
            let wt = weapon_lines[0]
                .default_weapon_type()
                .ok_or("Could not determine weapon type")?;
            Ok((wt, wt))
        }
        2 => {
            let wt1 = weapon_lines[0]
                .default_weapon_type()
                .ok_or("Could not determine weapon type for first weapon line")?;
            let wt2 = weapon_lines[1]
                .default_weapon_type()
                .ok_or("Could not determine weapon type for second weapon line")?;
            Ok((wt1, wt2))
        }
        n => Err(format!(
            "Found {} weapon skill lines ({}). Maximum 2 supported. \
             Use --bar1-weapon and --bar2-weapon to specify explicitly.",
            n,
            weapon_lines
                .iter()
                .map(|sl| sl.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        )),
    }
}

/// Generate all valid bar distributions for the given skills and weapon types.
/// Each bar has exactly 5 skills. Weapon-specific skills are forced to their bar,
/// class/guild skills are flexible and can go on either bar.
pub fn generate_distributions(
    skills: &[&'static SkillData],
    bar1_weapon: WeaponType,
    bar2_weapon: WeaponType,
) -> Vec<BarDistribution> {
    let bar1_skill_line = bar1_weapon.skill_line();
    let bar2_skill_line = bar2_weapon.skill_line();

    let mut bar1_forced: Vec<&'static SkillData> = Vec::new();
    let mut bar2_forced: Vec<&'static SkillData> = Vec::new();
    let mut flexible: Vec<&'static SkillData> = Vec::new();

    for &skill in skills {
        if skill.skill_line == bar1_skill_line && bar1_skill_line != bar2_skill_line {
            bar1_forced.push(skill);
        } else if skill.skill_line == bar2_skill_line && bar1_skill_line != bar2_skill_line {
            bar2_forced.push(skill);
        } else if skill.skill_line.is_weapon() && bar1_skill_line == bar2_skill_line {
            // Same weapon both bars - weapon skills are flexible
            flexible.push(skill);
        } else {
            // Class/guild skills are flexible
            flexible.push(skill);
        }
    }

    let bar1_slots_needed = 5_usize.saturating_sub(bar1_forced.len());

    if bar1_slots_needed > flexible.len() {
        // Not enough flexible skills - return single distribution with whatever we have
        let mut bar1_skills = bar1_forced;
        let mut bar2_skills = bar2_forced;
        let (fill1, fill2) = flexible.split_at(flexible.len().min(bar1_slots_needed));
        bar1_skills.extend_from_slice(fill1);
        bar2_skills.extend_from_slice(fill2);
        return vec![BarDistribution {
            bar1: WeaponBar {
                weapon_type: bar1_weapon,
                skills: bar1_skills,
            },
            bar2: WeaponBar {
                weapon_type: bar2_weapon,
                skills: bar2_skills,
            },
        }];
    }

    let bar2_slots_needed = 5_usize.saturating_sub(bar2_forced.len());
    if bar1_slots_needed + bar2_slots_needed != flexible.len() {
        // Mismatch - return single best-effort distribution
        let mut bar1_skills = bar1_forced;
        let mut bar2_skills = bar2_forced;
        let (fill1, fill2) = flexible.split_at(bar1_slots_needed.min(flexible.len()));
        bar1_skills.extend_from_slice(fill1);
        bar2_skills.extend_from_slice(fill2);
        return vec![BarDistribution {
            bar1: WeaponBar {
                weapon_type: bar1_weapon,
                skills: bar1_skills,
            },
            bar2: WeaponBar {
                weapon_type: bar2_weapon,
                skills: bar2_skills,
            },
        }];
    }

    // Generate all C(flexible.len(), bar1_slots_needed) combinations
    let indices: Vec<usize> = (0..flexible.len()).collect();
    let combos = combinations(&indices, bar1_slots_needed);

    combos
        .into_iter()
        .map(|bar1_indices| {
            let bar1_flex: Vec<&'static SkillData> =
                bar1_indices.iter().map(|&i| flexible[i]).collect();
            let bar2_flex: Vec<&'static SkillData> = (0..flexible.len())
                .filter(|i| !bar1_indices.contains(i))
                .map(|i| flexible[i])
                .collect();

            let mut bar1_skills = bar1_forced.clone();
            bar1_skills.extend(bar1_flex);
            let mut bar2_skills = bar2_forced.clone();
            bar2_skills.extend(bar2_flex);

            BarDistribution {
                bar1: WeaponBar {
                    weapon_type: bar1_weapon,
                    skills: bar1_skills,
                },
                bar2: WeaponBar {
                    weapon_type: bar2_weapon,
                    skills: bar2_skills,
                },
            }
        })
        .collect()
}

fn combinations(items: &[usize], k: usize) -> Vec<Vec<usize>> {
    if k == 0 {
        return vec![vec![]];
    }
    if items.len() < k {
        return vec![];
    }

    let mut result = Vec::new();
    for (i, &item) in items.iter().enumerate() {
        let rest = &items[i + 1..];
        for mut combo in combinations(rest, k - 1) {
            combo.insert(0, item);
            result.push(combo);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_combinations_basic() {
        let items = vec![0, 1, 2, 3];
        let combos = combinations(&items, 2);
        assert_eq!(combos.len(), 6); // C(4,2) = 6
    }

    #[test]
    fn test_combinations_choose_0() {
        let items = vec![0, 1, 2];
        let combos = combinations(&items, 0);
        assert_eq!(combos.len(), 1);
        assert!(combos[0].is_empty());
    }
}
