extern crate criterion;

use criterion::{criterion_group, criterion_main, Criterion};
use sick_ml::matrix_bench;

pub fn bench(c: &mut Criterion) {
    c.bench_function("500 matrix_mult", |b| b.iter(|| matrix_bench::<500>()));
}

criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(10000);
    targets = bench
}
criterion_main!(benches);
