//! Runeforge: A modern Rust roguelike library.
//!
//! This is the facade crate that re-exports all Runeforge sub-crates.
//! Use this crate to get access to the complete Runeforge API.
//!
//! # Quick Start
//!
//! ```rust,no_run
//! use runeforge_core::prelude::*;
//!
//! fn main() {
//!     let mut rng = Rng::new();
//!     let point = Point::new(10, 20);
//!     let color = Color::RED;
//!
//!     println!("Random d6 roll: {}", rng.roll_dice(1, 6));
//!     println!("Point: {:?}", point);
//!     println!("Color: {}", color);
//! }
//! ```

#![deny(missing_docs)]

// Re-export core types
pub use runeforge_color as color;
pub use runeforge_geometry as geometry;
pub use runeforge_random as random;

#[cfg(feature = "noise")]
pub use runeforge_noise as noise;

#[cfg(feature = "pathfinding")]
pub use runeforge_pathfinding as pathfinding;

#[cfg(feature = "fov")]
pub use runeforge_fov as fov;

#[cfg(feature = "bsp")]
pub use runeforge_bsp as bsp;

#[cfg(feature = "algorithms")]
pub use runeforge_algorithms as algorithms;

#[cfg(feature = "terminal")]
pub use runeforge_terminal as terminal;

#[cfg(feature = "tileset")]
pub use runeforge_tileset as tileset;

#[cfg(feature = "input")]
pub use runeforge_input as input;

/// Prelude module for convenient imports.
pub mod prelude {
    pub use crate::color::Color;
    pub use crate::geometry::{Point, Rect};
    pub use crate::random::{parse_dice, roll_dice_notation, Rng};

    #[cfg(feature = "fov")]
    pub use crate::fov;

    #[cfg(feature = "pathfinding")]
    pub use crate::pathfinding;

    #[cfg(feature = "noise")]
    pub use crate::noise;

    #[cfg(feature = "bsp")]
    pub use crate::bsp;

    #[cfg(feature = "algorithms")]
    pub use crate::algorithms;

    #[cfg(feature = "terminal")]
    pub use crate::terminal;

    #[cfg(feature = "input")]
    pub use crate::input;
}
