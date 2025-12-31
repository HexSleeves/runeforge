//! # Runeforge 🔥⚔️
//!
//! ## A modern, modular roguelike library for Rust
//!
//! `runeforge` is a pure Rust roguelike library inspired by [libtcod](https://github.com/libtcod/libtcod).
//! It provides a comprehensive suite of tools for building roguelike games, with a focus on modularity,
//! performance, and ease of use.
//!
//! This crate is the main facade, re-exporting all the functionality of the `runeforge` ecosystem.
//!
//! ## Core Principles
//!
//! -   **🦀 Pure Rust**: No C dependencies, for easy and fast builds.
//! -   **📦 Modular**: The library is split into small, focused crates. You only use what you need.
//! -   **⚡ Fast**: Backends can be accelerated with GPU support (`wgpu`).
//! -   **📚 Well-documented**: Aims for clear documentation and practical examples.
//!
//! ## Quick Start
//!
//! To get started, add `runeforge` to your `Cargo.toml`, enabling the features you need.
//!
//! ```toml
//! [dependencies]
//! runeforge = { version = "0.1.0", features = ["terminal", "algorithms", "fov"] }
//! ```
//!
//! Then, use the prelude to get started quickly:
//!
//! ```rust,no_run
//! use runeforge_rl::prelude::*;
//!
//! fn main() {
//!     // From `runeforge-random`
//!     let mut rng = Rng::new();
//!     let roll = rng.roll("1d6+2").unwrap();
//!     println!("You rolled a {}", roll);
//!
//!     // From `runeforge-geometry`
//!     let point = IVec2::new(10, 20);
//!
//!     // From `runeforge-color`
//!     let color = Color::rgb(255, 0, 255);
//!
//!     println!("A point at {:?} with color {}", point, color);
//! }
//! ```
//! ## Available Features
//!
//! `runeforge` is highly modular. You can enable features for the functionality you need:
//!
//! -   `algorithms`: Procedural generation (BSP, Cellular Automata).
//! -   `direction`: Grid-based direction handling.
//! -   `fov`: Field-of-view algorithms.
//! -   `input`: Action-based input mapping.
//! -   `noise`: Perlin noise generation.
//! -   `pathfinding`: A* and other pathfinding algorithms.
//! -   `terminal`: A complete rendering solution with multiple backends.
//! -   `tileset`: Loading for graphical tilesets and fonts.
//! -   `full`: Enables all features.
//!

#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)]
#![warn(clippy::dbg_macro, clippy::todo, clippy::unimplemented)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::multiple_crate_versions)] // Transitive deps from wgpu/pixels
#![deny(missing_docs)]

// Re-export core types (always available)
pub use runeforge_color as color;
pub use runeforge_geometry as geometry;
pub use runeforge_random as random;

// Optional feature-gated crates
#[cfg(feature = "algorithms")]
pub use runeforge_algorithms as algorithms;

#[cfg(feature = "direction")]
pub use runeforge_direction as direction;

#[cfg(feature = "fov")]
pub use runeforge_fov as fov;

#[cfg(feature = "input")]
pub use runeforge_input as input;

#[cfg(feature = "noise")]
pub use runeforge_noise as noise;

#[cfg(feature = "pathfinding")]
pub use runeforge_pathfinding as pathfinding;

#[cfg(feature = "terminal")]
pub use runeforge_terminal as terminal;

#[cfg(feature = "tileset")]
pub use runeforge_tileset as tileset;

/// Prelude module for convenient imports.
pub mod prelude {
    // Core types (always available)
    pub use crate::color::Color;
    pub use crate::geometry::prelude::*;
    pub use crate::random::prelude::*;

    // Optional feature-gated modules
    #[cfg(feature = "algorithms")]
    pub use crate::algorithms::prelude::*;

    #[cfg(feature = "fov")]
    pub use crate::fov::prelude::*;

    #[cfg(feature = "input")]
    pub use crate::input::*;

    #[cfg(feature = "noise")]
    pub use crate::noise::*;

    #[cfg(feature = "pathfinding")]
    pub use crate::pathfinding::prelude::*;

    #[cfg(feature = "terminal")]
    pub use crate::terminal::prelude::*;

    #[cfg(feature = "tileset")]
    pub use crate::tileset::prelude::*;
}
