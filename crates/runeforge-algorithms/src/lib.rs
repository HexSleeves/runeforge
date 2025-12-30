//! Procedural generation algorithms for roguelike games.
//!
//! This crate provides map generation algorithms including:
//! - **BSP (Binary Space Partitioning)**: Structured dungeons with rooms and corridors
//! - **Cellular Automata**: Organic cave systems
//! - **Drunkard's Walk**: Irregular cave-like tunnels
//!
//! # Example
//!
//! ```
//! use runeforge_algorithms::bsp::{BspConfig, DungeonGenerator};
//! use runeforge_algorithms::caves::{CaveConfig, CaveGenerator};
//! use runeforge_random::Rng;
//!
//! // Generate a structured BSP dungeon
//! let config = BspConfig::default();
//! let mut rng = Rng::new();
//! let dungeon = DungeonGenerator::generate(80, 50, &config, &mut rng);
//!
//! // Generate organic caves with cellular automata
//! let cave_config = CaveConfig::default();
//! let cave = CaveGenerator::generate(80, 50, &cave_config, &mut rng);
//! ```

#![deny(missing_docs)]

pub mod bsp;
pub mod caves;

// Re-export commonly used types
pub use bsp::{BspConfig, BspNode, Corridor, Dungeon, DungeonGenerator, SplitDirection};
pub use caves::{CaveConfig, CaveGenerator, CaveMap};
