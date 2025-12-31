//! BSP Dungeon Generation Demo
//!
//! This example demonstrates the Binary Space Partitioning (BSP) algorithm
//! for generating dungeon layouts with rooms and connecting corridors.

use runeforge_algorithms::prelude::{BspConfig, DungeonGenerator};
use runeforge_random::prelude::Rng;
use std::io::{self, Write};
use std::thread;
use std::time::Duration;

/// Clear the screen and reset cursor
fn clear_screen() {
    print!("\x1B[2J\x1B[H");
    io::stdout().flush().unwrap();
}

/// Render the dungeon to the terminal
fn render_dungeon(dungeon: &runeforge_algorithms::prelude::Dungeon, title: &str) {
    clear_screen();
    println!("\x1B[1;36m{}\x1B[0m\n", title);

    for y in 0..dungeon.height() {
        for x in 0..dungeon.width() {
            let ch = if dungeon.is_floor(x as i32, y as i32) {
                // Check if this is a room corner for visualization
                '.'
            } else {
                '#'
            };

            // Color based on tile type
            let color = if dungeon.is_floor(x as i32, y as i32) {
                "\x1B[33m" // Yellow for floor
            } else {
                "\x1B[90m" // Dark gray for walls
            };

            print!("{}{}\x1B[0m", color, ch);
        }
        println!();
    }

    println!();
    println!(
        "\x1B[32mRooms: {}\x1B[0m | \x1B[34mCorridors: {}\x1B[0m",
        dungeon.rooms().len(),
        dungeon.corridors().len()
    );
}

fn main() -> io::Result<()> {
    println!("\n\x1B[1;35m=== Runeforge BSP Dungeon Generation Demo ===\x1B[0m\n");
    println!("Binary Space Partitioning (BSP) is a technique that recursively");
    println!("divides space into smaller partitions, then creates rooms in");
    println!("the leaf nodes and connects them with corridors.\n");
    println!("Key features:");
    println!("  • Guarantees non-overlapping rooms");
    println!("  • All rooms are connected");
    println!("  • Configurable room sizes and split ratios\n");
    println!("Press Enter to see different dungeon configurations...");

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    // Demo 1: Default configuration
    let config1 = BspConfig::default();
    let mut rng = Rng::new();
    let dungeon1 = DungeonGenerator::generate(60, 25, &config1, &mut rng);
    render_dungeon(&dungeon1, "Demo 1: Default Configuration (60x25)");
    println!("\nDefault settings create balanced, medium-sized rooms.");
    thread::sleep(Duration::from_secs(3));

    // Demo 2: Smaller rooms, more splits
    let config2 = BspConfig::new()
        .with_min_partition_size(8, 8)
        .with_min_room_size(3, 3)
        .with_max_depth(6);
    let dungeon2 = DungeonGenerator::generate(60, 25, &config2, &mut rng);
    render_dungeon(&dungeon2, "Demo 2: Many Small Rooms (depth=6, min=3x3)");
    println!("\nIncreasing depth and reducing minimum sizes creates more rooms.");
    thread::sleep(Duration::from_secs(3));

    // Demo 3: Larger rooms, fewer splits
    let config3 = BspConfig::new()
        .with_min_partition_size(20, 15)
        .with_min_room_size(8, 6)
        .with_max_depth(3);
    let dungeon3 = DungeonGenerator::generate(60, 25, &config3, &mut rng);
    render_dungeon(&dungeon3, "Demo 3: Few Large Rooms (depth=3, min=8x6)");
    println!("\nReducing depth and increasing minimums creates fewer, larger rooms.");
    thread::sleep(Duration::from_secs(3));

    // Demo 4: Uneven split ratios
    let config4 = BspConfig::new()
        .with_split_ratio(0.3, 0.7)
        .with_max_depth(5);
    let dungeon4 = DungeonGenerator::generate(60, 25, &config4, &mut rng);
    render_dungeon(&dungeon4, "Demo 4: Varied Room Sizes (ratio 0.3-0.7)");
    println!("\nWider split ratio range creates more variety in room sizes.");
    thread::sleep(Duration::from_secs(3));

    // Demo 5: Large dungeon
    let config5 = BspConfig::new()
        .with_min_partition_size(12, 10)
        .with_min_room_size(5, 4)
        .with_max_depth(5);
    let dungeon5 = DungeonGenerator::generate(80, 30, &config5, &mut rng);
    render_dungeon(&dungeon5, "Demo 5: Large Dungeon (80x30)");
    println!("\nLarger dungeons with appropriate settings for roguelike games.");
    thread::sleep(Duration::from_secs(3));

    // Final stats
    clear_screen();
    println!("\x1B[1;35m=== Demo Complete ===\x1B[0m\n");
    println!("BSP dungeon generation provides:");
    println!("  ✓ Guaranteed room connectivity");
    println!("  ✓ Non-overlapping rooms");
    println!("  ✓ Configurable room sizes and counts");
    println!("  ✓ Reproducible results with seeded RNG\n");

    println!("Configuration options:");
    println!("  • min_partition_size: Minimum space before stopping splits");
    println!("  • min_room_size: Smallest allowed room dimensions");
    println!("  • max_depth: How many times to recursively split");
    println!("  • split_ratio: Range for where to place split lines");
    println!("  • room_padding: Space between rooms and partition edges\n");

    Ok(())
}
