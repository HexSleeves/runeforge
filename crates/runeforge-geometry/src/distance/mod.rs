//! Distance algorithms for 2D grids.
//!
//! This module provides various distance calculation algorithms commonly used
//! in roguelike games for pathfinding heuristics, FOV calculations, and range checks.
//!
//! # Available Algorithms
//!
//! - [`Manhattan`] - Taxicab distance: `|dx| + |dy|`
//! - [`Chebyshev`] - King's move distance: `max(|dx|, |dy|)`
//! - [`Pythagoras`] - Euclidean distance: `sqrt(dx² + dy²)`
//! - [`PythagorasSquared`] - Squared Euclidean (faster, avoids sqrt)
//! - [`Diagonal`] - Octile distance with √2 diagonal cost
//! - [`DiagonalWithCosts`] - Custom cardinal/diagonal costs
//!
//! # Example
//!
//! ```rust
//! use runeforge_geometry::prelude::*;
//!
//! let start = IVec2::new(0, 0);
//! let end = IVec2::new(3, 4);
//!
//! // Manhattan distance (3 + 4 = 7)
//! assert_eq!(Manhattan.distance2d(start, end), 7.0);
//!
//! // Euclidean distance (sqrt(3² + 4²) = 5)
//! assert_eq!(Pythagoras.distance2d(start, end), 5.0);
//! ```

use glam::IVec2;
use std::f32::consts::SQRT_2;
use std::ops::Sub;

/// Trait for 2D distance algorithms.
pub trait DistanceAlgorithm {
    /// Calculate the distance between two 2D points.
    fn distance2d(self, start: IVec2, end: IVec2) -> f32;
}

/// Manhattan (taxicab) distance: `|dx| + |dy|`.
///
/// This is the distance when you can only move in 4 cardinal directions.
/// Useful for 4-way movement games.
pub struct Manhattan;

impl DistanceAlgorithm for Manhattan {
    fn distance2d(self, start: IVec2, end: IVec2) -> f32 {
        let start = start.as_vec2();
        let end = end.as_vec2();
        let distance = start.max(end) - start.min(end);
        distance.x + distance.y
    }
}

/// Chebyshev distance: `max(|dx|, |dy|)`.
///
/// This is the distance when diagonal moves cost the same as cardinal moves.
/// Also known as "King's move" distance in chess.
pub struct Chebyshev;

impl DistanceAlgorithm for Chebyshev {
    fn distance2d(self, start: IVec2, end: IVec2) -> f32 {
        let start = start.as_vec2();
        let end = end.as_vec2();
        start.sub(end).abs().max_element()
    }
}

/// Euclidean distance: `sqrt(dx² + dy²)`.
///
/// True straight-line distance. More expensive due to sqrt.
pub struct Pythagoras;

impl DistanceAlgorithm for Pythagoras {
    fn distance2d(self, start: IVec2, end: IVec2) -> f32 {
        let distance_squared = PythagorasSquared.distance2d(start, end);
        f32::sqrt(distance_squared)
    }
}

/// Squared Euclidean distance: `dx² + dy²`.
///
/// Faster than [`Pythagoras`] for comparisons since it avoids sqrt.
/// Use when you only need to compare distances, not get actual values.
pub struct PythagorasSquared;

impl DistanceAlgorithm for PythagorasSquared {
    fn distance2d(self, start: IVec2, end: IVec2) -> f32 {
        let start = start.as_vec2();
        let end = end.as_vec2();
        let distance = (start.max(end) - start.min(end)).powf(2.0);
        distance.x + distance.y
    }
}

/// Default cardinal movement cost for diagonal distance.
const CARDINAL_COST: f32 = 1.0;

/// Default diagonal movement cost (√2 ≈ 1.414).
pub const DIAGONAL_COST: f32 = SQRT_2;

/// Diagonal (octile) distance with default costs.
///
/// Uses 1.0 for cardinal moves and √2 for diagonal moves.
/// This is the most accurate heuristic for 8-way movement.
pub struct Diagonal;

impl DistanceAlgorithm for Diagonal {
    fn distance2d(self, start: IVec2, end: IVec2) -> f32 {
        DiagonalWithCosts(CARDINAL_COST, DIAGONAL_COST).distance2d(start, end)
    }
}

/// Diagonal distance with custom cardinal and diagonal costs.
///
/// # Arguments
///
/// - `.0` - Cardinal move cost (default: 1.0)
/// - `.1` - Diagonal move cost (default: √2 ≈ 1.414)
pub struct DiagonalWithCosts(pub f32, pub f32);

impl DistanceAlgorithm for DiagonalWithCosts {
    fn distance2d(self, start: IVec2, end: IVec2) -> f32 {
        let start = start.as_vec2();
        let end = end.as_vec2();
        let distance = start.sub(end).abs();
        self.0.mul_add(
            distance.max_element(),
            (self.1 - self.0) * distance.min_element(),
        )
    }
}

/// Enum dispatcher for distance algorithms.
///
/// Convenient when you need to select an algorithm at runtime.
#[derive(Debug, Clone, Copy)]
pub enum Distance {
    /// Euclidean distance: `sqrt(dx² + dy²)`
    Pythagoras,
    /// Squared Euclidean (faster, avoids sqrt)
    PythagorasSquared,
    /// Manhattan distance: `|dx| + |dy|`
    Manhattan,
    /// Chebyshev distance: `max(|dx|, |dy|)`
    Chebyshev,
    /// Diagonal distance with default √2 cost
    Diagonal,
    /// Diagonal distance with custom costs
    DiagonalWithCosts(f32, f32),
}

impl Distance {
    /// Calculate distance between two points using the selected algorithm.
    pub fn distance2d(self, start: IVec2, end: IVec2) -> f32 {
        match self {
            Self::Manhattan => Manhattan.distance2d(start, end),
            Self::Chebyshev => Chebyshev.distance2d(start, end),
            Self::Diagonal => Diagonal.distance2d(start, end),
            Self::Pythagoras => Pythagoras.distance2d(start, end),
            Self::PythagorasSquared => PythagorasSquared.distance2d(start, end),
            Self::DiagonalWithCosts(d1, d2) => DiagonalWithCosts(d1, d2).distance2d(start, end),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_manhattan() {
        let start = IVec2::new(0, 0);
        let end = IVec2::new(3, 4);
        assert_eq!(Manhattan.distance2d(start, end), 7.0);
    }

    #[test]
    fn test_chebyshev() {
        let start = IVec2::new(0, 0);
        let end = IVec2::new(3, 4);
        assert_eq!(Chebyshev.distance2d(start, end), 4.0);
    }

    #[test]
    fn test_pythagoras() {
        let start = IVec2::new(0, 0);
        let end = IVec2::new(3, 4);
        assert_eq!(Pythagoras.distance2d(start, end), 5.0);
    }

    #[test]
    fn test_pythagoras_squared() {
        let start = IVec2::new(0, 0);
        let end = IVec2::new(3, 4);
        assert_eq!(PythagorasSquared.distance2d(start, end), 25.0);
    }

    #[test]
    fn test_diagonal() {
        let start = IVec2::new(0, 0);
        let end = IVec2::new(3, 3);
        // 3 diagonal moves * √2 ≈ 4.24
        let dist = Diagonal.distance2d(start, end);
        assert!((dist - 3.0 * SQRT_2).abs() < 0.001);
    }

    #[test]
    fn test_distance_enum() {
        let start = IVec2::new(0, 0);
        let end = IVec2::new(3, 4);
        assert_eq!(Distance::Manhattan.distance2d(start, end), 7.0);
        assert_eq!(Distance::Pythagoras.distance2d(start, end), 5.0);
    }
}
