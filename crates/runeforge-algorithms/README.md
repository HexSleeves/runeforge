# Runeforge Algorithms

Procedural map generation algorithms for roguelike games, written in Rust.

## Features

This crate provides three powerful procedural generation algorithms:

- **BSP (Binary Space Partitioning)**: Creates structured dungeons with rectangular rooms and connecting corridors
- **Cellular Automata**: Generates organic, cave-like environments using iterative neighbor rules
- **Drunkard's Walk**: Produces irregular, winding tunnel systems through random walking

## Quick Start

Add to your `Cargo.toml`:

```toml
[dependencies]
runeforge-algorithms = "0.1.0"
runeforge-random = "0.1.0"
```

### Basic Example

```rust
use runeforge_algorithms::{DungeonGenerator, BspConfig};
use runeforge_random::Rng;

fn main() {
    let config = BspConfig::default();
    let mut rng = Rng::new();
    let dungeon = DungeonGenerator::generate(80, 50, &config, &mut rng);

    // Print the dungeon
    for y in 0..dungeon.height() {
        for x in 0..dungeon.width() {
            let ch = if dungeon.is_floor(x as i32, y as i32) { '.' } else { '#' };
            print!("{}", ch);
        }
        println!();
    }
}
```

## Algorithms

### 1. BSP Dungeons

**Best for**: Traditional roguelike dungeons, structured environments, predictable layouts

Binary Space Partitioning creates dungeons by recursively dividing space into smaller regions, placing rooms in the leaf nodes, and connecting them with corridors.

**Characteristics**:

- Rectangular, grid-aligned rooms
- Guaranteed connectivity between all rooms
- Predictable, organized layouts
- Configurable room sizes and spacing

**Example**:

```rust
use runeforge_algorithms::{BspConfig, DungeonGenerator};
use runeforge_random::Rng;

let config = BspConfig::default()
    .with_min_room_size(5, 5)       // Minimum room dimensions
    .with_max_depth(5)              // How many times to subdivide
    .with_room_padding(2);          // Space between rooms and partition edges

let mut rng = Rng::new();
let dungeon = DungeonGenerator::generate(80, 50, &config, &mut rng);

// Access generated rooms
for room in dungeon.rooms() {
    println!("Room at ({}, {}) with size {}x{}",
        room.x, room.y, room.width, room.height);
}

// Access corridors
for corridor in dungeon.corridors() {
    println!("Corridor from {:?} to {:?}", corridor.start, corridor.end);
}
```

**Configuration Options**:

| Parameter | Default | Description |
| ----------- | ------- | ----------- |
| `min_partition_width` | 10 | Minimum width for BSP subdivisions |
| `min_partition_height` | 10 | Minimum height for BSP subdivisions |
| `min_room_width` | 4 | Minimum width for generated rooms |
| `min_room_height` | 4 | Minimum height for generated rooms |
| `room_padding` | 1 | Space between room edges and partition boundaries |
| `max_depth` | 5 | Maximum recursion depth for subdivision |
| `split_ratio_min` | 0.4 | Minimum split ratio (40% / 60%) |
| `split_ratio_max` | 0.6 | Maximum split ratio (60% / 40%) |

### 2. Cellular Automata Caves

**Best for**: Natural caves, organic environments, smooth formations

Cellular Automata generates caves through iterative simulation. It starts with random noise and applies neighbor-based rules to create smooth, natural-looking cave systems.

**Characteristics**:

- Organic, irregular shapes
- Smooth, rounded edges
- Natural-looking formations
- Highly configurable density

**Example**:

```rust
use runeforge_algorithms::{CaveConfig, CaveGenerator};
use runeforge_random::Rng;

let config = CaveConfig::default()
    .with_initial_wall_probability(0.45)  // 45% walls, 55% floors initially
    .with_iterations(5)                   // Number of simulation steps
    .with_wall_threshold(5);              // Cells with ≥5 wall neighbors become walls

let mut rng = Rng::new();
let cave = CaveGenerator::generate(80, 50, &config, &mut rng)?;

// Check floor percentage
println!("Cave has {:.1}% open space", cave.floor_percentage() * 100.0);

// Check individual tiles
if cave.is_floor(10, 10) {
    println!("Position (10, 10) is open floor");
}
```

**How it Works**:

1. **Initialization**: Randomly place walls based on `initial_wall_probability`
2. **Simulation**: For each iteration:
   - Count wall neighbors for each cell (Moore neighborhood: 8 surrounding cells)
   - Apply rule: if wall_neighbors >= `wall_threshold`, cell becomes wall; otherwise, floor
3. **Result**: After iterations, smooth organic caves emerge

**Configuration Options**:

| Parameter | Default | Description |
| ----------- | ------- | ----------- |
| `initial_wall_probability` | 0.45 | Probability of a tile starting as a wall (0.0-1.0) |
| `iterations` | 5 | Number of simulation iterations |
| `wall_threshold` | 5 | Neighbor count threshold (0-8 for Moore neighborhood) |

**Tuning Tips**:

- **More open caves**: Lower `initial_wall_probability` (0.35-0.40) or `wall_threshold` (3-4)
- **Denser caves**: Higher `initial_wall_probability` (0.50-0.55) or `wall_threshold` (6-7)
- **Smoother caves**: More `iterations` (7-10)
- **Rougher caves**: Fewer `iterations` (2-3)

### 3. Drunkard's Walk Caves

**Best for**: Winding tunnels, irregular caves, exploratory paths

Drunkard's Walk creates caves by simulating a random walker carving floor tiles as it moves. The walker takes random steps in cardinal directions until reaching a target floor percentage.

**Characteristics**:

- Meandering, unpredictable paths
- Variable tunnel width and density
- Guaranteed connectivity (single contiguous cave)
- Configurable coverage

**Example**:

```rust
use runeforge_algorithms::{DrunkardConfig, DrunkardGenerator, StartPosition};
use runeforge_random::Rng;

let config = DrunkardConfig::default()
    .with_target_floor_percentage(0.4)          // Stop when 40% is floor
    .with_start_position(StartPosition::Center) // Start from center
    .with_max_steps(100_000);                   // Safety limit

let mut rng = Rng::new();
let cave = DrunkardGenerator::generate(80, 50, &config, &mut rng)?;

println!("Generated cave with {:.1}% open space",
    cave.floor_percentage() * 100.0);
```

**Configuration Options**:

| Parameter | Default | Description |
| ----------- | ------- | ----------- |
| `target_floor_percentage` | 0.4 | Stop when this percentage of map is floor (0.0-1.0) |
| `start_position` | `Center` | Where walker starts: `Center` or `Random` |
| `max_steps` | 100,000 | Maximum steps before giving up (prevents infinite loops) |

**Start Position Variants**:

```rust
use runeforge_algorithms::StartPosition;

// Start from map center
let config = DrunkardConfig::default()
    .with_start_position(StartPosition::Center);

// Start from random position
let config = DrunkardConfig::default()
    .with_start_position(StartPosition::Random);
```

## Comparison

| Algorithm | Structure | Connectivity | Use Case | Complexity |
| ----------- | ------- | ----------- | ----------- | ----------- |
| **BSP** | Regular, grid-aligned | Guaranteed via corridors | Traditional dungeons, structured levels | Medium |
| **Cellular Automata** | Organic, irregular | May have disconnected regions | Natural caves, outdoor areas | Low |
| **Drunkard's Walk** | Winding, variable density | Single contiguous path | Tunnels, mines, exploratory caves | Very Low |

## Visual Demo

Run the included example to see all three algorithms in action:

```bash
cargo run --example map_generation_demo --package runeforge-algorithms
```

This will display:

1. A full-size BSP dungeon
2. A cellular automata cave
3. A drunkard's walk cave
4. A side-by-side comparison of all three

## Combining Algorithms

You can layer multiple algorithms for more interesting maps:

```rust
use runeforge_algorithms::*;
use runeforge_random::Rng;

let mut rng = Rng::new();

// Start with BSP dungeon
let mut dungeon = DungeonGenerator::generate(80, 50, &BspConfig::default(), &mut rng);

// Generate cellular caves
let caves = CaveGenerator::generate(80, 50, &CaveConfig::default(), &mut rng)?;

// Overlay caves onto dungeon (custom merging logic)
for y in 0..80 {
    for x in 0..50 {
        // If cave is floor and dungeon is wall, maybe carve it
        if caves.is_floor(x, y) && !dungeon.is_floor(x as i32, y as i32) {
            if rng.chance(0.3) { // 30% chance to carve
                dungeon.set_tile(x as i32, y as i32, true);
            }
        }
    }
}
```

## Advanced Usage

### Deterministic Generation

Use seeded RNG for reproducible maps:

```rust
use runeforge_random::Rng;

let seed = 12345u64;
let mut rng = Rng::with_seed(seed);

// Same seed = same map
let dungeon1 = DungeonGenerator::generate(80, 50, &BspConfig::default(), &mut rng);

let mut rng2 = Rng::with_seed(seed);
let dungeon2 = DungeonGenerator::generate(80, 50, &BspConfig::default(), &mut rng2);

// dungeon1 and dungeon2 are identical
```

### Post-Processing

Apply transformations after generation:

```rust
// Add border walls
for x in 0..cave.width() {
    cave.set_tile(x, 0, false);
    cave.set_tile(x, cave.height() - 1, false);
}
for y in 0..cave.height() {
    cave.set_tile(0, y, false);
    cave.set_tile(cave.width() - 1, y, false);
}

// Remove small isolated regions (flood fill from main area)
// Add decorative features (pillars, pools, etc.)
```

## Error Handling

All generators return `Result` types for proper error handling:

```rust
match DrunkardGenerator::generate(0, 50, &config, &mut rng) {
    Ok(cave) => println!("Generated successfully!"),
    Err(DrunkardError::InvalidDimensions { width, height }) => {
        eprintln!("Invalid dimensions: {}x{}", width, height);
    }
}
```

## Performance

Benchmarks on an M1 MacBook Pro:

| Algorithm | Map Size | Generation Time |
| ----------- | ------- | ----------- |
| BSP | 80x50 | ~0.5ms |
| Cellular Automata (5 iterations) | 80x50 | ~2ms |
| Drunkard's Walk (40% coverage) | 80x50 | ~1ms |

Larger maps scale linearly with area.

## License

This crate is part of the Runeforge library and shares its license.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
