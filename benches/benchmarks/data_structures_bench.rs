use criterion::{criterion_group, Criterion};
use fusion_calculator_rs::persona_data::utils::get_special_fusion;

fn search_comparisons(c: &mut Criterion) {
    let mut group = c.benchmark_group("data structures");

    let first = "Orthrus";
    let second = "Lucifer";

    group.bench_function("static slice", |b| {
        b.iter(|| get_special_fusion(first, second))
    });
}

criterion_group!(benches, search_comparisons);
