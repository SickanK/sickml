use criterion::{black_box, criterion_group, criterion_main, Criterion};
use sick_ml::old_matrix_mult;
use sick_ml::vector_iterator;

pub fn matrix_mult_benchmark(c: &mut Criterion) {
    c.bench_function("10 matrix_mult", |b| {
        b.iter(|| old_matrix_mult(black_box(10)))
    });

    c.bench_function("100 matrix_mult", |b| {
        b.iter(|| old_matrix_mult(black_box(100)))
    });

    c.bench_function("500 matrix_mult", |b| {
        b.iter(|| old_matrix_mult(black_box(500)))
    });

    c.bench_function("1000 matrix_mult", |b| {
        b.iter(|| old_matrix_mult(black_box(1000)))
    });
}

// 243.83 ns - original
// 111.84 ns - added Vector instead and changed adder system
// 1.3204 ns - inline
pub fn iterator_benchmark(c: &mut Criterion) {
    c.bench_function("10000 vector iterator", |b| b.iter(|| vector_iterator()));
    c.bench_function("10000 vector mut iterator", |b| {
        b.iter(|| vector_iterator())
    });
}

criterion_group!(benches, iterator_benchmark);
criterion_main!(benches);
