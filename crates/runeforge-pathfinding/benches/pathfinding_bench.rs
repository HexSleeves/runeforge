//! Benchmarks for pathfinding algorithms.

use std::hint::black_box;

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use glam::IVec2;
use runeforge_pathfinding::prelude::{PathFinder, PathProvider};

/// A simple grid-based path provider for benchmarking.
struct GridProvider<F> {
    is_walkable: F,
    eight_dir: bool,
}

impl<F> GridProvider<F>
where
    F: Fn(IVec2) -> bool,
{
    fn new(is_walkable: F, eight_dir: bool) -> Self {
        Self {
            is_walkable,
            eight_dir,
        }
    }
}

impl<F> PathProvider<()> for GridProvider<F>
where
    F: Fn(IVec2) -> bool,
{
    fn get_neighbors(&self, position: IVec2, _pass_through_data: &mut ()) -> Vec<IVec2> {
        let directions = if self.eight_dir {
            vec![
                IVec2::new(0, -1),
                IVec2::new(0, 1),
                IVec2::new(-1, 0),
                IVec2::new(1, 0),
                IVec2::new(-1, -1),
                IVec2::new(1, -1),
                IVec2::new(-1, 1),
                IVec2::new(1, 1),
            ]
        } else {
            vec![
                IVec2::new(0, -1),
                IVec2::new(0, 1),
                IVec2::new(-1, 0),
                IVec2::new(1, 0),
            ]
        };

        directions
            .into_iter()
            .map(|d| position + d)
            .filter(|&p| (self.is_walkable)(p))
            .collect()
    }
}

/// Creates a simple open map (no obstacles except boundary)
fn create_open_map(width: i32, height: i32) -> impl Fn(IVec2) -> bool {
    move |p: IVec2| p.x >= 0 && p.y >= 0 && p.x < width && p.y < height
}

/// Creates a map with scattered walls (every 3rd cell is blocked)
fn create_sparse_walls(width: i32, height: i32) -> impl Fn(IVec2) -> bool {
    move |p: IVec2| {
        // Boundary check
        if p.x < 0 || p.y < 0 || p.x >= width || p.y >= height {
            return false;
        }
        // Walls every 3 tiles (but not on edges)
        !(p.x > 0 && p.y > 0 && p.x % 3 == 0 && p.y % 3 == 0)
    }
}

/// Creates a maze-like map with corridors
fn create_maze_map(width: i32, height: i32) -> impl Fn(IVec2) -> bool {
    move |p: IVec2| {
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
        let start = IVec2::new(5, 5);
        let goal = IVec2::new(5 + distance, 5 + distance);

        group.bench_with_input(BenchmarkId::new("4dir", distance), distance, |b, _| {
            let mut provider = GridProvider::new(create_open_map(width, height), false);
            b.iter(|| {
                PathFinder::Astar.compute(black_box(start), black_box(goal), &mut provider, ())
            });
        });

        group.bench_with_input(BenchmarkId::new("8dir", distance), distance, |b, _| {
            let mut provider = GridProvider::new(create_open_map(width, height), true);
            b.iter(|| {
                PathFinder::Astar.compute(black_box(start), black_box(goal), &mut provider, ())
            });
        });
    }

    group.finish();
}

fn bench_pathfinding_map_complexity(c: &mut Criterion) {
    let mut group = c.benchmark_group("pathfinding_map_complexity");
    let start = IVec2::new(5, 5);
    let goal = IVec2::new(80, 80);

    // Open map (no obstacles)
    group.bench_function("open_4dir", |b| {
        let mut provider = GridProvider::new(create_open_map(100, 100), false);
        b.iter(|| PathFinder::Astar.compute(black_box(start), black_box(goal), &mut provider, ()));
    });

    group.bench_function("open_8dir", |b| {
        let mut provider = GridProvider::new(create_open_map(100, 100), true);
        b.iter(|| PathFinder::Astar.compute(black_box(start), black_box(goal), &mut provider, ()));
    });

    // Sparse walls
    group.bench_function("sparse_walls_4dir", |b| {
        let mut provider = GridProvider::new(create_sparse_walls(100, 100), false);
        b.iter(|| PathFinder::Astar.compute(black_box(start), black_box(goal), &mut provider, ()));
    });

    group.bench_function("sparse_walls_8dir", |b| {
        let mut provider = GridProvider::new(create_sparse_walls(100, 100), true);
        b.iter(|| PathFinder::Astar.compute(black_box(start), black_box(goal), &mut provider, ()));
    });

    // Maze-like map
    group.bench_function("maze_4dir", |b| {
        let mut provider = GridProvider::new(create_maze_map(100, 100), false);
        b.iter(|| PathFinder::Astar.compute(black_box(start), black_box(goal), &mut provider, ()));
    });

    group.bench_function("maze_8dir", |b| {
        let mut provider = GridProvider::new(create_maze_map(100, 100), true);
        b.iter(|| PathFinder::Astar.compute(black_box(start), black_box(goal), &mut provider, ()));
    });

    group.finish();
}

fn bench_pathfinding_no_path(c: &mut Criterion) {
    let mut group = c.benchmark_group("pathfinding_no_path");

    // Test worst-case scenario: exhaustive search with no path
    // Bounded area with complete wall blocking path
    let start = IVec2::new(1, 1);
    let goal = IVec2::new(18, 18);

    let is_walkable = |p: IVec2| {
        // Boundary
        if p.x < 0 || p.y < 0 || p.x >= 20 || p.y >= 20 {
            return false;
        }
        // Vertical wall at x=10 completely blocks path
        p.x != 10
    };

    group.bench_function("blocked_4dir", |b| {
        let mut provider = GridProvider::new(is_walkable, false);
        b.iter(|| PathFinder::Astar.compute(black_box(start), black_box(goal), &mut provider, ()));
    });

    group.bench_function("blocked_8dir", |b| {
        let mut provider = GridProvider::new(is_walkable, true);
        b.iter(|| PathFinder::Astar.compute(black_box(start), black_box(goal), &mut provider, ()));
    });

    group.finish();
}

fn bench_pathfinding_algorithm_comparison(c: &mut Criterion) {
    let mut group = c.benchmark_group("pathfinding_algorithm");

    let start = IVec2::new(5, 5);
    let goal = IVec2::new(50, 50);

    // Compare algorithms on an open map
    group.bench_function("astar", |b| {
        let mut provider = GridProvider::new(create_open_map(100, 100), false);
        b.iter(|| PathFinder::Astar.compute(black_box(start), black_box(goal), &mut provider, ()));
    });

    group.bench_function("dijkstra", |b| {
        let mut provider = GridProvider::new(create_open_map(100, 100), false);
        b.iter(|| {
            PathFinder::Dijkstra.compute(black_box(start), black_box(goal), &mut provider, ())
        });
    });

    group.bench_function("bfs", |b| {
        let mut provider = GridProvider::new(create_open_map(100, 100), false);
        b.iter(|| PathFinder::Bfs.compute(black_box(start), black_box(goal), &mut provider, ()));
    });

    group.finish();
}

fn bench_pathfinding_maze_complexity(c: &mut Criterion) {
    let mut group = c.benchmark_group("pathfinding_maze");

    let start = IVec2::new(5, 5);
    let goal = IVec2::new(15, 15);

    // Maze-like map
    group.bench_function("astar_4dir", |b| {
        let mut provider = GridProvider::new(create_maze_map(100, 100), false);
        b.iter(|| PathFinder::Astar.compute(black_box(start), black_box(goal), &mut provider, ()));
    });

    group.bench_function("astar_8dir", |b| {
        let mut provider = GridProvider::new(create_maze_map(100, 100), true);
        b.iter(|| PathFinder::Astar.compute(black_box(start), black_box(goal), &mut provider, ()));
    });

    group.bench_function("dijkstra_4dir", |b| {
        let mut provider = GridProvider::new(create_maze_map(100, 100), false);
        b.iter(|| {
            PathFinder::Dijkstra.compute(black_box(start), black_box(goal), &mut provider, ())
        });
    });

    group.bench_function("dijkstra_8dir", |b| {
        let mut provider = GridProvider::new(create_maze_map(100, 100), true);
        b.iter(|| {
            PathFinder::Dijkstra.compute(black_box(start), black_box(goal), &mut provider, ())
        });
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_pathfinding_distance,
    bench_pathfinding_map_complexity,
    bench_pathfinding_no_path,
    bench_pathfinding_algorithm_comparison,
    bench_pathfinding_maze_complexity
);
criterion_main!(benches);
