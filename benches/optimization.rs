use criterion::{black_box, criterion_group, criterion_main, Criterion};
use eso_build_calculator::data::{ClassName, SkillLineName};
use eso_build_calculator::services::{BuildOptimizer, BuildOptimizerOptions};
use std::time::Duration;

fn benchmark_pure_nightblade_bow(c: &mut Criterion) {
    let mut group = c.benchmark_group("optimization");
    // Set measurement time to get stable results
    group.measurement_time(Duration::from_secs(10));
    // Baseline: ~1.6s per iteration
    group.sample_size(10);

    group.bench_function("pure_nightblade_bow", |b| {
        b.iter(|| {
            let optimizer = BuildOptimizer::new(BuildOptimizerOptions {
                verbose: false,
                pure_class: Some(ClassName::Nightblade),
                required_class_names: vec![],
                required_weapon_skill_lines: vec![SkillLineName::Bow],
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
