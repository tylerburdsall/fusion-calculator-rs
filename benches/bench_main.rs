use benchmarks::data_structures_bench;
use criterion::criterion_main;

mod benchmarks;

criterion_main! {
    data_structures_bench::benches
}
