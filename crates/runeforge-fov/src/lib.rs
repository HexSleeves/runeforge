//! Field-of-view algorithms for roguelike games.
//!
//! This crate provides various FOV algorithms including symmetric shadowcasting,
//! which is efficient and produces visually pleasing results.
//!
//! # Example
//!
//! ```
//! use runeforge_fov::compute_fov;
//! use runeforge_geometry::Point;
//!
//! // Create a simple map (true = wall, false = floor)
//! let mut map = vec![vec![false; 10]; 10];
//! map[5][5] = true; // Add a wall
//!
//! let mut visible = vec![vec![false; 10]; 10];
//! let origin = Point::new(0, 0);
//! let radius = 8;
//!
//! compute_fov(
//!     origin,
//!     radius,
//!     &|p| {
//!         if p.x < 0 || p.y < 0 || p.x >= 10 || p.y >= 10 {
//!             true // Out of bounds is blocking
//!         } else {
//!             map[p.y as usize][p.x as usize]
//!         }
//!     },
//!     &mut |p| {
//!         if p.x >= 0 && p.y >= 0 && p.x < 10 && p.y < 10 {
//!             visible[p.y as usize][p.x as usize] = true;
//!         }
//!     },
//! );
//! ```

#![deny(missing_docs)]

use runeforge_geometry::Point;

/// A simple fraction type for exact slope calculations.
///
/// Using fractions avoids floating-point rounding errors in the shadowcasting algorithm.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Fraction {
    numerator: i32,
    denominator: i32,
}

impl Fraction {
    /// Creates a new fraction.
    #[inline]
    const fn new(numerator: i32, denominator: i32) -> Self {
        Self {
            numerator,
            denominator,
        }
    }

    /// Returns true if self >= other.
    #[inline]
    fn greater_equal(self, other: Self) -> bool {
        self.numerator * other.denominator >= other.numerator * self.denominator
    }

    /// Returns true if self <= other.
    #[inline]
    fn less_equal(self, other: Self) -> bool {
        self.numerator * other.denominator <= other.numerator * self.denominator
    }
}

/// Represents a quadrant for FOV calculation.
#[derive(Debug, Clone, Copy)]
struct Quadrant {
    cardinal: Cardinal,
}

#[derive(Debug, Clone, Copy)]
enum Cardinal {
    North,
    South,
    East,
    West,
}

impl Quadrant {
    /// Transforms relative (row, col) coordinates to absolute map coordinates.
    #[inline]
    fn transform(self, origin: Point, row: i32, col: i32) -> Point {
        match self.cardinal {
            Cardinal::North => Point::new(origin.x + col, origin.y - row),
            Cardinal::South => Point::new(origin.x + col, origin.y + row),
            Cardinal::East => Point::new(origin.x + row, origin.y + col),
            Cardinal::West => Point::new(origin.x - row, origin.y + col),
        }
    }
}

/// Represents a row being scanned in the FOV algorithm.
#[derive(Debug, Clone)]
struct Row {
    depth: i32,
    start_slope: Fraction,
    end_slope: Fraction,
}

impl Row {
    #[inline]
    fn new(depth: i32, start_slope: Fraction, end_slope: Fraction) -> Self {
        Self {
            depth,
            start_slope,
            end_slope,
        }
    }

    /// Returns an iterator over the column indices in this row.
    /// Avoids allocation by returning a range iterator directly.
    #[inline]
    fn tiles(&self) -> std::ops::RangeInclusive<i32> {
        let min_col = Self::round_ties_up_frac(self.start_slope, self.depth);
        let max_col = Self::round_ties_down_frac(self.end_slope, self.depth);
        min_col..=max_col
    }

    /// Returns the next row.
    #[inline]
    fn next(&self) -> Self {
        Self::new(self.depth + 1, self.start_slope, self.end_slope)
    }

    /// Round ties up: floor(fraction * depth + 0.5)
    #[inline]
    fn round_ties_up_frac(frac: Fraction, depth: i32) -> i32 {
        // Integer division rounds toward zero, so handle negatives explicitly.
        let num = frac.numerator * depth * 2 + frac.denominator;
        let den = frac.denominator * 2;
        num_integer::Integer::div_floor(&num, &den)
    }

    /// Round ties down: ceil(fraction * depth - 0.5)
    #[inline]
    fn round_ties_down_frac(frac: Fraction, depth: i32) -> i32 {
        // Integer division rounds toward zero, so handle negatives explicitly.
        let num = frac.numerator * depth * 2 - frac.denominator;
        let den = frac.denominator * 2;
        // Self::div_ceil(num, den)
        num_integer::Integer::div_ceil(&num, &den)
    }

    /// Returns the slope from the origin through the left edge of the tile.
    #[inline]
    fn slope(col: i32, depth: i32) -> Fraction {
        Fraction::new(2 * col - 1, 2 * depth)
    }

    /// Returns true if the floor tile is symmetric.
    #[inline]
    fn is_symmetric(&self, col: i32) -> bool {
        let slope = Fraction::new(col, self.depth);
        slope.greater_equal(self.start_slope) && slope.less_equal(self.end_slope)
    }

    /// Returns true if the wall tile is visible.
    #[inline]
    fn is_wall_visible(&self, col: i32) -> bool {
        let slope = Self::slope(col, self.depth);
        slope.greater_equal(self.start_slope) && slope.less_equal(self.end_slope)
    }
}

/// Computes the field of view using symmetric shadowcasting.
///
/// This algorithm satisfies all six desirable FOV properties:
/// - Symmetry
/// - Expansive walls
/// - Expanding pillar shadows
/// - No blind corners
/// - No artifacts
/// - Efficiency
///
/// # Arguments
///
/// * `origin` - The point from which to calculate visibility
/// * `max_radius` - The maximum distance to calculate (use i32::MAX for unlimited)
/// * `is_blocking` - Function that returns true if a point blocks vision
/// * `mark_visible` - Function called for each visible point
///
/// # Example
///
/// ```
/// use runeforge_fov::compute_fov;
/// use runeforge_geometry::Point;
///
/// let origin = Point::new(5, 5);
/// let mut visible_tiles = Vec::new();
///
/// compute_fov(
///     origin,
///     10,
///     &|p| false, // Nothing blocks vision in this example
///     &mut |p| visible_tiles.push(p),
/// );
/// ```
pub fn compute_fov<F, G>(origin: Point, max_radius: i32, is_blocking: &F, mark_visible: &mut G)
where
    F: Fn(Point) -> bool,
    G: FnMut(Point),
{
    // Mark the origin as visible
    mark_visible(origin);

    // Pre-calculate squared radius for distance checks
    let max_radius_squared = max_radius * max_radius;

    // Scan each quadrant using an iterative approach with an explicit stack
    for cardinal in [
        Cardinal::North,
        Cardinal::South,
        Cardinal::East,
        Cardinal::West,
    ] {
        let quadrant = Quadrant { cardinal };

        // Use a Vec as an explicit stack to avoid recursion overhead
        // Capacity hint based on expected maximum depth
        let mut rows: Vec<Row> = Vec::with_capacity(max_radius.min(64) as usize);
        rows.push(Row::new(1, Fraction::new(-1, 1), Fraction::new(1, 1)));

        while let Some(mut row) = rows.pop() {
            if row.depth > max_radius {
                continue;
            }

            let mut prev_tile_blocking: Option<bool> = None;

            for col in row.tiles() {
                let tile = quadrant.transform(origin, row.depth, col);

                // Early distance check using squared distance to avoid sqrt
                let dx = tile.x - origin.x;
                let dy = tile.y - origin.y;
                if dx * dx + dy * dy > max_radius_squared {
                    continue;
                }

                let tile_blocking = is_blocking(tile);

                // Walls use is_wall_visible, floors use is_symmetric
                if (tile_blocking && row.is_wall_visible(col))
                    || (!tile_blocking && row.is_symmetric(col))
                {
                    mark_visible(tile);
                }

                // Handle transitions between blocking and non-blocking tiles
                if let Some(prev_blocking) = prev_tile_blocking {
                    if prev_blocking && !tile_blocking {
                        // Transition from wall to floor - start from left edge of floor tile
                        row.start_slope = Row::slope(col, row.depth);
                    } else if !prev_blocking && tile_blocking {
                        // Transition from floor to wall - push segment ending at left edge of wall
                        let mut next_row = row.next();
                        next_row.end_slope = Row::slope(col, row.depth);
                        rows.push(next_row);
                    }
                }

                prev_tile_blocking = Some(tile_blocking);
            }

            // Continue to next row if we ended on a floor tile
            if prev_tile_blocking == Some(false) {
                rows.push(row.next());
            }
        }
    }
}

/// Computes a simple circular FOV.
///
/// This is a simpler alternative to shadowcasting that reveals all tiles
/// within the given radius, ignoring obstacles.
///
/// # Example
///
/// ```
/// use runeforge_fov::compute_fov_circle;
/// use runeforge_geometry::Point;
///
/// let origin = Point::new(5, 5);
/// let mut visible = Vec::new();
///
/// compute_fov_circle(origin, 5, &mut |p| visible.push(p));
/// ```
pub fn compute_fov_circle<F>(origin: Point, radius: i32, mark_visible: &mut F)
where
    F: FnMut(Point),
{
    let radius_squared = radius * radius;

    for dy in -radius..=radius {
        // Calculate the maximum dx for this row based on the circle equation
        // This avoids checking tiles that are guaranteed to be outside the circle
        let dy_squared = dy * dy;
        let max_dx_squared = radius_squared - dy_squared;

        // Use integer sqrt approximation to get exact bounds
        // This is faster than checking every tile in the row
        let max_dx = isqrt(max_dx_squared);

        for dx in -max_dx..=max_dx {
            mark_visible(Point::new(origin.x + dx, origin.y + dy));
        }
    }
}

/// Integer square root using Newton's method.
/// Returns the largest integer n such that n*n <= x.
#[inline]
fn isqrt(x: i32) -> i32 {
    if x <= 0 {
        return 0;
    }
    if x == 1 {
        return 1;
    }

    let mut guess = x;
    let mut result = (guess + 1) / 2;

    while result < guess {
        guess = result;
        result = (guess + x / guess) / 2;
    }

    guess
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_fov_empty_room() {
        let origin = Point::new(5, 5);
        let mut visible = Vec::new();

        compute_fov(
            origin,
            5,
            &|_| false, // No obstacles
            &mut |p| visible.push(p),
        );

        // Origin should be visible
        assert!(visible.contains(&origin));
        // Should see multiple tiles
        assert!(visible.len() > 10);
    }

    #[test]
    fn test_fov_with_wall() {
        let origin = Point::new(5, 5);
        let wall = Point::new(7, 7);
        let behind_wall = Point::new(9, 9);
        let mut visible = Vec::new();

        compute_fov(origin, 10, &|p| p == wall, &mut |p| visible.push(p));

        // Origin should be visible
        assert!(visible.contains(&origin));
        // Wall should be visible
        assert!(visible.contains(&wall));
        // Tile behind wall should NOT be visible (shadowed)
        assert!(!visible.contains(&behind_wall));
        // Adjacent tiles should be visible
        assert!(visible.contains(&Point::new(6, 5)));
        assert!(visible.contains(&Point::new(5, 6)));
    }

    #[test]
    fn test_fov_circle() {
        let origin = Point::new(0, 0);
        let mut visible = Vec::new();

        compute_fov_circle(origin, 3, &mut |p| visible.push(p));

        // Origin should be visible
        assert!(visible.contains(&origin));
        // Adjacent tiles should be visible
        assert!(visible.contains(&Point::new(1, 0)));
        assert!(visible.contains(&Point::new(0, 1)));
        // Tiles exactly at radius should be visible
        assert!(visible.contains(&Point::new(3, 0)));
    }

    #[test]
    fn test_fraction_comparison() {
        let half = Fraction::new(1, 2);
        let quarter = Fraction::new(1, 4);

        assert!(half.greater_equal(quarter));
        assert!(!quarter.greater_equal(half) || quarter == half);
    }

    #[test]
    fn test_row_tiles() {
        let row = Row::new(5, Fraction::new(-1, 1), Fraction::new(1, 1));
        let tiles: Vec<_> = row.tiles().collect();

        assert!(!tiles.is_empty());
        assert!(tiles.contains(&0)); // Center tile should always be included
    }

    #[test]
    fn test_isqrt() {
        assert_eq!(isqrt(0), 0);
        assert_eq!(isqrt(1), 1);
        assert_eq!(isqrt(4), 2);
        assert_eq!(isqrt(9), 3);
        assert_eq!(isqrt(10), 3); // floor(sqrt(10)) = 3
        assert_eq!(isqrt(15), 3);
        assert_eq!(isqrt(16), 4);
        assert_eq!(isqrt(100), 10);
    }

    #[test]
    fn test_fov_circle_bounds() {
        // Verify that compute_fov_circle produces correct circular bounds
        let origin = Point::new(0, 0);
        let radius = 5;
        let mut visible = Vec::new();

        compute_fov_circle(origin, radius, &mut |p| visible.push(p));

        // All visible points should be within radius
        for p in &visible {
            let dist_sq = (p.x - origin.x).pow(2) + (p.y - origin.y).pow(2);
            assert!(
                dist_sq <= radius * radius,
                "Point {:?} is outside radius {}",
                p,
                radius
            );
        }

        // Cardinal directions at exact radius should be visible
        assert!(visible.contains(&Point::new(5, 0)));
        assert!(visible.contains(&Point::new(-5, 0)));
        assert!(visible.contains(&Point::new(0, 5)));
        assert!(visible.contains(&Point::new(0, -5)));
    }

    #[test]
    fn test_fov_diagonal_visible_empty_room() {
        let origin = Point::new(0, 0);
        let radius = 8;
        let mut visible = HashSet::new();

        compute_fov(origin, radius, &|_| false, &mut |p| {
            visible.insert(p);
        });

        let radius_sq = radius * radius;
        let mut d = 1;
        while d * d * 2 <= radius_sq {
            assert!(visible.contains(&Point::new(d, d)));
            assert!(visible.contains(&Point::new(-d, d)));
            assert!(visible.contains(&Point::new(d, -d)));
            assert!(visible.contains(&Point::new(-d, -d)));
            d += 1;
        }
    }
}
