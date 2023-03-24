use blobs::{Rng, SpatialHash};
use glam::Vec2;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::SeedableRng;
use rand_xorshift::XorShiftRng;

fn spatial_hash_benchmark(c: &mut Criterion) {
    let mut rng = XorShiftRng::from_seed([1; 16]);
    let mut spatial_hash = SpatialHash::new(100.0);
    let mut points = Vec::new();

    for _ in 0..1000 {
        let x = rng.gen_range(-500.0..500.0);
        let y = rng.gen_range(-500.0..500.0);
        let radius = rng.gen_range(0.0..10.0);
        let position = Vec2::new(x, y);
        let id = spatial_hash.insert(position, radius);
        points.push((id, position));
    }

    let query_position = Vec2::new(0.0, 0.0);
    let query_radius = black_box(50.0);

    c.bench_function("query", |b| {
        b.iter(|| spatial_hash.query(query_position, query_radius))
    });

    let move_offset = Vec2::new(10.0, 10.0);

    c.bench_function("move_point", |b| {
        b.iter(|| {
            for &(id, _) in points.iter() {
                spatial_hash.move_point(id, move_offset).unwrap();
            }
        });
    });
}

criterion_group!(benches, spatial_hash_benchmark);
criterion_main!(benches);
