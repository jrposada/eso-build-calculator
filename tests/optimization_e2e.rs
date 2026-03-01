use eso_build_calculator::data::bonuses::CHAMPION_POINTS;
use eso_build_calculator::data::sets::ALL_SETS;
use eso_build_calculator::domain::{BonusData, CharacterStats, ClassName, SetData, SkillLineName};
use eso_build_calculator::infrastructure::logger;
use eso_build_calculator::services::{BuildOptimizer, BuildOptimizerOptions};

fn get_champion_point(name: &str) -> BonusData {
    CHAMPION_POINTS
        .iter()
        .find(|cp| cp.name == name)
        .cloned()
        .unwrap_or_else(|| panic!("Champion point '{}' not found", name))
}

fn get_set(name: &str) -> &'static SetData {
    ALL_SETS
        .iter()
        .find(|s| s.name == name)
        .unwrap_or_else(|| panic!("Set '{}' not found", name))
}

fn resolve_set_bonuses(sets: &[&'static SetData]) -> (Vec<BonusData>, Vec<String>) {
    let mut bonuses = Vec::new();
    let mut names = Vec::new();
    for set in sets {
        bonuses.extend(
            set.bonuses_at(set.set_type.max_pieces())
                .into_iter()
                .cloned(),
        );
        names.push(set.name.clone());
    }
    (bonuses, names)
}

/// Single CP combo path (all 4 CPs forced → 1 combination).
/// Matches benchmark: nightblade_bow_2h_with_cp
#[test]
fn nightblade_bow_2h_all_cp_forced() {
    logger::set_quiet(true);

    let optimizer = BuildOptimizer::new(BuildOptimizerOptions {
        character_stats: CharacterStats::default(),
        verbose: false,
        pure_class: Some(ClassName::Nightblade),
        required_class_names: vec![],
        required_weapon_skill_lines: vec![SkillLineName::Bow, SkillLineName::TwoHanded],
        required_champion_points: vec![
            get_champion_point("Deadly Aim"),
            get_champion_point("Master-at-Arms"),
            get_champion_point("Thaumaturge"),
            get_champion_point("Biting Aura"),
        ],
        required_skills: vec![],
        forced_morphs: vec![],
        parallelism: 4,
        max_pool_size: None,
        set_bonuses: vec![],
        set_names: vec![],
    });

    let builds = optimizer.find_optimal_build();
    assert!(!builds.is_empty(), "Should find a build");
    let build = &builds[0];

    assert_eq!(
        build.total_damage_per_cast as u64, 14_173,
        "Total damage changed — optimization may have introduced a regression"
    );

    let skill_names = build.skill_names();
    assert_eq!(skill_names.len(), 10);

    let expected_skills = [
        "Thunderous Volley",
        "Dark Shade",
        "Stampede",
        "Crippling Grasp",
        "Barbed Trap",
        "Scalding Rune",
        "Carve",
        "Merciless Resolve",
        "Lotus Fan",
        "Concealed Weapon",
    ];
    for expected in &expected_skills {
        assert!(
            skill_names.iter().any(|s| s == expected),
            "Expected skill '{}' not found in build: {:?}",
            expected,
            skill_names
        );
    }

    let cp_names = build.champion_point_names();
    let expected_cps = ["Biting Aura", "Deadly Aim", "Master-at-Arms", "Thaumaturge"];
    for expected in &expected_cps {
        assert!(
            cp_names.iter().any(|s| s == expected),
            "Expected CP '{}' not found in build: {:?}",
            expected,
            cp_names
        );
    }
}

/// Multi CP combo path (1 CP forced → 20 combinations, exercises cached passive path).
/// Matches benchmark: nightblade_bow_2h_multi_cp
#[test]
fn nightblade_bow_2h_one_cp_forced() {
    logger::set_quiet(true);

    let optimizer = BuildOptimizer::new(BuildOptimizerOptions {
        character_stats: CharacterStats::default(),
        verbose: false,
        pure_class: Some(ClassName::Nightblade),
        required_class_names: vec![],
        required_weapon_skill_lines: vec![SkillLineName::Bow, SkillLineName::TwoHanded],
        required_champion_points: vec![get_champion_point("Deadly Aim")],
        required_skills: vec![],
        forced_morphs: vec![],
        parallelism: 4,
        max_pool_size: None,
        set_bonuses: vec![],
        set_names: vec![],
    });

    let builds = optimizer.find_optimal_build();
    assert!(!builds.is_empty(), "Should find a build");
    let build = &builds[0];

    assert_eq!(
        build.total_damage_per_cast as u64, 14_326,
        "Total damage changed — optimization may have introduced a regression"
    );

    let skill_names = build.skill_names();
    assert_eq!(skill_names.len(), 10);

    let expected_skills = [
        "Thunderous Volley",
        "Dark Shade",
        "Stampede",
        "Crippling Grasp",
        "Barbed Trap",
        "Scalding Rune",
        "Carve",
        "Merciless Resolve",
        "Lotus Fan",
        "Concealed Weapon",
    ];
    for expected in &expected_skills {
        assert!(
            skill_names.iter().any(|s| s == expected),
            "Expected skill '{}' not found in build: {:?}",
            expected,
            skill_names
        );
    }

    let cp_names = build.champion_point_names();
    let expected_cps = ["Backstabber", "Deadly Aim", "Fighting Finesse", "Thaumaturge"];
    for expected in &expected_cps {
        assert!(
            cp_names.iter().any(|s| s == expected),
            "Expected CP '{}' not found in build: {:?}",
            expected,
            cp_names
        );
    }
}

/// Fixed sets passed to optimizer should increase damage over no sets
/// and set names should appear in the result build.
#[test]
fn optimizer_with_fixed_sets_increases_damage() {
    logger::set_quiet(true);

    let mothers = get_set("Mother's Sorrow");
    let julianos = get_set("Law of Julianos");
    let (set_bonuses, set_names) = resolve_set_bonuses(&[mothers, julianos]);

    let make_optimizer = |bonuses: Vec<BonusData>, names: Vec<String>| {
        BuildOptimizer::new(BuildOptimizerOptions {
            character_stats: CharacterStats::default(),
            verbose: false,
            pure_class: Some(ClassName::Nightblade),
            required_class_names: vec![],
            required_weapon_skill_lines: vec![SkillLineName::Bow, SkillLineName::TwoHanded],
            required_champion_points: vec![
                get_champion_point("Deadly Aim"),
                get_champion_point("Master-at-Arms"),
                get_champion_point("Thaumaturge"),
                get_champion_point("Biting Aura"),
            ],
            required_skills: vec![],
            forced_morphs: vec![],
            parallelism: 4,
            max_pool_size: None,
            set_bonuses: bonuses,
            set_names: names,
        })
    };

    // Without sets
    let builds_no_sets = make_optimizer(vec![], vec![]).find_optimal_build();
    assert!(!builds_no_sets.is_empty());
    let damage_no_sets = builds_no_sets[0].total_damage_per_cast;

    // With sets
    let builds_with_sets = make_optimizer(set_bonuses, set_names).find_optimal_build();
    assert!(!builds_with_sets.is_empty());
    let damage_with_sets = builds_with_sets[0].total_damage_per_cast;

    assert!(
        damage_with_sets > damage_no_sets,
        "Sets should increase damage: with_sets={} vs no_sets={}",
        damage_with_sets,
        damage_no_sets
    );

    // Set names should appear in the output build
    let result_set_names = builds_with_sets[0].set_names();
    assert!(
        result_set_names.contains(&"Mother's Sorrow".to_string()),
        "Mother's Sorrow should be in build set_names: {:?}",
        result_set_names
    );
    assert!(
        result_set_names.contains(&"Law of Julianos".to_string()),
        "Law of Julianos should be in build set_names: {:?}",
        result_set_names
    );
}

/// Fixed sets should not change the skill selection for a fully-constrained build.
/// The same skills/CPs should be optimal regardless of flat stat bonuses from sets.
#[test]
fn optimizer_with_fixed_sets_same_skills() {
    logger::set_quiet(true);

    let mothers = get_set("Mother's Sorrow");
    let (set_bonuses, set_names) = resolve_set_bonuses(&[mothers]);

    let make_optimizer = |bonuses: Vec<BonusData>, names: Vec<String>| {
        BuildOptimizer::new(BuildOptimizerOptions {
            character_stats: CharacterStats::default(),
            verbose: false,
            pure_class: Some(ClassName::Nightblade),
            required_class_names: vec![],
            required_weapon_skill_lines: vec![SkillLineName::Bow, SkillLineName::TwoHanded],
            required_champion_points: vec![
                get_champion_point("Deadly Aim"),
                get_champion_point("Master-at-Arms"),
                get_champion_point("Thaumaturge"),
                get_champion_point("Biting Aura"),
            ],
            required_skills: vec![],
            forced_morphs: vec![],
            parallelism: 4,
            max_pool_size: None,
            set_bonuses: bonuses,
            set_names: names,
        })
    };

    let builds_no_sets = make_optimizer(vec![], vec![]).find_optimal_build();
    let builds_with_sets = make_optimizer(set_bonuses, set_names).find_optimal_build();

    let mut skills_no_sets = builds_no_sets[0].skill_names();
    let mut skills_with_sets = builds_with_sets[0].skill_names();
    skills_no_sets.sort();
    skills_with_sets.sort();

    // Flat stat bonuses (magicka, crit) scale all skills equally,
    // so skill ranking should be preserved
    assert_eq!(
        skills_no_sets, skills_with_sets,
        "Same skills should be optimal with flat-bonus sets"
    );
}
