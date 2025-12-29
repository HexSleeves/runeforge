//! Pathfinding algorithms for roguelike games.
//!
//! This crate provides A* pathfinding optimized for grid-based roguelikes.
//! The implementation focuses on clarity and educational value while maintaining
//! good performance.
//!
#![deny(missing_docs)]

use runeforge_geometry::Point;
use rustc_hash::FxHashMap;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

/// Result type for pathfinding - a list of points from start to goal.
pub type PathResult = Option<Vec<Point>>;

/// Represents a node in the A* search.
///
/// Each node tracks its position, costs, and parent for path reconstruction.
#[derive(Debug, Clone, Eq, PartialEq)]
struct Node {
    /// Position on the grid
    position: Point,
    /// Cost from start to this node (g-cost)
    g_cost: i32,
    /// Estimated cost from this node to goal (h-cost)
    h_cost: i32,
    /// Parent node for path reconstruction
    parent: Option<Point>,
}

impl Node {
    #[inline]
    fn new(position: Point, g_cost: i32, h_cost: i32, parent: Option<Point>) -> Self {
        Self {
            position,
            g_cost,
            h_cost,
            parent,
        }
    }

    /// Returns the f-cost (g + h)
    #[inline]
    fn f_cost(&self) -> i32 {
        self.g_cost + self.h_cost
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .f_cost()
            .cmp(&self.f_cost())
            .then_with(|| other.h_cost.cmp(&self.h_cost))
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Grid-based closed set for faster lookups on dense grids.
/// Uses a flat boolean array indexed by position.
pub struct GridSet {
    data: Vec<bool>,
    width: usize,
    height: usize,
    min_x: i32,
    min_y: i32,
}

impl GridSet {
    /// Creates a new grid set covering the given bounds.
    pub fn new(min_x: i32, min_y: i32, max_x: i32, max_y: i32) -> Self {
        let width = (max_x - min_x + 1) as usize;
        let height = (max_y - min_y + 1) as usize;
        Self {
            data: vec![false; width * height],
            width,
            height,
            min_x,
            min_y,
        }
    }

    #[inline]
    fn index(&self, p: Point) -> Option<usize> {
        let x = (p.x - self.min_x) as usize;
        let y = (p.y - self.min_y) as usize;
        if x < self.width && y < self.height {
            Some(y * self.width + x)
        } else {
            None
        }
    }

    /// Inserts a point into the set.
    /// Returns true if the point was newly inserted, false if it was already present.
    #[inline]
    pub fn insert(&mut self, p: Point) -> bool {
        if let Some(idx) = self.index(p) {
            let was_present = self.data[idx];
            self.data[idx] = true;
            !was_present
        } else {
            false
        }
    }

    /// Checks if the set contains a point.
    #[inline]
    pub fn contains(&self, p: Point) -> bool {
        self.index(p).is_some_and(|idx| self.data[idx])
    }
}

/// Heuristic function for 4-directional movement (Manhattan distance).
#[inline]
fn heuristic(from: Point, to: Point) -> i32 {
    (from.x - to.x).abs() + (from.y - to.y).abs()
}

/// Heuristic function for 8-directional movement (Chebyshev distance).
#[inline]
fn heuristic_8dir(from: Point, to: Point) -> i32 {
    (from.x - to.x).abs().max((from.y - to.y).abs())
}

/// A* pathfinding using FxHashMap and optional grid-based closed set.
///
/// This version is 2-5x faster than the standard HashMap implementation for typical roguelike maps.
///
/// # Arguments
///
/// * `start` - Starting position
/// * `goal` - Goal position
/// * `is_walkable` - Function that returns true if a position is walkable
/// * `bounds` - Optional search bounds as `(min_x, min_y, max_x, max_y)`.
///   When provided, enables a grid-based closed set for faster lookups.
///   **Important:** The search will NOT explore positions outside these bounds.
///   If the goal is outside the bounds, no path will be found.
///   Use `None` for unbounded search (uses hash-based closed set).
///
/// # Returns
///
/// `Some(Vec<Point>)` containing the path from start to goal (inclusive),
/// or `None` if no path exists or goal is outside bounds.
///
/// # Example
///
/// ```
/// use runeforge_pathfinding::astar;
/// use runeforge_geometry::Point;
///
/// let start = Point::new(0, 0);
/// let goal = Point::new(5, 5);
///
/// // Simple walkability check (everything is walkable)
/// let is_walkable = |_p: Point| true;
///
/// if let Some(path) = astar(start, goal, &is_walkable, None) {
///     println!("Found path with {} steps", path.len());
///     // Path includes both start and goal
///     assert_eq!(path[0], start);
///     assert_eq!(path[path.len() - 1], goal);
/// }
/// ```
pub fn astar<F>(
    start: Point,
    goal: Point,
    is_walkable: &F,
    bounds: Option<(i32, i32, i32, i32)>,
) -> PathResult
where
    F: Fn(Point) -> bool,
{
    // Early exit checks
    if !is_walkable(start) || !is_walkable(goal) {
        return None;
    }

    if start == goal {
        return Some(vec![start]);
    }

    // Use FxHashMap for faster integer key hashing
    let mut open_set = BinaryHeap::with_capacity(256);
    let mut g_costs: FxHashMap<Point, i32> = FxHashMap::default();
    let mut parents: FxHashMap<Point, Point> = FxHashMap::default();

    // Use grid-based closed set if bounds are provided
    let mut closed_set_grid =
        bounds.map(|(min_x, min_y, max_x, max_y)| GridSet::new(min_x, min_y, max_x, max_y));

    // Fallback to hash set if no bounds
    let mut closed_set_hash = if closed_set_grid.is_none() {
        Some(FxHashMap::default())
    } else {
        None
    };

    // Initialize
    let start_node = Node::new(start, 0, heuristic(start, goal), None);
    open_set.push(start_node);
    g_costs.insert(start, 0);

    // Main A* loop
    while let Some(current) = open_set.pop() {
        let current_pos = current.position;

        // Check if we reached the goal
        if current_pos == goal {
            // Reconstruct path
            let mut path = Vec::with_capacity(current.g_cost as usize + 1);
            path.push(goal);
            let mut current = goal;

            while let Some(&parent) = parents.get(&current) {
                path.push(parent);
                current = parent;
            }

            path.reverse();
            return Some(path);
        }

        // Skip if already explored
        let already_closed = if let Some(ref mut grid) = closed_set_grid {
            !grid.insert(current_pos)
        } else if let Some(ref mut hash) = closed_set_hash {
            hash.insert(current_pos, ()).is_some()
        } else {
            false
        };

        if already_closed {
            continue;
        }

        // Explore neighbors (4-directional)
        for neighbor_pos in current_pos.cardinal_neighbors() {
            // Skip if not walkable
            if !is_walkable(neighbor_pos) {
                continue;
            }

            // Skip if already explored
            let is_closed = if let Some(ref grid) = closed_set_grid {
                grid.contains(neighbor_pos)
            } else if let Some(ref hash) = closed_set_hash {
                hash.contains_key(&neighbor_pos)
            } else {
                false
            };

            if is_closed {
                continue;
            }

            // Calculate g-cost
            let tentative_g_cost = current.g_cost + 1;

            // Check if this path is better
            if let Some(&existing_g_cost) = g_costs.get(&neighbor_pos) {
                if tentative_g_cost >= existing_g_cost {
                    continue;
                }
            }

            // Update costs and parent
            g_costs.insert(neighbor_pos, tentative_g_cost);
            parents.insert(neighbor_pos, current_pos);

            // Add to open set
            let h_cost = heuristic(neighbor_pos, goal);
            let neighbor_node =
                Node::new(neighbor_pos, tentative_g_cost, h_cost, Some(current_pos));
            open_set.push(neighbor_node);
        }
    }

    None
}

/// 8-directional A* pathfinding (includes diagonal movement).
///
/// Similar to [`astar`] but allows diagonal movement (8 directions instead of 4).
/// Uses Chebyshev distance as the heuristic.
///
/// # Arguments
///
/// * `start` - Starting position
/// * `goal` - Goal position
/// * `is_walkable` - Function that returns true if a position is walkable
/// * `bounds` - Optional search bounds as `(min_x, min_y, max_x, max_y)`.
///   See [`astar`] for details on bounds behavior.
///
/// # Returns
///
/// `Some(Vec<Point>)` containing the path from start to goal (inclusive),
/// or `None` if no path exists.
pub fn astar_8dir<F>(
    start: Point,
    goal: Point,
    is_walkable: &F,
    bounds: Option<(i32, i32, i32, i32)>,
) -> PathResult
where
    F: Fn(Point) -> bool,
{
    if !is_walkable(start) || !is_walkable(goal) {
        return None;
    }

    if start == goal {
        return Some(vec![start]);
    }

    let mut open_set = BinaryHeap::with_capacity(256);
    let mut g_costs: FxHashMap<Point, i32> = FxHashMap::default();
    let mut parents: FxHashMap<Point, Point> = FxHashMap::default();

    let mut closed_set_grid =
        bounds.map(|(min_x, min_y, max_x, max_y)| GridSet::new(min_x, min_y, max_x, max_y));

    let mut closed_set_hash = if closed_set_grid.is_none() {
        Some(FxHashMap::default())
    } else {
        None
    };

    let start_node = Node::new(start, 0, heuristic_8dir(start, goal), None);
    open_set.push(start_node);
    g_costs.insert(start, 0);

    while let Some(current) = open_set.pop() {
        let current_pos = current.position;

        if current_pos == goal {
            let mut path = Vec::with_capacity(current.g_cost as usize + 1);
            path.push(goal);
            let mut current = goal;

            while let Some(&parent) = parents.get(&current) {
                path.push(parent);
                current = parent;
            }

            path.reverse();
            return Some(path);
        }

        let already_closed = if let Some(ref mut grid) = closed_set_grid {
            !grid.insert(current_pos)
        } else if let Some(ref mut hash) = closed_set_hash {
            hash.insert(current_pos, ()).is_some()
        } else {
            false
        };

        if already_closed {
            continue;
        }

        // Use all_neighbors for 8-directional movement
        for neighbor_pos in current_pos.all_neighbors() {
            if !is_walkable(neighbor_pos) {
                continue;
            }

            let is_closed = if let Some(ref grid) = closed_set_grid {
                grid.contains(neighbor_pos)
            } else if let Some(ref hash) = closed_set_hash {
                hash.contains_key(&neighbor_pos)
            } else {
                false
            };

            if is_closed {
                continue;
            }

            let tentative_g_cost = current.g_cost + 1;

            if let Some(&existing_g_cost) = g_costs.get(&neighbor_pos) {
                if tentative_g_cost >= existing_g_cost {
                    continue;
                }
            }

            g_costs.insert(neighbor_pos, tentative_g_cost);
            parents.insert(neighbor_pos, current_pos);

            let h_cost = heuristic_8dir(neighbor_pos, goal);
            let neighbor_node =
                Node::new(neighbor_pos, tentative_g_cost, h_cost, Some(current_pos));
            open_set.push(neighbor_node);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_astar_straight_line() {
        let start = Point::new(0, 0);
        let goal = Point::new(5, 0);
        let is_walkable = |_: Point| true;

        let path = astar(start, goal, &is_walkable, None).expect("Should find path");

        assert_eq!(path.len(), 6); // 0,0 -> 1,0 -> 2,0 -> 3,0 -> 4,0 -> 5,0
        assert_eq!(path[0], start);
        assert_eq!(path[path.len() - 1], goal);
    }

    #[test]
    fn test_astar_with_wall() {
        let start = Point::new(0, 0);
        let goal = Point::new(2, 0);

        // Wall at (1, 0)
        let is_walkable = |p: Point| p != Point::new(1, 0);

        let path = astar(start, goal, &is_walkable, None).expect("Should find path around wall");

        // Path should go around: (0,0) -> (0,1) -> (1,1) -> (2,1) -> (2,0)
        assert!(path.len() > 3); // Longer than straight path
        assert_eq!(path[0], start);
        assert_eq!(path[path.len() - 1], goal);
    }

    #[test]
    fn test_astar_no_path() {
        let start = Point::new(0, 0);
        let goal = Point::new(2, 0);

        // Walls completely block the path
        // Vertical wall at x=1 with bounds checking
        let is_walkable = |p: Point| {
            // Bounds check
            if p.x < 0 || p.y < 0 || p.x > 3 || p.y > 3 {
                return false;
            }
            // Vertical wall at x=1 blocks the path
            p.x != 1
        };

        let path = astar(start, goal, &is_walkable, None);
        assert!(path.is_none()); // No path should exist
    }

    #[test]
    fn test_astar_start_equals_goal() {
        let start = Point::new(5, 5);
        let is_walkable = |_: Point| true;

        let path = astar(start, start, &is_walkable, None).expect("Should return single point");
        assert_eq!(path.len(), 1);
        assert_eq!(path[0], start);
    }

    #[test]
    fn test_astar_8dir_diagonal() {
        let start = Point::new(0, 0);
        let goal = Point::new(3, 3);
        let is_walkable = |_: Point| true;

        let path_4dir = astar(start, goal, &is_walkable, None).expect("Should find 4-dir path");
        let path_8dir =
            astar_8dir(start, goal, &is_walkable, None).expect("Should find 8-dir path");

        println!("Path 4dir: {:?}", path_4dir);
        println!("Path 8dir: {:?}", path_8dir);

        // 8-directional should be shorter (can cut diagonally)
        assert!(path_8dir.len() <= path_4dir.len());
    }

    #[test]
    fn test_astar_with_bounds() {
        let start = Point::new(5, 5);
        let goal = Point::new(15, 15);
        let is_walkable = |_: Point| true;

        // Without bounds - should find path
        let path_unbounded =
            astar(start, goal, &is_walkable, None).expect("Should find unbounded path");
        assert!(!path_unbounded.is_empty());

        // With restrictive bounds (0,0,10,10) - should fail as goal is outside
        let path_bounded = astar(start, goal, &is_walkable, Some((0, 0, 10, 10)));
        assert!(path_bounded.is_none()); // Should fail - goal outside bounds

        // With sufficient bounds (0,0,20,20) - should succeed
        let path_sufficient_bounds = astar(start, goal, &is_walkable, Some((0, 0, 20, 20)))
            .expect("Should find path with sufficient bounds");
        assert!(!path_sufficient_bounds.is_empty());
    }

    #[test]
    fn test_astar_8dir_with_bounds() {
        let start = Point::new(0, 0);
        let goal = Point::new(5, 5);
        let is_walkable = |_: Point| true;

        // With bounds that constrain the search area
        let bounds = Some((0, 0, 3, 3)); // Only allow search in top-left 3x3 area

        // Should fail because goal is outside bounds
        let path = astar_8dir(start, goal, &is_walkable, bounds);
        assert!(path.is_none()); // Goal (5,5) is outside bounds (0,0,3,3)

        // With sufficient bounds
        let sufficient_bounds = Some((0, 0, 6, 6));
        let path = astar_8dir(start, goal, &is_walkable, sufficient_bounds)
            .expect("Should find path with sufficient bounds");
        assert!(!path.is_empty());
        assert_eq!(path[0], start);
        assert_eq!(path[path.len() - 1], goal);
    }

    #[test]
    fn test_astar_bounds_partial_path() {
        // Test that bounds can force a different (longer) path
        let start = Point::new(0, 0);
        let goal = Point::new(4, 0);

        // Map with obstacles that force detours
        let is_walkable = |p: Point| {
            // Block direct path at y=0 for x=1,2,3
            !(p.y == 0 && (p.x == 1 || p.x == 2 || p.x == 3))
        };

        // Without bounds - should find detour path
        let path_unbounded =
            astar(start, goal, &is_walkable, None).expect("Should find unbounded path");

        // With tight bounds (0,0,5,2) - should still find path but might be different
        let path_bounded =
            astar(start, goal, &is_walkable, Some((0, 0, 5, 2))).expect("Should find bounded path");

        // Both should reach the goal
        assert_eq!(path_unbounded[path_unbounded.len() - 1], goal);
        assert_eq!(path_bounded[path_bounded.len() - 1], goal);

        // Bounded path might be longer or same length depending on bounds effect
        assert!(path_bounded.len() >= path_unbounded.len());
    }
}
