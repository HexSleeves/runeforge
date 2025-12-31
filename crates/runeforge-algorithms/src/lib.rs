//! Procedural generation algorithms for roguelike maps.
//!
//! # Overview
//!
//! `runeforge-algorithms` provides a collection of classic algorithms for generating game maps.
//! These are essential for creating varied and interesting level layouts automatically.
//!
//! # Algorithms
//!
//! *   **BSP (Binary Space Partitioning):** Creates structured dungeons with rooms and corridors.
//! *   **Cellular Automata:** Generates organic, cave-like systems.
//! *   **Drunkard's Walk:** Produces irregular, winding cave tunnels.
//!
//! # Usage
//!
//! Add this to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! runeforge-algorithms = "0.1"
//! ```
//!
//! ## BSP Dungeon Example
//!
//! ```rust
//! use runeforge_algorithms::prelude::*;
//! use runeforge_random::prelude::Rng;
//!
//! fn main() {
//!     let config = BspConfig::default();
//!     let mut rng = Rng::new();
//!     let dungeon = DungeonGenerator::generate(80, 50, &config, &mut rng);
//!     
//!     // dungeon.rooms() returns an iterator over the rooms
//!     for room in dungeon.rooms() {
//!         // Draw room...
//!         println!("Found a room: {:?}", room);
//!     }
//! }
//! ```
//!
//! ## Cellular Automata Cave Example
//!
//! ```rust
//! use runeforge_algorithms::prelude::*;
//! use runeforge_random::prelude::Rng;
//!
//! fn main() {
//!     let cave_config = CaveConfig::default();
//!     let mut rng = Rng::new();
//!     let cave = CaveGenerator::generate(80, 50, &cave_config, &mut rng);
//!     
//!     // cave.map is a Vec<bool> where true represents a wall
//! }
//! ```
pub mod bsp;
pub mod caves;
pub mod drunkard;

pub mod prelude {
    pub use runeforge_random::prelude::Rng;

    pub use crate::bsp::{BspConfig, BspNode, Corridor, Dungeon, DungeonGenerator, SplitDirection};
    pub use crate::caves::{CaveConfig, CaveGenerator, CaveMap};
    pub use crate::drunkard::{DrunkardConfig, DrunkardGenerator, DrunkardMap, StartPosition};
}
