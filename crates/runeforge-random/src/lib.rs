//! Random number generation and dice utilities for roguelikes.
//!
//! # Overview
//!
//! `runeforge-random` provides tools for randomness, specifically tailored for RPGs and roguelikes.
//! It includes:
//!
//! *   **RNG Wrapper:** A seeded random number generator (using `rand`).
//! *   **Dice Notation:** Parse and roll dice strings like "3d6+2", "1d20", "2d4-1".
//! *   **Weighted Choice:** Select items from a list with different probabilities.
//!
//! # Usage
//!
//! Add this to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! runeforge-random = "0.1"
//! ```
//!
//! ## Basic Example
//!
//! ```rust
//! use runeforge_random::prelude::*;
//!
//! fn main() {
//!     // Create a new RNG (optionally seeded)
//!     let mut rng = Rng::new();
//!
//!     // Roll dice
//!     let damage = rng.roll("2d6+1").unwrap();
//!     println!("Dealt {} damage!", damage);
//!
//!     // Random chance
//!     if rng.chance(0.1) {
//!         println!("Critical hit!");
//!     }
//! }
//! ```

pub mod rand;
pub mod random_value;
pub mod rng;

pub mod prelude {
    pub use crate::rand::*;
    pub use crate::random_value::*;
    pub use crate::rng::*;
    pub use std::ops::{Bound, Index, IndexMut, RangeBounds};
}
