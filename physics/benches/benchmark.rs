use criterion::{black_box, criterion_group, criterion_main, Criterion};
use physics::*;
use rand::{Rng};

fn random_circle() -> Circle {
    let mut rng = rand::thread_rng();

    Circle {
        center: Vector2d {
            x: rng.gen_range(-1000.0..1000.0),
            y: rng.gen_range(-1000.0..1000.0),
        },
        r: rng.gen_range(0.0..500.0),
    }
}

fn random_aabb() -> AABB {
    let mut rng = rand::thread_rng();

    AABB {
        center: Vector2d {
            x: rng.gen_range(-1000.0..1000.0),
            y: rng.gen_range(-1000.0..1000.0),
        },
        rx: rng.gen_range(0.0..500.0),
        ry: rng.gen_range(0.0..500.0),
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("sirkel aabb", |b| b.iter(||
        intersection(
            black_box(Shape::Circle(random_circle())),
            black_box(Shape::AABB(random_aabb())),
        )
    ));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);