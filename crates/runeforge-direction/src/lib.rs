//! Grid-based direction handling for roguelikes and 2D games.
//!
//! # Overview
//!
//! `runeforge-direction` provides robust types and utilities for managing directions on a 2D grid.
//! It is designed to handle common roguelike movement patterns including cardinal (4-way) and
//! ordinal (8-way) directions, as well as 3D verticality (Up/Down).
//!
//! # Key Features
//!
//! *   **Bitflag Implementation:** Directions are implemented as bitflags, allowing for efficient combination and masking.
//! *   **Flexible Types:** Supports `CardinalDirection`, `OrdinalDirection`, and general `Direction`.
//! *   **Arithmetic Operations:** Add/Subtract directions (e.g., `NORTH + EAST = NORTH_EAST`).
//! *   **Coordinate Conversion:** Easily convert between Directions and `IVec2`/`IVec3` vectors.
//! *   **Iterators:** Efficiently iterate over all neighbors or specific subsets of directions.
//!
//! # Usage
//!
//! Add this to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! runeforge-direction = "0.1"
//! ```
//!
//! ## Basic Example
//!
//! ```rust
//! use runeforge_direction::prelude::*;
//!
//! fn main() {
//!     // Use predefined constants
//!     let dir = Direction::NORTH;
//!     
//!     // Combine directions
//!     let move_dir = Direction::NORTH + Direction::EAST;
//!     assert_eq!(move_dir, Direction::NORTH_EAST);
//!     
//!     // Convert to coordinate vector
//!     let delta = move_dir.coord();
//!     assert_eq!(delta.x, 1);
//!     assert_eq!(delta.y, 1);
//!     
//!     // Iterate over neighbors
//!     for d in Direction::all() {
//!         println!("Neighbor: {:?}", d);
//!     }
//! }
//! ```
//!
//! # Modules
//!
//! *   [`direction`]: The core `Direction` type and its methods.
//! *   [`cardinal_direction`]: Helpers for 4-way movement.
//! *   [`ordinal_direction`]: Helpers for diagonal movement.
//! *   [`direction_iter`]: Iterators for direction sets.

pub mod cardinal_direction;
pub mod direction;
pub mod direction_flags;
pub mod direction_iter;
pub mod direction_iterator;
pub mod direction_table;
pub mod direction_type;
pub mod ordinal_direction;
pub mod vertical_direction;

pub mod prelude {
    pub use glam::{IVec2, IVec3};

    pub use crate::cardinal_direction::*;
    pub use crate::direction::*;
    pub use crate::direction_flags::*;
    pub use crate::direction_iter::*;
    pub use crate::direction_iterator::*;
    pub use crate::direction_table::*;
    pub use crate::direction_type::*;
    pub use crate::ordinal_direction::*;
    pub use crate::vertical_direction::*;
}
