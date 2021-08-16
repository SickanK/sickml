use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn bench(c: &mut Criterion) {
    print!("pog");
}

criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(10000);
    targets = bench
}
criterion_main!(benches);
