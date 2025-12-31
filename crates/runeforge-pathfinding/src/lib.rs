//! Pathfinding algorithms for 2D grids.
//!
//! # Overview
//!
//! `runeforge-pathfinding` provides a flexible pathfinding system for tile-based games.
//! It supports multiple algorithms and allows you to define your own movement costs and connectivity
//! via the `PathProvider` trait.
//!
//! # Algorithms
//!
//! *   **A* (A-Star):** The standard algorithm for finding the shortest path. Uses heuristics.
//! *   **Dijkstra:** Like A* but without heuristics. Finds shortest paths to all nodes (or one).
//! *   **BFS:** Breadth-First Search. Good for unweighted graphs.
//! *   **DFS:** Depth-First Search. Does not guarantee shortest path.
//! *   **ID A* / ID DFS:** Iterative deepening variants.
//!
//! # Usage
//!
//! Add this to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! runeforge-pathfinding = "0.1"
//! ```
//!

pub mod algorithms;
pub mod path_algorithm;
pub mod path_provider;
pub mod pathfinder;

pub mod prelude {
    pub use glam::IVec2;
    pub use runeforge_geometry::prelude::GridPoint;
    pub use std::collections::BinaryHeap;

    pub use crate::algorithms::*;
    pub use crate::path_algorithm::*;
    pub use crate::path_provider::*;
    pub use crate::pathfinder::*;
}
