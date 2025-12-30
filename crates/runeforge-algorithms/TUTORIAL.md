# Runeforge Algorithms Tutorial

A hands-on guide to generating procedural maps for your roguelike game.

## Table of Contents

1. [Getting Started](#getting-started)
2. [Tutorial 1: Your First Dungeon](#tutorial-1-your-first-dungeon)
3. [Tutorial 2: Natural Cave Systems](#tutorial-2-natural-cave-systems)
4. [Tutorial 3: Winding Tunnels](#tutorial-3-winding-tunnels)
5. [Tutorial 4: Multi-Level Dungeons](#tutorial-4-multi-level-dungeons)
6. [Tutorial 5: Hybrid Maps](#tutorial-5-hybrid-maps)
7. [Tips & Tricks](#tips--tricks)

## Getting Started

### Installation

Create a new Rust project and add dependencies:

```bash
cargo new my-roguelike
cd my-roguelike
```

Add to `Cargo.toml`:

```toml
[dependencies]
runeforge-algorithms = "0.1.0"
runeforge-random = "0.1.0"
```

### Basic Concepts

All algorithms in this crate work with:

- **Width & Height**: Map dimensions in tiles
- **Configuration**: Algorithm-specific settings
- **RNG**: Random number generator for deterministic generation

## Tutorial 1: Your First Dungeon

Let's create a classic dungeon with rooms and corridors using BSP.

### Step 1: Basic Generation

```rust
use runeforge_algorithms::{BspConfig, DungeonGenerator};
use runeforge_random::Rng;

fn main() {
    // Create RNG (random number generator)
    let mut rng = Rng::new();

    // Use default configuration
    let config = BspConfig::default();

    // Generate 80x50 dungeon
    let dungeon = DungeonGenerator::generate(80, 50, &config, &mut rng);

    println!("Generated dungeon with {} rooms", dungeon.rooms().len());
}
```

### Step 2: Rendering the Dungeon

```rust
fn print_dungeon(dungeon: &Dungeon) {
    for y in 0..dungeon.height() {
        for x in 0..dungeon.width() {
            let ch = if dungeon.is_floor(x as i32, y as i32) {
                '.'
            } else {
                '#'
            };
            print!("{}", ch);
        }
        println!();
    }
}

fn main() {
    let mut rng = Rng::new();
    let config = BspConfig::default();
    let dungeon = DungeonGenerator::generate(80, 50, &config, &mut rng);

    print_dungeon(&dungeon);
}
```

### Step 3: Customizing Room Sizes

```rust
fn main() {
    let mut rng = Rng::new();

    // Customize configuration
    let config = BspConfig::default()
        .with_min_room_size(6, 6)      // Larger minimum rooms
        .with_room_padding(2)          // More space around rooms
        .with_max_depth(4);            // Fewer subdivisions = larger rooms

    let dungeon = DungeonGenerator::generate(80, 50, &config, &mut rng);

    // Print room info
    for (i, room) in dungeon.rooms().iter().enumerate() {
        println!("Room {}: {}x{} at ({}, {})",
            i, room.width, room.height, room.x, room.y);
    }

    print_dungeon(&dungeon);
}
```

### Step 4: Finding a Starting Room

```rust
use runeforge_random::Rng;

fn find_starting_room(dungeon: &Dungeon, rng: &mut Rng) -> (i32, i32) {
    let rooms = dungeon.rooms();
    let start_room = &rooms[rng.range(0, (rooms.len() - 1) as i32) as usize];

    // Return center of room
    let center_x = start_room.x as i32 + (start_room.width as i32 / 2);
    let center_y = start_room.y as i32 + (start_room.height as i32 / 2);

    (center_x, center_y)
}

fn main() {
    let mut rng = Rng::new();
    let config = BspConfig::default();
    let dungeon = DungeonGenerator::generate(80, 50, &config, &mut rng);

    let (player_x, player_y) = find_starting_room(&dungeon, &mut rng);
    println!("Player starts at ({}, {})", player_x, player_y);
}
```

## Tutorial 2: Natural Cave Systems

Create organic caves using cellular automata.

### Step 1: Basic Cave

```rust
use runeforge_algorithms::{CaveConfig, CaveGenerator};
use runeforge_random::Rng;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut rng = Rng::new();

    let config = CaveConfig::default();
    let cave = CaveGenerator::generate(80, 50, &config, &mut rng)?;

    println!("Cave has {:.1}% open space", cave.floor_percentage() * 100.0);

    // Print cave
    for y in 0..cave.height() {
        for x in 0..cave.width() {
            print!("{}", if cave.is_floor(x, y) { '.' } else { '#' });
        }
        println!();
    }

    Ok(())
}
```

### Step 2: Adjusting Cave Density

```rust
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut rng = Rng::new();

    // More open caves (less walls)
    let open_config = CaveConfig::default()
        .with_initial_wall_probability(0.35)  // Start with fewer walls
        .with_wall_threshold(4);              // Lower threshold

    let open_cave = CaveGenerator::generate(80, 50, &open_config, &mut rng)?;
    println!("Open cave: {:.1}% floor", open_cave.floor_percentage() * 100.0);

    // Denser caves (more walls)
    let dense_config = CaveConfig::default()
        .with_initial_wall_probability(0.55)  // Start with more walls
        .with_wall_threshold(6);              // Higher threshold

    let dense_cave = CaveGenerator::generate(80, 50, &dense_config, &mut rng)?;
    println!("Dense cave: {:.1}% floor", dense_cave.floor_percentage() * 100.0);

    Ok(())
}
```

### Step 3: Smoothness Control

```rust
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut rng = Rng::new();

    // Rough cave (fewer iterations)
    let rough_config = CaveConfig::default()
        .with_iterations(2);
    let rough_cave = CaveGenerator::generate(80, 50, &rough_config, &mut rng)?;

    // Smooth cave (more iterations)
    let smooth_config = CaveConfig::default()
        .with_iterations(8);
    let smooth_cave = CaveGenerator::generate(80, 50, &smooth_config, &mut rng)?;

    println!("Rough cave (2 iterations):");
    print_cave(&rough_cave);

    println!("\nSmooth cave (8 iterations):");
    print_cave(&smooth_cave);

    Ok(())
}

fn print_cave(cave: &CaveMap) {
    for y in 0..cave.height() {
        for x in 0..cave.width() {
            print!("{}", if cave.is_floor(x, y) { '.' } else { '#' });
        }
        println!();
    }
}
```

## Tutorial 3: Winding Tunnels

Create meandering caves with drunkard's walk.

### Step 1: Basic Tunnel System

```rust
use runeforge_algorithms::{DrunkardConfig, DrunkardGenerator};
use runeforge_random::Rng;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut rng = Rng::new();

    let config = DrunkardConfig::default();
    let cave = DrunkardGenerator::generate(80, 50, &config, &mut rng)?;

    println!("Drunkard cave: {:.1}% floor", cave.floor_percentage() * 100.0);

    // Print cave
    for y in 0..cave.height() {
        for x in 0..cave.width() {
            print!("{}", if cave.is_floor(x, y) { '.' } else { '#' });
        }
        println!();
    }

    Ok(())
}
```

### Step 2: Controlling Coverage

```rust
use runeforge_algorithms::StartPosition;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut rng = Rng::new();

    // Sparse tunnels (20% coverage)
    let sparse_config = DrunkardConfig::default()
        .with_target_floor_percentage(0.2);

    // Moderate tunnels (40% coverage)
    let moderate_config = DrunkardConfig::default()
        .with_target_floor_percentage(0.4);

    // Dense tunnels (60% coverage)
    let dense_config = DrunkardConfig::default()
        .with_target_floor_percentage(0.6);

    let sparse = DrunkardGenerator::generate(80, 50, &sparse_config, &mut rng)?;
    println!("Sparse: {:.1}% floor", sparse.floor_percentage() * 100.0);

    let moderate = DrunkardGenerator::generate(80, 50, &moderate_config, &mut rng)?;
    println!("Moderate: {:.1}% floor", moderate.floor_percentage() * 100.0);

    let dense = DrunkardGenerator::generate(80, 50, &dense_config, &mut rng)?;
    println!("Dense: {:.1}% floor", dense.floor_percentage() * 100.0);

    Ok(())
}
```

### Step 3: Start Position Strategies

```rust
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut rng = Rng::new();

    // Start from center (radial expansion)
    let center_config = DrunkardConfig::default()
        .with_start_position(StartPosition::Center);
    let center_cave = DrunkardGenerator::generate(80, 50, &center_config, &mut rng)?;

    // Start from random position (unpredictable)
    let random_config = DrunkardConfig::default()
        .with_start_position(StartPosition::Random);
    let random_cave = DrunkardGenerator::generate(80, 50, &random_config, &mut rng)?;

    Ok(())
}
```

## Tutorial 4: Multi-Level Dungeons

Generate different levels with different algorithms.

```rust
use runeforge_algorithms::*;
use runeforge_random::Rng;

enum LevelType {
    Dungeon,
    Cave,
    Mine,
}

struct GameLevel {
    level_type: LevelType,
    depth: u32,
    // Could be any of the map types
}

fn generate_level(depth: u32, rng: &mut Rng) -> Result<(), Box<dyn std::error::Error>> {
    match depth {
        1..=3 => {
            // Early levels: structured dungeons
            println!("Level {}: Dungeon", depth);
            let config = BspConfig::default();
            let dungeon = DungeonGenerator::generate(80, 50, &config, rng);
            print_dungeon(&dungeon);
        }
        4..=6 => {
            // Mid levels: natural caves
            println!("Level {}: Natural Cave", depth);
            let config = CaveConfig::default()
                .with_initial_wall_probability(0.45);
            let cave = CaveGenerator::generate(80, 50, &config, rng)?;
            print_cave(&cave);
        }
        _ => {
            // Deep levels: chaotic mines
            println!("Level {}: Abandoned Mine", depth);
            let config = DrunkardConfig::default()
                .with_target_floor_percentage(0.35);
            let mine = DrunkardGenerator::generate(80, 50, &config, rng)?;
            print_drunkard(&mine);
        }
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut rng = Rng::new();

    // Generate levels 1-10
    for depth in 1..=10 {
        generate_level(depth, &mut rng)?;
        println!("\n");
    }

    Ok(())
}
```

## Tutorial 5: Hybrid Maps

Combine multiple algorithms for unique maps.

### Example 1: Dungeon with Cave Sections

```rust
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut rng = Rng::new();

    // Generate BSP dungeon
    let bsp_config = BspConfig::default();
    let mut dungeon = DungeonGenerator::generate(80, 50, &bsp_config, &mut rng);

    // Generate cellular cave
    let cave_config = CaveConfig::default();
    let cave = CaveGenerator::generate(80, 50, &cave_config, &mut rng)?;

    // Overlay cave onto dungeon (30% chance per tile)
    for y in 0..50 {
        for x in 0..80 {
            if cave.is_floor(x, y) && !dungeon.is_floor(x as i32, y as i32) {
                if rng.chance(0.3) {
                    dungeon.set_tile(x as i32, y as i32, true);
                }
            }
        }
    }

    println!("Hybrid dungeon with cave sections:");
    print_dungeon(&dungeon);

    Ok(())
}
```

### Example 2: Cave with Carved Tunnels

```rust
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut rng = Rng::new();

    // Start with cellular cave
    let cave_config = CaveConfig::default();
    let mut cave = CaveGenerator::generate(80, 50, &cave_config, &mut rng)?;

    // Carve drunkard paths for guaranteed connectivity
    let tunnel_config = DrunkardConfig::default()
        .with_target_floor_percentage(0.15)  // Just carve some paths
        .with_start_position(StartPosition::Center);
    let tunnels = DrunkardGenerator::generate(80, 50, &tunnel_config, &mut rng)?;

    // Merge tunnels into cave
    for y in 0..50 {
        for x in 0..80 {
            if tunnels.is_floor(x, y) {
                cave.set_tile(x, y, true);
            }
        }
    }

    println!("Cave with connecting tunnels:");
    print_cave(&cave);

    Ok(())
}
```

## Tips & Tricks

### Tip 1: Deterministic Maps

Use seeded RNG for reproducible maps (e.g., for speedrunning, testing):

```rust
let seed = 12345u64;
let mut rng = Rng::with_seed(seed);

// Always generates the same map
let dungeon = DungeonGenerator::generate(80, 50, &BspConfig::default(), &mut rng);
```

### Tip 2: Add Border Walls

Prevent edge cases by ensuring map boundaries are walls:

```rust
fn add_border_walls(cave: &mut CaveMap) {
    for x in 0..cave.width() {
        cave.set_tile(x, 0, false);
        cave.set_tile(x, cave.height() - 1, false);
    }
    for y in 0..cave.height() {
        cave.set_tile(0, y, false);
        cave.set_tile(cave.width() - 1, y, false);
    }
}
```

### Tip 3: Find Valid Spawn Points

```rust
fn find_floor_tiles(cave: &CaveMap) -> Vec<(u32, u32)> {
    let mut floors = Vec::new();

    for y in 0..cave.height() {
        for x in 0..cave.width() {
            if cave.is_floor(x, y) {
                floors.push((x, y));
            }
        }
    }

    floors
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut rng = Rng::new();
    let cave = CaveGenerator::generate(80, 50, &CaveConfig::default(), &mut rng)?;

    let floor_tiles = find_floor_tiles(&cave);
    let spawn_index = rng.range(0, (floor_tiles.len() - 1) as i32) as usize;
    let (spawn_x, spawn_y) = floor_tiles[spawn_index];

    println!("Player spawns at ({}, {})", spawn_x, spawn_y);

    Ok(())
}
```

### Tip 4: Generate Multiple Candidates

Generate several maps and pick the best:

```rust
fn rate_dungeon(dungeon: &Dungeon) -> f32 {
    let room_count = dungeon.rooms().len() as f32;
    let corridor_count = dungeon.corridors().len() as f32;

    // Prefer dungeons with 8-15 rooms and good connectivity
    let room_score = if room_count >= 8.0 && room_count <= 15.0 {
        1.0
    } else {
        0.5
    };

    let connectivity_score = (corridor_count / room_count).min(1.0);

    room_score * connectivity_score
}

fn main() {
    let mut rng = Rng::new();
    let config = BspConfig::default();

    let mut best_dungeon = None;
    let mut best_score = 0.0;

    // Generate 10 candidates
    for _ in 0..10 {
        let dungeon = DungeonGenerator::generate(80, 50, &config, &mut rng);
        let score = rate_dungeon(&dungeon);

        if score > best_score {
            best_score = score;
            best_dungeon = Some(dungeon);
        }
    }

    println!("Best dungeon (score: {:.2}):", best_score);
    if let Some(dungeon) = best_dungeon {
        print_dungeon(&dungeon);
    }
}
```

### Tip 5: Progressive Difficulty

Adjust algorithm parameters based on depth:

```rust
fn get_config_for_depth(depth: u32) -> CaveConfig {
    CaveConfig::default()
        .with_initial_wall_probability(0.4 + (depth as f32 * 0.02)) // Denser at depth
        .with_iterations(5 + (depth / 3))                            // Smoother at depth
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut rng = Rng::new();

    for depth in 1..=10 {
        let config = get_config_for_depth(depth);
        let cave = CaveGenerator::generate(80, 50, &config, &mut rng)?;

        println!("Depth {} - {:.1}% floor", depth, cave.floor_percentage() * 100.0);
    }

    Ok(())
}
```

## Next Steps

- Explore the [API documentation](https://docs.rs/runeforge-algorithms) for all available methods
- Run the [visual demo](../examples/map_generation_demo.rs) to see all algorithms in action
- Read the [README](README.md) for detailed algorithm explanations
- Check out [runeforge-geometry](../runeforge-geometry) for spatial utilities

Happy dungeon crawling! 🗡️
