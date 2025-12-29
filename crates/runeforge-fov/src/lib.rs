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
    const fn new(numerator: i32, denominator: i32) -> Self {
        Self {
            numerator,
            denominator,
        }
    }

    /// Returns true if self >= other.
    fn greater_equal(self, other: Self) -> bool {
        self.numerator * other.denominator >= other.numerator * self.denominator
    }

    /// Returns true if self < other.
    fn less_than(self, other: Self) -> bool {
        self.numerator * other.denominator < other.numerator * self.denominator
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
    fn new(depth: i32, start_slope: Fraction, end_slope: Fraction) -> Self {
        Self {
            depth,
            start_slope,
            end_slope,
        }
    }

    /// Returns the list of tiles in this row.
    fn tiles(&self) -> Vec<i32> {
        // Calculate the fractional bounds
        let min_col = Self::round_ties_up_frac(self.start_slope, self.depth);
        let max_col = Self::round_ties_down_frac(self.end_slope, self.depth);
        (min_col..=max_col).collect()
    }

    /// Returns the next row.
    fn next(&self) -> Self {
        Self::new(self.depth + 1, self.start_slope, self.end_slope)
    }

    /// Round ties up: floor(fraction * depth + 0.5)
    fn round_ties_up_frac(frac: Fraction, depth: i32) -> i32 {
        // floor((num * depth) / den + 0.5) = floor((num * depth + den/2) / den)
        let num = frac.numerator * depth;
        let den = frac.denominator;
        (num * 2 + den) / (den * 2)
    }

    /// Round ties down: ceil(fraction * depth - 0.5)
    fn round_ties_down_frac(frac: Fraction, depth: i32) -> i32 {
        // ceil((num * depth) / den - 0.5) = ceil((num * depth - den/2) / den)
        let num = frac.numerator * depth;
        let den = frac.denominator;
        (num * 2 - den + (den * 2 - 1)) / (den * 2)
    }

    /// Returns the slope from the origin through the left edge of the tile.
    fn slope(col: i32, depth: i32) -> Fraction {
        Fraction::new(2 * col - 1, 2 * depth)
    }

    /// Returns true if the floor tile is symmetric.
    fn is_symmetric(&self, col: i32) -> bool {
        let slope = Fraction::new(col, self.depth);
        slope.greater_equal(self.start_slope) && slope.less_than(self.end_slope)
    }

    /// Returns true if the wall tile is visible.
    fn is_wall_visible(&self, col: i32) -> bool {
        let slope = Self::slope(col, self.depth);
        let adj_start = Fraction::new(
            self.start_slope.numerator * self.depth,
            self.start_slope.denominator,
        );
        let adj_end = Fraction::new(
            self.end_slope.numerator * self.depth,
            self.end_slope.denominator,
        );

        slope.greater_equal(adj_start) && slope.less_than(adj_end)
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

    // Scan each quadrant
    for cardinal in [
        Cardinal::North,
        Cardinal::South,
        Cardinal::East,
        Cardinal::West,
    ] {
        let quadrant = Quadrant { cardinal };
        let first_row = Row::new(1, Fraction::new(-1, 1), Fraction::new(1, 1));

        scan(
            origin,
            max_radius,
            first_row,
            quadrant,
            is_blocking,
            mark_visible,
        );
    }
}

/// Scans a single row in a quadrant.
fn scan<F, G>(
    origin: Point,
    max_radius: i32,
    row: Row,
    quadrant: Quadrant,
    is_blocking: &F,
    mark_visible: &mut G,
) where
    F: Fn(Point) -> bool,
    G: FnMut(Point),
{
    if row.depth > max_radius {
        return;
    }

    let mut prev_tile_blocking = None;
    let tiles = row.tiles();

    for col in tiles {
        let tile = quadrant.transform(origin, row.depth, col);
        let tile_blocking = is_blocking(tile);

        // Walls use is_wall_visible, floors use is_symmetric
        if (tile_blocking && row.is_wall_visible(col)) || (!tile_blocking && row.is_symmetric(col))
        {
            mark_visible(tile);
        }

        // Handle transitions between blocking and non-blocking tiles
        if let Some(prev_blocking) = prev_tile_blocking {
            if prev_blocking && !tile_blocking {
                // Transition from wall to floor - start new slope
                let mut new_row = row.next();
                new_row.start_slope = Row::slope(col, new_row.depth);
                scan(
                    origin,
                    max_radius,
                    new_row,
                    quadrant,
                    is_blocking,
                    mark_visible,
                );
            } else if !prev_blocking && tile_blocking {
                // Transition from floor to wall - end current slope
                let mut next_row = row.next();
                next_row.end_slope = Row::slope(col, next_row.depth);
                scan(
                    origin,
                    max_radius,
                    next_row,
                    quadrant,
                    is_blocking,
                    mark_visible,
                );
                return;
            }
        }

        prev_tile_blocking = Some(tile_blocking);
    }

    // Continue to next row if we didn't end on a blocking tile
    if let Some(false) = prev_tile_blocking {
        scan(
            origin,
            max_radius,
            row.next(),
            quadrant,
            is_blocking,
            mark_visible,
        );
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
        for dx in -radius..=radius {
            if dx * dx + dy * dy <= radius_squared {
                mark_visible(Point::new(origin.x + dx, origin.y + dy));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        assert!(quarter.less_than(half));
    }

    #[test]
    fn test_row_tiles() {
        let row = Row::new(5, Fraction::new(-1, 1), Fraction::new(1, 1));
        let tiles = row.tiles();

        assert!(!tiles.is_empty());
        assert!(tiles.contains(&0)); // Center tile should always be included
    }
}
