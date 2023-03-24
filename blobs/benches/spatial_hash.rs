use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn spatial_hash_benchmark(c: &mut Criterion) {
    let mut arr = black_box([6, 2, 3, 4, 5]);

    c.bench_function("sorting", |b| b.iter(|| arr.sort()));
}

criterion_group!(benches, spatial_hash_benchmark);
criterion_main!(benches);
