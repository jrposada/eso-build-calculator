use criterion::{criterion_group, criterion_main, Criterion};
use eso_build_calculator::data::bonuses::CHAMPION_POINTS;
use eso_build_calculator::domain::{ClassName, SkillLineName};
use eso_build_calculator::infrastructure::logger;
use eso_build_calculator::services::{BuildOptimizer, BuildOptimizerOptions};
use std::hint::black_box;
use std::time::Duration;

fn get_champion_point(name: &str) -> eso_build_calculator::domain::BonusData {
    CHAMPION_POINTS
        .iter()
        .find(|cp| cp.name == name)
        .cloned()
        .expect(&format!("Champion point '{}' not found", name))
}

fn benchmark_nightblade_bow_2h_with_cp(c: &mut Criterion) {
    logger::set_quiet(true);
    let mut group = c.benchmark_group("optimization");
    group.measurement_time(Duration::from_secs(10));
    group.sample_size(10);

    group.bench_function("nightblade_bow_2h_with_cp", |b| {
        b.iter(|| {
            let optimizer = BuildOptimizer::new(BuildOptimizerOptions {
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
            });
            black_box(optimizer.find_optimal_build())
        })
    });

    group.finish();
}

criterion_group!(benches, benchmark_nightblade_bow_2h_with_cp);
criterion_main!(benches);
