//! Visual demonstration of all procedural generation algorithms.
//!
//! Run with: `cargo run --example map_generation_demo`

use runeforge_algorithms::{
    BspConfig, CaveConfig, CaveGenerator, DrunkardConfig, DrunkardGenerator, DungeonGenerator,
    StartPosition,
};
use runeforge_random::Rng;

const MAP_WIDTH: u32 = 80;
const MAP_HEIGHT: u32 = 40;

fn main() {
    println!("{}", "=".repeat(MAP_WIDTH as usize));
    println!("Runeforge Procedural Generation Demo");
    println!("{}", "=".repeat(MAP_WIDTH as usize));
    println!();

    let mut rng = Rng::with_seed(12345);

    // Demo 1: BSP Dungeon
    demo_bsp_dungeon(&mut rng);
    println!();

    // Demo 2: Cellular Automata Caves
    demo_cellular_automata(&mut rng);
    println!();

    // Demo 3: Drunkard's Walk Caves
    demo_drunkard_walk(&mut rng);
    println!();

    // Demo 4: Side-by-side comparison
    demo_comparison(&mut rng);
}

fn demo_bsp_dungeon(rng: &mut Rng) {
    println!("1. BSP (Binary Space Partitioning) Dungeon");
    println!("   - Structured rooms connected by corridors");
    println!("   - Great for traditional dungeon layouts");
    println!("{}", "-".repeat(MAP_WIDTH as usize));

    let config = BspConfig::default()
        .with_min_room_size(4, 4)
        .with_max_depth(5);

    let dungeon = DungeonGenerator::generate(MAP_WIDTH, MAP_HEIGHT, &config, rng);

    // Print the dungeon
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

    println!();
    println!(
        "   Stats: {} rooms, {} corridors",
        dungeon.rooms().len(),
        dungeon.corridors().len()
    );
}

fn demo_cellular_automata(rng: &mut Rng) {
    println!("2. Cellular Automata Caves");
    println!("   - Organic, natural-looking caves");
    println!("   - Smooth, rounded formations");
    println!("{}", "-".repeat(MAP_WIDTH as usize));

    let config = CaveConfig::default()
        .with_initial_wall_probability(0.45)
        .with_iterations(5)
        .with_wall_threshold(5);

    let cave = CaveGenerator::generate(MAP_WIDTH, MAP_HEIGHT, &config, rng).unwrap();

    // Print the cave
    for y in 0..cave.height() {
        for x in 0..cave.width() {
            let ch = if cave.is_floor(x, y) { '.' } else { '#' };
            print!("{}", ch);
        }
        println!();
    }

    println!();
    println!("   Stats: {:.1}% floor", cave.floor_percentage() * 100.0);
}

fn demo_drunkard_walk(rng: &mut Rng) {
    println!("3. Drunkard's Walk Caves");
    println!("   - Irregular, winding tunnels");
    println!("   - Organic and unpredictable");
    println!("{}", "-".repeat(MAP_WIDTH as usize));

    let config = DrunkardConfig::default()
        .with_target_floor_percentage(0.4)
        .with_start_position(StartPosition::Center);

    let cave = DrunkardGenerator::generate(MAP_WIDTH, MAP_HEIGHT, &config, rng).unwrap();

    // Print the cave
    for y in 0..cave.height() {
        for x in 0..cave.width() {
            let ch = if cave.is_floor(x, y) { '.' } else { '#' };
            print!("{}", ch);
        }
        println!();
    }

    println!();
    println!("   Stats: {:.1}% floor", cave.floor_percentage() * 100.0);
}

fn demo_comparison(rng: &mut Rng) {
    println!("4. Side-by-Side Comparison (30x20 each)");
    println!("{}", "=".repeat(MAP_WIDTH as usize));

    let small_width = 30;
    let small_height = 20;

    // Generate all three
    let bsp_config = BspConfig::default().with_min_room_size(3, 3);
    let bsp = DungeonGenerator::generate(small_width, small_height, &bsp_config, rng);

    let cave_config = CaveConfig::default();
    let cellular = CaveGenerator::generate(small_width, small_height, &cave_config, rng).unwrap();

    let drunkard_config = DrunkardConfig::default().with_target_floor_percentage(0.35);
    let drunkard =
        DrunkardGenerator::generate(small_width, small_height, &drunkard_config, rng).unwrap();

    println!("BSP Dungeon              Cellular Automata        Drunkard's Walk");
    println!("{}", "-".repeat(MAP_WIDTH as usize));

    // Print side by side
    for y in 0..small_height {
        // BSP
        for x in 0..small_width {
            print!(
                "{}",
                if bsp.is_floor(x as i32, y as i32) {
                    '.'
                } else {
                    '#'
                }
            );
        }
        print!("  ");

        // Cellular
        for x in 0..small_width {
            print!("{}", if cellular.is_floor(x, y) { '.' } else { '#' });
        }
        print!("  ");

        // Drunkard
        for x in 0..small_width {
            print!("{}", if drunkard.is_floor(x, y) { '.' } else { '#' });
        }
        println!();
    }

    println!();
    println!(
        "{} rooms/corridors      Floor: {:.1}%                Floor: {:.1}%",
        bsp.rooms().len() + bsp.corridors().len(),
        cellular.floor_percentage() * 100.0,
        drunkard.floor_percentage() * 100.0
    );
}
