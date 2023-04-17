use criterion::{criterion_group, Criterion};
use fusion_calculator_rs::persona_data::utils::get_fused_arcana;

fn search_comparisons(c: &mut Criterion) {
    let mut group = c.benchmark_group("data structures");

    let first = "Strength";
    let second = "Tower";

    group.bench_function("map of maps", |b| {
        b.iter(|| get_fused_arcana(first, second))
    });
}

criterion_group!(benches, search_comparisons);
