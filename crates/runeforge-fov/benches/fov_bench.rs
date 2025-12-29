//! Benchmarks for FOV algorithms.

use std::hint::black_box;

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use runeforge_fov::{compute_fov, compute_fov_circle};
use runeforge_geometry::Point;

/// Creates a simple map with walls around the edges
fn create_bounded_map(width: i32, height: i32) -> impl Fn(Point) -> bool {
    move |p: Point| p.x <= 0 || p.y <= 0 || p.x >= width - 1 || p.y >= height - 1
}

/// Creates a map with scattered pillars
fn create_pillar_map(width: i32, height: i32) -> impl Fn(Point) -> bool {
    move |p: Point| {
        // Boundary walls
        if p.x <= 0 || p.y <= 0 || p.x >= width - 1 || p.y >= height - 1 {
            return true;
        }
        // Pillars every 5 tiles
        p.x % 5 == 0 && p.y % 5 == 0
    }
}

fn bench_fov_radius(c: &mut Criterion) {
    let mut group = c.benchmark_group("fov_radius");

    for radius in [5, 10, 15, 20, 30].iter() {
        let is_blocking = create_bounded_map(100, 100);
        let origin = Point::new(50, 50);

        group.bench_with_input(BenchmarkId::new("shadowcast", radius), radius, |b, &r| {
            b.iter(|| {
                let mut count = 0;
                compute_fov(black_box(origin), black_box(r), &is_blocking, &mut |_| {
                    count += 1
                });
                count
            });
        });
    }

    group.finish();
}

fn bench_fov_map_complexity(c: &mut Criterion) {
    let mut group = c.benchmark_group("fov_map_complexity");
    let radius = 15;
    let origin = Point::new(50, 50);

    // Open map (no obstacles)
    group.bench_function("open_map", |b| {
        let is_blocking = |_: Point| false;
        b.iter(|| {
            let mut count = 0;
            compute_fov(
                black_box(origin),
                black_box(radius),
                &is_blocking,
                &mut |_| count += 1,
            );
            count
        });
    });

    // Bounded map (walls at edges)
    group.bench_function("bounded_map", |b| {
        let is_blocking = create_bounded_map(100, 100);
        b.iter(|| {
            let mut count = 0;
            compute_fov(
                black_box(origin),
                black_box(radius),
                &is_blocking,
                &mut |_| count += 1,
            );
            count
        });
    });

    // Dense pillars
    group.bench_function("pillar_map", |b| {
        let is_blocking = create_pillar_map(100, 100);
        b.iter(|| {
            let mut count = 0;
            compute_fov(
                black_box(origin),
                black_box(radius),
                &is_blocking,
                &mut |_| count += 1,
            );
            count
        });
    });

    group.finish();
}

fn bench_fov_circle(c: &mut Criterion) {
    let mut group = c.benchmark_group("fov_circle");

    for radius in [5, 10, 20].iter() {
        let origin = Point::new(50, 50);

        group.bench_with_input(BenchmarkId::new("circle", radius), radius, |b, &r| {
            b.iter(|| {
                let mut count = 0;
                compute_fov_circle(black_box(origin), black_box(r), &mut |_| count += 1);
                count
            });
        });
    }

    group.finish();
}

criterion_group!(
    benches,
    bench_fov_radius,
    bench_fov_map_complexity,
    bench_fov_circle
);
criterion_main!(benches);
