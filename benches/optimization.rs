use criterion::{black_box, criterion_group, criterion_main, Criterion};
use eso_build_calculator::domain::{ClassName, SkillLineName};
use eso_build_calculator::services::{BuildOptimizer, BuildOptimizerOptions};
use std::time::Duration;

fn benchmark_pure_nightblade_bow(c: &mut Criterion) {
    let mut group = c.benchmark_group("optimization");
    group.measurement_time(Duration::from_secs(10));
    group.sample_size(10);

    group.bench_function("pure_nightblade_bow", |b| {
        b.iter(|| {
            let optimizer = BuildOptimizer::new(BuildOptimizerOptions {
                verbose: false,
                pure_class: Some(ClassName::Nightblade),
                required_class_names: vec![],
                required_weapon_skill_lines: vec![SkillLineName::Bow],
                required_champion_points: vec![],
                forced_morphs: vec![],
                parallelism: 4,
            });
            black_box(optimizer.find_optimal_build())
        })
    });

    group.finish();
}

criterion_group!(benches, benchmark_pure_nightblade_bow);
criterion_main!(benches);
