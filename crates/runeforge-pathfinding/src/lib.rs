//! Pathfinding algorithms for roguelike games.
//!
//! This crate provides A* pathfinding optimized for grid-based roguelikes.
//! The implementation focuses on clarity and educational value while maintaining
//! good performance.

#![deny(missing_docs)]

use runeforge_geometry::Point;
use std::cmp::{max, Ordering};
use std::collections::{BinaryHeap, HashMap, HashSet};

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
    /// Creates a new node
    fn new(position: Point, g_cost: i32, h_cost: i32, parent: Option<Point>) -> Self {
        Self {
            position,
            g_cost,
            h_cost,
            parent,
        }
    }

    /// Returns the f-cost (g + h)
    fn f_cost(&self) -> i32 {
        self.g_cost + self.h_cost
    }
}

// Implement Ord for BinaryHeap (min-heap based on f-cost)
impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse comparison for min-heap
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

/// Result type for pathfinding - a list of points from start to goal.
pub type PathResult = Option<Vec<Point>>;

/// Heuristic function for 4-directional movement.
///
/// Uses Manhattan distance: |dx| + |dy|
/// This is admissible for 4-directional movement because you must move
/// along the grid axes (no diagonal shortcuts).
fn heuristic(from: Point, to: Point) -> i32 {
    let dx = (from.x - to.x).abs();
    let dy = (from.y - to.y).abs();
    dx + dy // Manhattan distance for 4-directional movement
}

/// Heuristic function for 8-directional movement.
///
/// Uses Chebyshev distance: max(|dx|, |dy|)
/// This is admissible for 8-directional movement because you can move
/// diagonally, so the minimum number of steps is the larger of dx or dy.
fn heuristic_8dir(from: Point, to: Point) -> i32 {
    let dx = (from.x - to.x).abs();
    let dy = (from.y - to.y).abs();
    max(dx, dy) // Chebyshev distance for 8-directional movement
}

/// Finds the shortest path from `start` to `goal` using A* algorithm.
///
/// The `is_walkable` function should return `true` if a position can be traversed.
/// Movement is 4-directional (cardinal directions only).
///
/// # Arguments
///
/// * `start` - Starting position
/// * `goal` - Goal position
/// * `is_walkable` - Function that returns true if a position is walkable
///
/// # Returns
///
/// `Some(Vec<Point>)` containing the path from start to goal (inclusive),
/// or `None` if no path exists.
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
/// if let Some(path) = astar(start, goal, &is_walkable) {
///     println!("Found path with {} steps", path.len());
///     // Path includes both start and goal
///     assert_eq!(path[0], start);
///     assert_eq!(path[path.len() - 1], goal);
/// }
/// ```
pub fn astar<F>(start: Point, goal: Point, is_walkable: &F) -> PathResult
where
    F: Fn(Point) -> bool,
{
    // Early exit if start or goal is not walkable
    if !is_walkable(start) || !is_walkable(goal) {
        return None;
    }

    // Early exit if start equals goal
    if start == goal {
        return Some(vec![start]);
    }

    // Open set (nodes to explore) - using BinaryHeap as priority queue
    let mut open_set = BinaryHeap::new();

    // Closed set (already explored positions)
    let mut closed_set = HashSet::new();

    // Track best g-cost for each position
    let mut g_costs: HashMap<Point, i32> = HashMap::new();

    // Track parent for path reconstruction
    let mut parents: HashMap<Point, Point> = HashMap::new();

    // Add start node to open set
    let start_node = Node::new(start, 0, heuristic(start, goal), None);
    open_set.push(start_node);
    g_costs.insert(start, 0);

    // Main A* loop
    while let Some(current) = open_set.pop() {
        let current_pos = current.position;

        // Check if we reached the goal
        if current_pos == goal {
            // Reconstruct path
            let mut path = vec![goal];
            let mut current = goal;

            while let Some(&parent) = parents.get(&current) {
                path.push(parent);
                current = parent;
            }

            path.reverse();
            return Some(path);
        }

        // Skip if already explored
        if closed_set.contains(&current_pos) {
            continue;
        }

        // Mark as explored
        closed_set.insert(current_pos);

        // Explore neighbors (4-directional movement)
        for neighbor_pos in current_pos.cardinal_neighbors() {
            // Skip if not walkable or already explored
            if !is_walkable(neighbor_pos) || closed_set.contains(&neighbor_pos) {
                continue;
            }

            // Calculate g-cost (cost from start to neighbor through current)
            let tentative_g_cost = current.g_cost + 1; // Cost of 1 for each step

            // Check if this path to neighbor is better than any previous one
            if let Some(&existing_g_cost) = g_costs.get(&neighbor_pos) {
                if tentative_g_cost >= existing_g_cost {
                    continue; // This path is not better
                }
            }

            // This is the best path to neighbor so far
            g_costs.insert(neighbor_pos, tentative_g_cost);
            parents.insert(neighbor_pos, current_pos);

            // Calculate h-cost and add to open set
            let h_cost = heuristic(neighbor_pos, goal);
            let neighbor_node =
                Node::new(neighbor_pos, tentative_g_cost, h_cost, Some(current_pos));
            open_set.push(neighbor_node);
        }
    }

    // No path found
    None
}

/// Finds the shortest path using 8-directional movement (including diagonals).
///
/// Diagonal moves cost the same as cardinal moves (1 step).
///
/// # Arguments
///
/// * `start` - Starting position
/// * `goal` - Goal position
/// * `is_walkable` - Function that returns true if a position is walkable
///
/// # Example
///
/// ```
/// use runeforge_pathfinding::astar_8dir;
/// use runeforge_geometry::Point;
///
/// let start = Point::new(0, 0);
/// let goal = Point::new(5, 5);
/// let is_walkable = |_p: Point| true;
///
/// if let Some(path) = astar_8dir(start, goal, &is_walkable) {
///     println!("Found 8-directional path with {} steps", path.len());
///     // 8-directional is often shorter than 4-directional
/// }
/// ```
pub fn astar_8dir<F>(start: Point, goal: Point, is_walkable: &F) -> PathResult
where
    F: Fn(Point) -> bool,
{
    // Similar to astar but uses all_neighbors() instead of cardinal_neighbors()

    if !is_walkable(start) || !is_walkable(goal) {
        return None;
    }

    if start == goal {
        return Some(vec![start]);
    }

    let mut open_set = BinaryHeap::new();
    let mut closed_set = HashSet::new();
    let mut g_costs: HashMap<Point, i32> = HashMap::new();
    let mut parents: HashMap<Point, Point> = HashMap::new();

    let start_node = Node::new(start, 0, heuristic_8dir(start, goal), None);
    open_set.push(start_node);
    g_costs.insert(start, 0);

    while let Some(current) = open_set.pop() {
        let current_pos = current.position;

        if current_pos == goal {
            let mut path = vec![goal];
            let mut current = goal;

            while let Some(&parent) = parents.get(&current) {
                path.push(parent);
                current = parent;
            }

            path.reverse();
            return Some(path);
        }

        if closed_set.contains(&current_pos) {
            continue;
        }

        closed_set.insert(current_pos);

        // Use all_neighbors() for 8-directional movement
        for neighbor_pos in current_pos.all_neighbors() {
            if !is_walkable(neighbor_pos) || closed_set.contains(&neighbor_pos) {
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

        let path = astar(start, goal, &is_walkable).expect("Should find path");

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

        let path = astar(start, goal, &is_walkable).expect("Should find path around wall");

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

        let path = astar(start, goal, &is_walkable);
        assert!(path.is_none()); // No path should exist
    }

    #[test]
    fn test_astar_start_equals_goal() {
        let start = Point::new(5, 5);
        let is_walkable = |_: Point| true;

        let path = astar(start, start, &is_walkable).expect("Should return single point");
        assert_eq!(path.len(), 1);
        assert_eq!(path[0], start);
    }

    #[test]
    fn test_astar_8dir_diagonal() {
        let start = Point::new(0, 0);
        let goal = Point::new(3, 3);
        let is_walkable = |_: Point| true;

        let path_4dir = astar(start, goal, &is_walkable).expect("Should find 4-dir path");
        let path_8dir = astar_8dir(start, goal, &is_walkable).expect("Should find 8-dir path");

        println!("Path 4dir: {:?}", path_4dir);
        println!("Path 8dir: {:?}", path_8dir);

        // 8-directional should be shorter (can cut diagonally)
        assert!(path_8dir.len() <= path_4dir.len());
    }
}
