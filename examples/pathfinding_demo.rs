//! Pathfinding visualization demo
//!
//! This example demonstrates the A* pathfinding algorithm by showing
//! a character finding paths through a map with obstacles.

use runeforge_geometry::prelude::IVec2;
use runeforge_pathfinding::prelude::{PathFinder, PathProvider};
use std::collections::HashSet;
use std::io::{self, Write};
use std::thread;
use std::time::Duration;

/// Represents the game map
struct Map {
    width: u32,
    height: u32,
    tiles: Vec<bool>, // true = wall, false = floor
}

impl Map {
    /// Creates a new map with a maze-like layout
    fn new(width: u32, height: u32) -> Self {
        let mut tiles = vec![false; (width * height) as usize];

        // Create border walls
        for x in 0..width {
            tiles[x as usize] = true; // Top wall
            tiles[((height - 1) * width + x) as usize] = true; // Bottom wall
        }
        for y in 0..height {
            tiles[(y * width) as usize] = true; // Left wall
            tiles[(y * width + width - 1) as usize] = true; // Right wall
        }

        // Add interior walls to create a maze
        for x in 10..25 {
            if x != 15 && x != 20 {
                // Leave gaps
                tiles[(5 * width + x) as usize] = true;
            }
        }

        for y in 3..12 {
            if y != 7 {
                // Leave a gap
                tiles[(y * width + 25) as usize] = true;
            }
        }

        // Vertical wall with gap
        for y in 8..18 {
            if y != 13 {
                tiles[(y * width + 35) as usize] = true;
            }
        }

        // Some pillars
        tiles[(8 * width + 15) as usize] = true;
        tiles[(12 * width + 18) as usize] = true;
        tiles[(10 * width + 30) as usize] = true;

        Self {
            width,
            height,
            tiles,
        }
    }

    /// Returns true if the position is a wall
    fn is_blocking(&self, pos: IVec2) -> bool {
        if pos.x < 0 || pos.y < 0 || pos.x >= self.width as i32 || pos.y >= self.height as i32 {
            return true; // Out of bounds is blocking
        }
        let index = (pos.y as u32 * self.width + pos.x as u32) as usize;
        self.tiles[index]
    }

    /// Returns true if the position is walkable
    fn is_walkable(&self, pos: IVec2) -> bool {
        !self.is_blocking(pos)
    }
}

/// Implement PathProvider for Map
impl PathProvider<()> for Map {
    fn get_neighbors(&self, pos: IVec2, _data: &mut ()) -> Vec<IVec2> {
        let mut neighbors = Vec::new();
        // Cardinal directions (4-directional)
        let directions = [
            IVec2::new(1, 0),
            IVec2::new(-1, 0),
            IVec2::new(0, 1),
            IVec2::new(0, -1),
        ];
        for dir in directions {
            let new_pos = pos + dir;
            if self.is_walkable(new_pos) {
                neighbors.push(new_pos);
            }
        }
        neighbors
    }

    fn cost(&self, _from: IVec2, to: IVec2, _data: &mut ()) -> u32 {
        if self.is_walkable(to) {
            1
        } else {
            u32::MAX
        }
    }
}

/// Clear the screen and reset cursor
fn clear_screen() {
    print!("\x1B[2J\x1B[H");
    io::stdout().flush().unwrap();
}

/// Render the map with optional path
fn render_map(map: &Map, path: Option<&Vec<IVec2>>, start: IVec2, goal: IVec2) {
    clear_screen();

    // Convert path to a set for fast lookup
    let path_set: HashSet<IVec2> = path
        .map(|p| p.iter().copied().collect())
        .unwrap_or_default();

    for y in 0..map.height {
        for x in 0..map.width {
            let pos = IVec2::new(x as i32, y as i32);
            let is_wall = map.tiles[(y * map.width + x) as usize];

            let (ch, color_code) = if pos == start {
                ('@', "\x1B[33m") // Yellow for start
            } else if pos == goal {
                ('X', "\x1B[31m") // Red for goal
            } else if path_set.contains(&pos) {
                ('·', "\x1B[32m") // Green for path
            } else if is_wall {
                ('#', "\x1B[90m") // Dark gray for walls
            } else {
                ('.', "\x1B[37m") // White for floor
            };

            print!("{}{}\x1B[0m", color_code, ch);
        }
        println!();
    }
    println!();
}

/// 8-directional path provider for comparison
struct Map8Dir {
    map: Map,
}

impl Map8Dir {
    fn new(map: Map) -> Self {
        Self { map }
    }
}

impl PathProvider<()> for Map8Dir {
    fn get_neighbors(&self, pos: IVec2, _data: &mut ()) -> Vec<IVec2> {
        let mut neighbors = Vec::new();
        // 8-directional movement
        for dy in -1..=1 {
            for dx in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }
                let new_pos = pos + IVec2::new(dx, dy);
                if self.map.is_walkable(new_pos) {
                    neighbors.push(new_pos);
                }
            }
        }
        neighbors
    }

    fn cost(&self, from: IVec2, to: IVec2, _data: &mut ()) -> u32 {
        if self.map.is_walkable(to) {
            // Diagonal movement costs more
            if from.x != to.x && from.y != to.y {
                14 // Approximate sqrt(2) * 10
            } else {
                10
            }
        } else {
            u32::MAX
        }
    }
}

fn main() -> io::Result<()> {
    println!("\n=== Runeforge Pathfinding Demo ===\n");
    println!("This demo shows the A* pathfinding algorithm finding paths through a maze.");
    println!("Legend:");
    println!("  @ = Start position (yellow)");
    println!("  X = Goal position (red)");
    println!("  · = Path (green)");
    println!("  # = Walls (gray)");
    println!("  . = Walkable floor (white)");
    println!("\nPress Enter to begin...");
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let mut map = Map::new(50, 20);

    // Demo 1: Simple path
    println!("\n--- Demo 1: Simple 4-directional path ---");
    let start1 = IVec2::new(2, 2);
    let goal1 = IVec2::new(8, 2);

    render_map(&map, None, start1, goal1);
    println!("Finding path from @ to X...");
    thread::sleep(Duration::from_secs(1));

    let path1: Vec<IVec2> = PathFinder::Astar.compute(start1, goal1, &mut map, ());
    render_map(&map, Some(&path1), start1, goal1);
    println!("Path found! Length: {} steps", path1.len());
    thread::sleep(Duration::from_secs(2));

    // Demo 2: Path around obstacles
    println!("\n--- Demo 2: Navigating around walls ---");
    let start2 = IVec2::new(2, 2);
    let goal2 = IVec2::new(22, 2);

    render_map(&map, None, start2, goal2);
    println!("Finding path around the wall...");
    thread::sleep(Duration::from_secs(1));

    let path2: Vec<IVec2> = PathFinder::Astar.compute(start2, goal2, &mut map, ());
    render_map(&map, Some(&path2), start2, goal2);
    println!("Path found! Length: {} steps", path2.len());
    println!("Notice how A* finds the optimal route around the obstacles.");
    thread::sleep(Duration::from_secs(3));

    // Demo 3: Complex maze navigation
    println!("\n--- Demo 3: Complex maze navigation ---");
    let start3 = IVec2::new(2, 2);
    let goal3 = IVec2::new(40, 15);

    render_map(&map, None, start3, goal3);
    println!("Finding path through the complex maze...");
    thread::sleep(Duration::from_secs(1));

    let path3: Vec<IVec2> = PathFinder::Astar.compute(start3, goal3, &mut map, ());
    render_map(&map, Some(&path3), start3, goal3);
    println!("Path found! Length: {} steps", path3.len());
    println!("A* efficiently navigates through multiple obstacles!");
    thread::sleep(Duration::from_secs(3));

    // Demo 4: 8-directional vs 4-directional
    println!("\n--- Demo 4: Comparing 4-directional vs 8-directional ---");
    let start4 = IVec2::new(2, 10);
    let goal4 = IVec2::new(10, 18);

    render_map(&map, None, start4, goal4);
    println!("4-directional path (cardinal directions only)...");
    thread::sleep(Duration::from_secs(1));

    let path4_4dir: Vec<IVec2> = PathFinder::Astar.compute(start4, goal4, &mut map, ());
    render_map(&map, Some(&path4_4dir), start4, goal4);
    println!("4-directional path length: {} steps", path4_4dir.len());
    thread::sleep(Duration::from_secs(2));

    println!("\nNow trying 8-directional (with diagonals)...");
    thread::sleep(Duration::from_secs(1));

    // Create a new map for 8-directional demo (don't consume the original map)
    let mut map8 = Map8Dir::new(Map::new(50, 20));
    let path4_8dir: Vec<IVec2> = PathFinder::Astar.compute(start4, goal4, &mut map8, ());
    render_map(&map8.map, Some(&path4_8dir), start4, goal4);
    println!("8-directional path length: {} steps", path4_8dir.len());
    println!(
        "8-directional saved {} steps by cutting corners!",
        path4_4dir.len() - path4_8dir.len()
    );
    thread::sleep(Duration::from_secs(3));

    // Demo 5: No path scenario
    println!("\n--- Demo 5: When there's no path ---");
    let start5 = IVec2::new(2, 2);
    let goal5 = IVec2::new(40, 2); // Behind walls with no gap

    render_map(&map, None, start5, goal5);
    println!("Trying to find a path to an unreachable location...");
    thread::sleep(Duration::from_secs(1));

    let path5: Vec<IVec2> = PathFinder::Astar.compute(start5, goal5, &mut map, ());
    render_map(&map, Some(&path5), start5, goal5);
    if path5.is_empty() {
        println!("No path found! A* correctly identified this goal is unreachable.");
    }
    thread::sleep(Duration::from_secs(2));

    println!("\n=== Demo Complete ===\n");
    println!("Key Takeaways:");
    println!("  • A* finds optimal paths efficiently");
    println!("  • Manhattan distance heuristic guides 4-directional search");
    println!("  • Chebyshev distance heuristic guides 8-directional search");
    println!("  • A* correctly handles unreachable goals");
    println!("  • 8-directional movement can be more efficient when allowed\n");

    Ok(())
}
