use criterion::{black_box, criterion_group, criterion_main, Criterion};
use sick_ml::fromprimitive_vs_uninit;
use sick_ml::old_matrix_mult;
use sick_ml::sum_test;
use sick_ml::vector::Vector;
use sick_ml::vector_enum_vs_idx;
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

// 8.3103 us - idx
// 8.3871 us - enum
// no diff
pub fn enum_vs_idx_benchmark(c: &mut Criterion) {
    c.bench_function("10000 enum vs idx", |b| b.iter(|| vector_enum_vs_idx()));
}

// 30.019 us - uninit
// 32.251 us - fromprimitive
// uninit faster
pub fn fromprimitive_vs_uninit_benchmark(c: &mut Criterion) {
    c.bench_function("10000 FromPrimitive vs uninit", |b| {
        b.iter(|| fromprimitive_vs_uninit())
    });
}

// 1.5440 ns
fn sum_tst(c: &mut Criterion) {
    c.bench_function("vector addition", |b| {
        b.iter(|| {
            let vec1 = Vector::<i32, 3>::new([2, 3, 4]);
            let vec2 = Vector::<i32, 3>::new([2, 3, 4]);
            criterion::black_box(vec1 + vec2);
        })
    });
}

criterion_group!(benches, sum_tst);
criterion_main!(benches);
