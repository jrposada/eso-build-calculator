use criterion::{black_box, criterion_group, criterion_main, Criterion};
use eso_build_calculator::data::ClassName;
use eso_build_calculator::services::{BuildOptimizer, BuildOptimizerOptions};

fn benchmark_optimize_single_class(c: &mut Criterion) {
    c.bench_function("optimize_dragonknight", |b| {
        b.iter(|| {
            let optimizer = BuildOptimizer::new(BuildOptimizerOptions {
                verbose: false,
                required_class_names: vec![ClassName::Dragonknight],
                required_weapons: vec![],
                forced_morphs: vec![],
            });
            black_box(optimizer.find_optimal_build())
        })
    });
}

criterion_group!(benches, benchmark_optimize_single_class);
criterion_main!(benches);
