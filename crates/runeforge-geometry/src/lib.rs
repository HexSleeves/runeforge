//! 2D Geometry primitives for grid-based games.
//!
//! # Overview
//!
//! `runeforge-geometry` provides essential geometric types and operations tailored for
//! tile-based games. It leverages `glam` for vector math but adds game-specific abstractions.
//!
//! # Key Features
//!
//! *   **Primitives:** `Rect` (Rectangle), `Circle`, `Line`.
//! *   **Distance:** Multiple distance algorithms (Manhattan, Euclidean, Chebyshev, Diagonal).
//! *   **Iterators:** Efficiently iterate over points within shapes (e.g., all tiles in a room).
//! *   **Intersections:** Check if shapes overlap.
//! *   **Grid Points:** Utilities for manipulating integer coordinates (`IVec2`).
//!
//! # Usage
//!
//! Add this to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! runeforge-geometry = "0.1"
//! ```
//!
//! ## Basic Example
//!
//! ```rust
//! use runeforge_geometry::prelude::*;
//!
//! fn main() {
//!     // Create a room (Rectangle)
//!     let room = Rect::new(IVec2::new(10, 10), IVec2::new(20, 20));
//!
//!     // Check if a point is inside
//!     let player_pos = IVec2::new(15, 15);
//!     if room.intersects(Rect::new(player_pos, player_pos)) {
//!         println!("Player is in the room!");
//!     }
//!
//!     // Iterate over all points in the room
//!     for point in room.points() {
//!         // Draw floor tile at point...
//!     }
//!
//!     // Calculate distance between points
//!     let a = IVec2::new(0, 0);
//!     let b = IVec2::new(3, 4);
//!     let dist = Pythagoras.distance2d(a, b);
//!     assert_eq!(dist, 5.0);
//! }
//! ```

pub mod distance;
pub mod point;
pub mod shapes;

pub mod prelude {
    pub use crate::distance::*;
    pub use crate::point::*;
    pub use crate::shapes::*;
    pub use glam::{IVec2, UVec2, Vec2};
    pub use runeforge_direction::prelude::*;
}
