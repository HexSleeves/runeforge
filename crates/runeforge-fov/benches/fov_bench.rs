//! Benchmarks for FOV algorithms.

use std::hint::black_box;

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use glam::IVec2;
use runeforge_fov::prelude::{Fov, FovProvider};

/// A provider that wraps a blocking function
struct MapProvider<F: Fn(IVec2) -> bool> {
    is_blocking: F,
}

impl<F: Fn(IVec2) -> bool> FovProvider<()> for MapProvider<F> {
    fn is_opaque(&mut self, position: IVec2, _pass_through_data: &mut ()) -> bool {
        (self.is_blocking)(position)
    }
}

/// Creates a simple map with walls around the edges
fn create_bounded_map(width: i32, height: i32) -> impl Fn(IVec2) -> bool {
    move |p: IVec2| p.x <= 0 || p.y <= 0 || p.x >= width - 1 || p.y >= height - 1
}

/// Creates a map with scattered pillars
fn create_pillar_map(width: i32, height: i32) -> impl Fn(IVec2) -> bool {
    move |p: IVec2| {
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
        let origin = IVec2::new(50, 50);

        group.bench_with_input(BenchmarkId::new("shadowcast", radius), radius, |b, &r| {
            b.iter(|| {
                let mut provider = MapProvider {
                    is_blocking: create_bounded_map(100, 100),
                };
                Fov::Shadowcast.compute(black_box(origin), black_box(r as u32), &mut provider, ())
            });
        });

        group.bench_with_input(BenchmarkId::new("adams", radius), radius, |b, &r| {
            b.iter(|| {
                let mut provider = MapProvider {
                    is_blocking: create_bounded_map(100, 100),
                };
                Fov::Adams.compute(black_box(origin), black_box(r as u32), &mut provider, ())
            });
        });
    }

    group.finish();
}

fn bench_fov_map_complexity(c: &mut Criterion) {
    let mut group = c.benchmark_group("fov_map_complexity");
    let radius = 15_u32;
    let origin = IVec2::new(50, 50);

    // Open map (no obstacles)
    group.bench_function("open_map", |b| {
        b.iter(|| {
            let mut provider = MapProvider {
                is_blocking: |_: IVec2| false,
            };
            Fov::Shadowcast.compute(black_box(origin), black_box(radius), &mut provider, ())
        });
    });

    // Bounded map (walls at edges)
    group.bench_function("bounded_map", |b| {
        b.iter(|| {
            let mut provider = MapProvider {
                is_blocking: create_bounded_map(100, 100),
            };
            Fov::Shadowcast.compute(black_box(origin), black_box(radius), &mut provider, ())
        });
    });

    // Dense pillars
    group.bench_function("pillar_map", |b| {
        b.iter(|| {
            let mut provider = MapProvider {
                is_blocking: create_pillar_map(100, 100),
            };
            Fov::Shadowcast.compute(black_box(origin), black_box(radius), &mut provider, ())
        });
    });

    group.finish();
}

fn bench_fov_algorithms(c: &mut Criterion) {
    let mut group = c.benchmark_group("fov_algorithms");
    let radius = 15_u32;
    let origin = IVec2::new(50, 50);

    group.bench_function("shadowcast", |b| {
        b.iter(|| {
            let mut provider = MapProvider {
                is_blocking: create_bounded_map(100, 100),
            };
            Fov::Shadowcast.compute(black_box(origin), black_box(radius), &mut provider, ())
        });
    });

    group.bench_function("adams", |b| {
        b.iter(|| {
            let mut provider = MapProvider {
                is_blocking: create_bounded_map(100, 100),
            };
            Fov::Adams.compute(black_box(origin), black_box(radius), &mut provider, ())
        });
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_fov_radius,
    bench_fov_map_complexity,
    bench_fov_algorithms
);
criterion_main!(benches);
