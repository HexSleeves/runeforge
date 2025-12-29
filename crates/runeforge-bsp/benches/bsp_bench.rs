//! Benchmarks for BSP dungeon generation.

use std::hint::black_box;

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use runeforge_bsp::{BspConfig, DungeonGenerator};
use runeforge_random::Rng;

fn bench_dungeon_size(c: &mut Criterion) {
    let mut group = c.benchmark_group("bsp_dungeon_size");
    let config = BspConfig::default();

    // Test different dungeon sizes
    for (width, height) in [(40, 25), (60, 40), (80, 50), (100, 60), (120, 80)].iter() {
        let mut rng = Rng::new();

        group.bench_with_input(
            BenchmarkId::new("generate", format!("{}x{}", width, height)),
            &(*width, *height),
            |b, &(w, h)| {
                b.iter(|| {
                    let dungeon = DungeonGenerator::generate(
                        black_box(w),
                        black_box(h),
                        black_box(&config),
                        &mut rng,
                    );
                    dungeon.rooms().len()
                });
            },
        );
    }

    group.finish();
}

fn bench_dungeon_depth(c: &mut Criterion) {
    let mut group = c.benchmark_group("bsp_dungeon_depth");
    let width = 80;
    let height = 50;

    // Test different recursion depths
    for depth in [3, 4, 5, 6, 7].iter() {
        let config = BspConfig::new()
            .with_min_partition_size(8, 8)
            .with_min_room_size(4, 4)
            .with_max_depth(*depth);
        let mut rng = Rng::new();

        group.bench_with_input(BenchmarkId::new("depth", depth), depth, |b, _| {
            b.iter(|| {
                let dungeon = DungeonGenerator::generate(
                    black_box(width),
                    black_box(height),
                    black_box(&config),
                    &mut rng,
                );
                dungeon.rooms().len()
            });
        });
    }

    group.finish();
}

fn bench_dungeon_room_sizes(c: &mut Criterion) {
    let mut group = c.benchmark_group("bsp_room_sizes");
    let width = 80;
    let height = 50;

    // Small rooms (more splits)
    group.bench_function("small_rooms", |b| {
        let config = BspConfig::new()
            .with_min_partition_size(8, 8)
            .with_min_room_size(3, 3)
            .with_max_depth(6);
        let mut rng = Rng::new();

        b.iter(|| {
            let dungeon = DungeonGenerator::generate(
                black_box(width),
                black_box(height),
                black_box(&config),
                &mut rng,
            );
            dungeon.rooms().len()
        });
    });

    // Medium rooms (default)
    group.bench_function("medium_rooms", |b| {
        let config = BspConfig::default();
        let mut rng = Rng::new();

        b.iter(|| {
            let dungeon = DungeonGenerator::generate(
                black_box(width),
                black_box(height),
                black_box(&config),
                &mut rng,
            );
            dungeon.rooms().len()
        });
    });

    // Large rooms (fewer splits)
    group.bench_function("large_rooms", |b| {
        let config = BspConfig::new()
            .with_min_partition_size(20, 15)
            .with_min_room_size(8, 6)
            .with_max_depth(3);
        let mut rng = Rng::new();

        b.iter(|| {
            let dungeon = DungeonGenerator::generate(
                black_box(width),
                black_box(height),
                black_box(&config),
                &mut rng,
            );
            dungeon.rooms().len()
        });
    });

    group.finish();
}

fn bench_dungeon_access(c: &mut Criterion) {
    let mut group = c.benchmark_group("bsp_dungeon_access");
    let config = BspConfig::default();
    let mut rng = Rng::new();
    let dungeon = DungeonGenerator::generate(80, 50, &config, &mut rng);

    // Benchmark is_floor checks (common operation in games)
    group.bench_function("is_floor_100_checks", |b| {
        b.iter(|| {
            let mut floor_count = 0;
            for y in 0..10 {
                for x in 0..10 {
                    if dungeon.is_floor(black_box(x * 8), black_box(y * 5)) {
                        floor_count += 1;
                    }
                }
            }
            floor_count
        });
    });

    // Benchmark full dungeon iteration
    group.bench_function("iterate_all_tiles", |b| {
        b.iter(|| {
            let mut floor_count = 0;
            for y in 0..dungeon.height() {
                for x in 0..dungeon.width() {
                    if dungeon.is_floor(black_box(x as i32), black_box(y as i32)) {
                        floor_count += 1;
                    }
                }
            }
            floor_count
        });
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_dungeon_size,
    bench_dungeon_depth,
    bench_dungeon_room_sizes,
    bench_dungeon_access
);
criterion_main!(benches);
