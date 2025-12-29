//! Benchmarks for pathfinding algorithms.

use std::hint::black_box;

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use runeforge_geometry::Point;
use runeforge_pathfinding::{astar, astar_8dir};

/// Creates a simple open map (no obstacles except boundary)
fn create_open_map(width: i32, height: i32) -> impl Fn(Point) -> bool {
    move |p: Point| p.x >= 0 && p.y >= 0 && p.x < width && p.y < height
}

/// Creates a map with scattered walls (every 3rd cell is blocked)
fn create_sparse_walls(width: i32, height: i32) -> impl Fn(Point) -> bool {
    move |p: Point| {
        // Boundary check
        if p.x < 0 || p.y < 0 || p.x >= width || p.y >= height {
            return false;
        }
        // Walls every 3 tiles (but not on edges)
        !(p.x > 0 && p.y > 0 && p.x % 3 == 0 && p.y % 3 == 0)
    }
}

/// Creates a maze-like map with corridors
fn create_maze_map(width: i32, height: i32) -> impl Fn(Point) -> bool {
    move |p: Point| {
        // Boundary check
        if p.x < 0 || p.y < 0 || p.x >= width || p.y >= height {
            return false;
        }
        // Create corridors: walkable if x or y is even
        p.x % 2 == 0 || p.y % 2 == 0
    }
}

fn bench_pathfinding_distance(c: &mut Criterion) {
    let mut group = c.benchmark_group("pathfinding_distance");
    let width = 100;
    let height = 100;

    // Test different path lengths
    for distance in [10, 25, 50, 75].iter() {
        let is_walkable = create_open_map(width, height);
        let start = Point::new(5, 5);
        let goal = Point::new(5 + distance, 5 + distance);

        group.bench_with_input(BenchmarkId::new("4dir", distance), distance, |b, _| {
            b.iter(|| astar(black_box(start), black_box(goal), &is_walkable, None));
        });

        group.bench_with_input(BenchmarkId::new("8dir", distance), distance, |b, _| {
            b.iter(|| astar_8dir(black_box(start), black_box(goal), &is_walkable, None));
        });
    }

    group.finish();
}

fn bench_pathfinding_map_complexity(c: &mut Criterion) {
    let mut group = c.benchmark_group("pathfinding_map_complexity");
    let start = Point::new(5, 5);
    let goal = Point::new(80, 80);

    // Open map (no obstacles)
    group.bench_function("open_4dir", |b| {
        let is_walkable = create_open_map(100, 100);
        b.iter(|| astar(black_box(start), black_box(goal), &is_walkable, None));
    });

    group.bench_function("open_8dir", |b| {
        let is_walkable = create_open_map(100, 100);
        b.iter(|| astar_8dir(black_box(start), black_box(goal), &is_walkable, None));
    });

    // Sparse walls
    group.bench_function("sparse_walls_4dir", |b| {
        let is_walkable = create_sparse_walls(100, 100);
        b.iter(|| astar(black_box(start), black_box(goal), &is_walkable, None));
    });

    group.bench_function("sparse_walls_8dir", |b| {
        let is_walkable = create_sparse_walls(100, 100);
        b.iter(|| astar_8dir(black_box(start), black_box(goal), &is_walkable, None));
    });

    // Maze-like map
    group.bench_function("maze_4dir", |b| {
        let is_walkable = create_maze_map(100, 100);
        b.iter(|| astar(black_box(start), black_box(goal), &is_walkable, None));
    });

    group.bench_function("maze_8dir", |b| {
        let is_walkable = create_maze_map(100, 100);
        b.iter(|| astar_8dir(black_box(start), black_box(goal), &is_walkable, None));
    });

    group.finish();
}

fn bench_pathfinding_no_path(c: &mut Criterion) {
    let mut group = c.benchmark_group("pathfinding_no_path");

    // Test worst-case scenario: exhaustive search with no path
    // Bounded area with complete wall blocking path
    let start = Point::new(1, 1);
    let goal = Point::new(18, 18);

    let is_walkable = |p: Point| {
        // Boundary
        if p.x < 0 || p.y < 0 || p.x >= 20 || p.y >= 20 {
            return false;
        }
        // Vertical wall at x=10 completely blocks path
        p.x != 10
    };

    group.bench_function("blocked_4dir", |b| {
        b.iter(|| astar(black_box(start), black_box(goal), &is_walkable, None));
    });

    group.bench_function("blocked_8dir", |b| {
        b.iter(|| astar_8dir(black_box(start), black_box(goal), &is_walkable, None));
    });

    group.finish();
}

fn bench_pathfinding_bounds(c: &mut Criterion) {
    let mut group = c.benchmark_group("pathfinding_bounds");

    // Test the impact of bounds on search performance
    // Large open map to see bounds effect clearly
    let start = Point::new(10, 10);
    let goal = Point::new(90, 90);

    // Large open area
    let is_walkable = |p: Point| p.x >= 0 && p.y >= 0 && p.x < 100 && p.y < 100;

    // Unbounded search (can explore entire map)
    group.bench_function("unbounded_4dir", |b| {
        b.iter(|| astar(black_box(start), black_box(goal), &is_walkable, None));
    });

    group.bench_function("unbounded_8dir", |b| {
        b.iter(|| astar_8dir(black_box(start), black_box(goal), &is_walkable, None));
    });

    // Bounded search (constrained to smaller area)
    // Bounds that are just large enough for the path
    let tight_bounds = Some((5, 5, 95, 95));

    group.bench_function("bounded_tight_4dir", |b| {
        b.iter(|| {
            astar(
                black_box(start),
                black_box(goal),
                &is_walkable,
                tight_bounds,
            )
        });
    });

    group.bench_function("bounded_tight_8dir", |b| {
        b.iter(|| {
            astar_8dir(
                black_box(start),
                black_box(goal),
                &is_walkable,
                tight_bounds,
            )
        });
    });

    // Very tight bounds that still allow path but constrain exploration
    let very_tight_bounds = Some((8, 8, 92, 92));

    group.bench_function("bounded_very_tight_4dir", |b| {
        b.iter(|| {
            astar(
                black_box(start),
                black_box(goal),
                &is_walkable,
                very_tight_bounds,
            )
        });
    });

    group.bench_function("bounded_very_tight_8dir", |b| {
        b.iter(|| {
            astar_8dir(
                black_box(start),
                black_box(goal),
                &is_walkable,
                very_tight_bounds,
            )
        });
    });

    group.finish();
}

fn bench_pathfinding_bounds_complexity(c: &mut Criterion) {
    let mut group = c.benchmark_group("pathfinding_bounds_complexity");

    // Test bounds impact on different map complexities
    let start = Point::new(5, 5);
    let goal = Point::new(15, 15);

    // Maze-like map
    let maze_walkable = create_maze_map(100, 100);

    // Unbounded maze search
    group.bench_function("maze_unbounded_4dir", |b| {
        b.iter(|| astar(black_box(start), black_box(goal), &maze_walkable, None));
    });

    group.bench_function("maze_unbounded_8dir", |b| {
        b.iter(|| astar_8dir(black_box(start), black_box(goal), &maze_walkable, None));
    });

    // Bounded maze search (constrain to area around path)
    let maze_bounds = Some((0, 0, 25, 25));

    group.bench_function("maze_bounded_4dir", |b| {
        b.iter(|| {
            astar(
                black_box(start),
                black_box(goal),
                &maze_walkable,
                maze_bounds,
            )
        });
    });

    group.bench_function("maze_bounded_8dir", |b| {
        b.iter(|| {
            astar_8dir(
                black_box(start),
                black_box(goal),
                &maze_walkable,
                maze_bounds,
            )
        });
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_pathfinding_distance,
    bench_pathfinding_map_complexity,
    bench_pathfinding_no_path,
    bench_pathfinding_bounds,
    bench_pathfinding_bounds_complexity
);
criterion_main!(benches);
