//! Field of View (FOV) algorithms for 2D grids.
//!
//! # Overview
//!
//! `runeforge-fov` provides algorithms to determine which cells are visible from a given point on a grid.
//! It abstracts the map representation via the `FovProvider` trait, allowing it to work with any
//! map structure (dense arrays, sparse hashmaps, etc.).
//!
//! # Algorithms
//!
//! *   **Shadowcasting:** Recursive shadowcasting. Efficient and symmetric. Good for most use cases.
//! *   **Adams:** An implementation of the Adams FOV algorithm.
//!
//! # Usage
//!
//! Add this to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! runeforge-fov = "0.1"
//! ```
//!
//! ## Basic Example
//!
//! ```rust
//! use runeforge_fov::prelude::*;
//! use runeforge_geometry::prelude::IVec2;
//! use hashbrown::HashSet;
//!
//! // 1. Define your map representation
//! struct MyMap {
//!     walls: HashSet<IVec2>,
//! }
//!
//! // 2. Implement the FovProvider trait
//! impl FovProvider<()> for MyMap {
//!     fn is_opaque(&mut self, position: IVec2, _data: &mut ()) -> bool {
//!         self.walls.contains(&position)
//!     }
//! }
//!
//! fn main() {
//!     let mut map = MyMap { walls: HashSet::new() };
//!     map.walls.insert(IVec2::new(5, 5)); // Add a wall
//!
//!     // 3. Compute FOV
//!     let origin = IVec2::new(0, 0);
//!     let range: u32 = 10;
//!     let visible_cells = Fov::Shadowcast.compute(origin, range, &mut map, ());
//!
//!     assert!(visible_cells.contains(&IVec2::new(1, 1)));
//! }
//! ```

pub mod adams;
pub mod fov;
pub mod shadowcast;
pub mod slope;

pub mod prelude {
    pub use hashbrown::HashSet;
    pub use runeforge_direction::prelude::{CardinalDirection, Direction, DirectionIterator};
    pub use runeforge_geometry::prelude::IVec2;

    pub use crate::adams::*;
    pub use crate::fov::*;
    pub use crate::shadowcast::*;
    pub use crate::slope::*;
}
