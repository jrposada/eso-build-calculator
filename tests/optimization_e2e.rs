use eso_build_calculator::data::bonuses::CHAMPION_POINTS;
use eso_build_calculator::domain::{CharacterStats, ClassName, SkillLineName};
use eso_build_calculator::infrastructure::logger;
use eso_build_calculator::services::{BuildOptimizer, BuildOptimizerOptions};

fn get_champion_point(name: &str) -> eso_build_calculator::domain::BonusData {
    CHAMPION_POINTS
        .iter()
        .find(|cp| cp.name == name)
        .cloned()
        .unwrap_or_else(|| panic!("Champion point '{}' not found", name))
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
        forced_morphs: vec![],
        parallelism: 4,
        max_pool_size: None,
    });

    let build = optimizer.find_optimal_build().expect("Should find a build");

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
        forced_morphs: vec![],
        parallelism: 4,
        max_pool_size: None,
    });

    let build = optimizer.find_optimal_build().expect("Should find a build");

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
