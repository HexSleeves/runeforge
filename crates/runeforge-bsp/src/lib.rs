//! Binary space partitioning for dungeon generation.
//!
//! This crate provides BSP tree structures for procedural map generation
//! in roguelike games. BSP recursively divides space into smaller regions,
//! creating rooms in the leaves and connecting them with corridors.
//!
//! # Example
//!
//! ```
//! use runeforge_bsp::{BspConfig, DungeonGenerator};
//! use runeforge_random::Rng;
//!
//! let config = BspConfig::default();
//! let mut rng = Rng::new();
//! let dungeon = DungeonGenerator::generate(80, 50, &config, &mut rng);
//!
//! // Access generated rooms
//! for room in dungeon.rooms() {
//!     println!("Room at ({}, {}) size {}x{}", room.x, room.y, room.width, room.height);
//! }
//!
//! // Check if a tile is floor or wall
//! for y in 0..dungeon.height() {
//!     for x in 0..dungeon.width() {
//!         let tile = if dungeon.is_floor(x as i32, y as i32) { '.' } else { '#' };
//!         print!("{}", tile);
//!     }
//!     println!();
//! }
//! ```

#![deny(missing_docs)]

use runeforge_geometry::{Point, Rect};
use runeforge_random::Rng;

/// Configuration for BSP dungeon generation.
///
/// Controls the splitting behavior and room generation parameters.
#[derive(Debug, Clone)]
pub struct BspConfig {
    /// Minimum width for a BSP partition (must fit a room)
    pub min_partition_width: u32,
    /// Minimum height for a BSP partition (must fit a room)
    pub min_partition_height: u32,
    /// Minimum room width
    pub min_room_width: u32,
    /// Minimum room height
    pub min_room_height: u32,
    /// Room padding from partition edges
    pub room_padding: u32,
    /// Maximum recursion depth for splitting
    pub max_depth: u32,
    /// Split ratio range (0.3 to 0.7 means split between 30% and 70%)
    pub split_ratio_min: f32,
    /// Split ratio range maximum
    pub split_ratio_max: f32,
}

impl Default for BspConfig {
    fn default() -> Self {
        Self {
            min_partition_width: 10,
            min_partition_height: 10,
            min_room_width: 4,
            min_room_height: 4,
            room_padding: 1,
            max_depth: 5,
            split_ratio_min: 0.4,
            split_ratio_max: 0.6,
        }
    }
}

impl BspConfig {
    /// Creates a new configuration with default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the minimum partition size.
    pub fn with_min_partition_size(mut self, width: u32, height: u32) -> Self {
        self.min_partition_width = width;
        self.min_partition_height = height;
        self
    }

    /// Sets the minimum room size.
    pub fn with_min_room_size(mut self, width: u32, height: u32) -> Self {
        self.min_room_width = width;
        self.min_room_height = height;
        self
    }

    /// Sets the room padding from partition edges.
    pub fn with_room_padding(mut self, padding: u32) -> Self {
        self.room_padding = padding;
        self
    }

    /// Sets the maximum recursion depth.
    pub fn with_max_depth(mut self, depth: u32) -> Self {
        self.max_depth = depth;
        self
    }

    /// Sets the split ratio range.
    pub fn with_split_ratio(mut self, min: f32, max: f32) -> Self {
        self.split_ratio_min = min.clamp(0.1, 0.9);
        self.split_ratio_max = max.clamp(0.1, 0.9);
        self
    }
}

/// Direction of a BSP split.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SplitDirection {
    /// Split horizontally (creates top and bottom children)
    Horizontal,
    /// Split vertically (creates left and right children)
    Vertical,
}

/// A node in the BSP tree.
///
/// Each node represents a rectangular partition of the dungeon space.
/// Internal nodes have two children; leaf nodes contain rooms.
#[derive(Debug, Clone)]
pub struct BspNode {
    /// The bounds of this partition
    pub bounds: Rect,
    /// The room within this partition (only for leaf nodes)
    pub room: Option<Rect>,
    /// Left or top child
    left: Option<Box<BspNode>>,
    /// Right or bottom child
    right: Option<Box<BspNode>>,
    /// How this node was split (None for leaf nodes)
    split: Option<SplitDirection>,
}

impl BspNode {
    /// Creates a new leaf node with the given bounds.
    pub fn new(bounds: Rect) -> Self {
        Self {
            bounds,
            room: None,
            left: None,
            right: None,
            split: None,
        }
    }

    /// Returns true if this is a leaf node (has no children).
    pub fn is_leaf(&self) -> bool {
        self.left.is_none() && self.right.is_none()
    }

    /// Returns the left/top child if it exists.
    pub fn left(&self) -> Option<&BspNode> {
        self.left.as_deref()
    }

    /// Returns the right/bottom child if it exists.
    pub fn right(&self) -> Option<&BspNode> {
        self.right.as_deref()
    }

    /// Returns the split direction if this node was split.
    pub fn split_direction(&self) -> Option<SplitDirection> {
        self.split
    }

    /// Splits this node into two children.
    ///
    /// Returns true if the split was successful.
    pub fn split(&mut self, direction: SplitDirection, ratio: f32, config: &BspConfig) -> bool {
        if !self.is_leaf() {
            return false; // Already split
        }

        let ratio = ratio.clamp(0.1, 0.9);

        match direction {
            SplitDirection::Horizontal => {
                let split_y = self.bounds.y + (self.bounds.height as f32 * ratio) as i32;
                let top_height = (split_y - self.bounds.y) as u32;
                let bottom_height = self.bounds.height - top_height;

                // Check minimum sizes
                if top_height < config.min_partition_height
                    || bottom_height < config.min_partition_height
                {
                    return false;
                }

                let top = Rect::new(self.bounds.x, self.bounds.y, self.bounds.width, top_height);
                let bottom = Rect::new(self.bounds.x, split_y, self.bounds.width, bottom_height);

                self.left = Some(Box::new(BspNode::new(top)));
                self.right = Some(Box::new(BspNode::new(bottom)));
            }
            SplitDirection::Vertical => {
                let split_x = self.bounds.x + (self.bounds.width as f32 * ratio) as i32;
                let left_width = (split_x - self.bounds.x) as u32;
                let right_width = self.bounds.width - left_width;

                // Check minimum sizes
                if left_width < config.min_partition_width
                    || right_width < config.min_partition_width
                {
                    return false;
                }

                let left = Rect::new(self.bounds.x, self.bounds.y, left_width, self.bounds.height);
                let right = Rect::new(split_x, self.bounds.y, right_width, self.bounds.height);

                self.left = Some(Box::new(BspNode::new(left)));
                self.right = Some(Box::new(BspNode::new(right)));
            }
        }

        self.split = Some(direction);
        true
    }

    /// Recursively splits this node until minimum size or max depth is reached.
    pub fn split_recursive(&mut self, config: &BspConfig, rng: &mut Rng, depth: u32) {
        if depth >= config.max_depth {
            return;
        }

        // Decide split direction based on aspect ratio with some randomness
        let aspect = self.bounds.width as f32 / self.bounds.height as f32;
        let direction = if aspect > 1.25 {
            SplitDirection::Vertical
        } else if aspect < 0.75 {
            SplitDirection::Horizontal
        } else {
            // Square-ish: random direction
            if rng.chance(0.5) {
                SplitDirection::Horizontal
            } else {
                SplitDirection::Vertical
            }
        };

        // Random split ratio
        let ratio = config.split_ratio_min
            + rng.float() * (config.split_ratio_max - config.split_ratio_min);

        if self.split(direction, ratio, config) {
            // Recursively split children
            if let Some(ref mut left) = self.left {
                left.split_recursive(config, rng, depth + 1);
            }
            if let Some(ref mut right) = self.right {
                right.split_recursive(config, rng, depth + 1);
            }
        }
    }

    /// Creates a room within this leaf node.
    pub fn create_room(&mut self, config: &BspConfig, rng: &mut Rng) {
        if !self.is_leaf() {
            return;
        }

        let padding = config.room_padding;
        let min_w = config.min_room_width;
        let min_h = config.min_room_height;

        // Available space after padding
        let available_w = self.bounds.width.saturating_sub(padding * 2);
        let available_h = self.bounds.height.saturating_sub(padding * 2);

        if available_w < min_w || available_h < min_h {
            return; // Not enough space for a room
        }

        // Random room size within available space (cap at available)
        let room_w = rng.range(min_w as i32, (available_w + 1) as i32) as u32;
        let room_w = room_w.min(available_w);
        let room_h = rng.range(min_h as i32, (available_h + 1) as i32) as u32;
        let room_h = room_h.min(available_h);

        // Random position within padded bounds (use saturating_sub to prevent overflow)
        let x_range = available_w.saturating_sub(room_w);
        let y_range = available_h.saturating_sub(room_h);

        let min_x = self.bounds.x + padding as i32;
        let min_y = self.bounds.y + padding as i32;

        let room_x = if x_range > 0 {
            rng.range(min_x, min_x + x_range as i32 + 1)
        } else {
            min_x
        };
        let room_y = if y_range > 0 {
            rng.range(min_y, min_y + y_range as i32 + 1)
        } else {
            min_y
        };

        self.room = Some(Rect::new(room_x, room_y, room_w, room_h));
    }

    /// Creates rooms in all leaf nodes.
    pub fn create_rooms_recursive(&mut self, config: &BspConfig, rng: &mut Rng) {
        if self.is_leaf() {
            self.create_room(config, rng);
        } else {
            if let Some(ref mut left) = self.left {
                left.create_rooms_recursive(config, rng);
            }
            if let Some(ref mut right) = self.right {
                right.create_rooms_recursive(config, rng);
            }
        }
    }

    /// Collects all rooms from leaf nodes.
    pub fn collect_rooms(&self) -> Vec<Rect> {
        let mut rooms = Vec::new();
        self.collect_rooms_recursive(&mut rooms);
        rooms
    }

    fn collect_rooms_recursive(&self, rooms: &mut Vec<Rect>) {
        if let Some(room) = self.room {
            rooms.push(room);
        }
        if let Some(ref left) = self.left {
            left.collect_rooms_recursive(rooms);
        }
        if let Some(ref right) = self.right {
            right.collect_rooms_recursive(rooms);
        }
    }

    /// Gets a room from this subtree (for corridor generation).
    pub fn get_room(&self) -> Option<Rect> {
        if let Some(room) = self.room {
            return Some(room);
        }
        // Try to find a room in children
        if let Some(ref left) = self.left {
            if let Some(room) = left.get_room() {
                return Some(room);
            }
        }
        if let Some(ref right) = self.right {
            if let Some(room) = right.get_room() {
                return Some(room);
            }
        }
        None
    }
}

/// A corridor connecting two points in the dungeon.
#[derive(Debug, Clone)]
pub struct Corridor {
    /// Start point of the corridor
    pub start: Point,
    /// End point of the corridor
    pub end: Point,
    /// The corner point for L-shaped corridors
    pub corner: Option<Point>,
}

impl Corridor {
    /// Creates a new straight corridor.
    pub fn straight(start: Point, end: Point) -> Self {
        Self {
            start,
            end,
            corner: None,
        }
    }

    /// Creates an L-shaped corridor.
    pub fn l_shaped(start: Point, corner: Point, end: Point) -> Self {
        Self {
            start,
            end,
            corner: Some(corner),
        }
    }

    /// Returns all points along this corridor.
    pub fn points(&self) -> Vec<Point> {
        let mut pts = Vec::new();

        if let Some(corner) = self.corner {
            // L-shaped: start -> corner -> end
            Self::line_points(self.start, corner, &mut pts);
            Self::line_points(corner, self.end, &mut pts);
        } else {
            // Straight line
            Self::line_points(self.start, self.end, &mut pts);
        }

        pts
    }

    fn line_points(from: Point, to: Point, pts: &mut Vec<Point>) {
        // Horizontal segment
        let x_start = from.x.min(to.x);
        let x_end = from.x.max(to.x);
        for x in x_start..=x_end {
            pts.push(Point::new(x, from.y));
        }

        // Vertical segment
        let y_start = from.y.min(to.y);
        let y_end = from.y.max(to.y);
        for y in y_start..=y_end {
            pts.push(Point::new(to.x, y));
        }
    }
}

/// A generated dungeon with rooms and corridors.
#[derive(Debug, Clone)]
pub struct Dungeon {
    width: u32,
    height: u32,
    tiles: Vec<bool>, // true = floor, false = wall
    rooms: Vec<Rect>,
    corridors: Vec<Corridor>,
}

impl Dungeon {
    /// Creates a new empty dungeon (all walls).
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            tiles: vec![false; (width * height) as usize],
            rooms: Vec::new(),
            corridors: Vec::new(),
        }
    }

    /// Returns the dungeon width.
    pub fn width(&self) -> u32 {
        self.width
    }

    /// Returns the dungeon height.
    pub fn height(&self) -> u32 {
        self.height
    }

    /// Returns true if the position is a floor tile.
    pub fn is_floor(&self, x: i32, y: i32) -> bool {
        if x < 0 || y < 0 || x >= self.width as i32 || y >= self.height as i32 {
            return false;
        }
        self.tiles[(y as u32 * self.width + x as u32) as usize]
    }

    /// Returns true if the position is walkable (a floor tile).
    pub fn is_walkable(&self, pos: Point) -> bool {
        self.is_floor(pos.x, pos.y)
    }

    /// Sets a tile to floor.
    pub fn set_floor(&mut self, x: i32, y: i32) {
        if x >= 0 && y >= 0 && x < self.width as i32 && y < self.height as i32 {
            self.tiles[(y as u32 * self.width + x as u32) as usize] = true;
        }
    }

    /// Carves out a room (sets all tiles to floor).
    pub fn carve_room(&mut self, room: Rect) {
        for point in room.points() {
            self.set_floor(point.x, point.y);
        }
        self.rooms.push(room);
    }

    /// Carves out a corridor.
    pub fn carve_corridor(&mut self, corridor: &Corridor) {
        for point in corridor.points() {
            self.set_floor(point.x, point.y);
        }
        self.corridors.push(corridor.clone());
    }

    /// Returns all rooms in the dungeon.
    pub fn rooms(&self) -> &[Rect] {
        &self.rooms
    }

    /// Returns all corridors in the dungeon.
    pub fn corridors(&self) -> &[Corridor] {
        &self.corridors
    }

    /// Returns the tiles as a slice.
    pub fn tiles(&self) -> &[bool] {
        &self.tiles
    }

    /// Returns a random floor position (useful for spawning entities).
    pub fn random_floor_position(&self, rng: &mut Rng) -> Option<Point> {
        let floor_tiles: Vec<Point> = (0..self.height)
            .flat_map(|y| {
                (0..self.width).filter_map(move |x| {
                    if self.is_floor(x as i32, y as i32) {
                        Some(Point::new(x as i32, y as i32))
                    } else {
                        None
                    }
                })
            })
            .collect();

        rng.choose(&floor_tiles).copied()
    }
}

/// Generator for BSP-based dungeons.
pub struct DungeonGenerator;

impl DungeonGenerator {
    /// Generates a dungeon using BSP.
    ///
    /// # Arguments
    ///
    /// * `width` - Dungeon width in tiles
    /// * `height` - Dungeon height in tiles
    /// * `config` - BSP configuration
    /// * `rng` - Random number generator
    ///
    /// # Returns
    ///
    /// A `Dungeon` with rooms and corridors carved out.
    pub fn generate(width: u32, height: u32, config: &BspConfig, rng: &mut Rng) -> Dungeon {
        let mut dungeon = Dungeon::new(width, height);

        // Create root BSP node covering the entire dungeon
        let root_bounds = Rect::new(0, 0, width, height);
        let mut root = BspNode::new(root_bounds);

        // Recursively split the space
        root.split_recursive(config, rng, 0);

        // Create rooms in leaf nodes
        root.create_rooms_recursive(config, rng);

        // Carve rooms into dungeon
        for room in root.collect_rooms() {
            dungeon.carve_room(room);
        }

        // Generate corridors between sibling rooms
        Self::generate_corridors(&root, &mut dungeon, rng);

        dungeon
    }

    /// Generates corridors by traversing the BSP tree.
    fn generate_corridors(node: &BspNode, dungeon: &mut Dungeon, rng: &mut Rng) {
        if node.is_leaf() {
            return;
        }

        // Connect children
        if let (Some(left), Some(right)) = (node.left(), node.right()) {
            // Get a room from each subtree
            if let (Some(left_room), Some(right_room)) = (left.get_room(), right.get_room()) {
                let corridor = Self::create_corridor(left_room, right_room, rng);
                dungeon.carve_corridor(&corridor);
            }

            // Recursively process children
            Self::generate_corridors(left, dungeon, rng);
            Self::generate_corridors(right, dungeon, rng);
        }
    }

    /// Creates a corridor between two rooms.
    fn create_corridor(room1: Rect, room2: Rect, rng: &mut Rng) -> Corridor {
        let center1 = room1.center();
        let center2 = room2.center();

        // Randomly choose horizontal-first or vertical-first
        if rng.chance(0.5) {
            // Horizontal first, then vertical
            let corner = Point::new(center2.x, center1.y);
            Corridor::l_shaped(center1, corner, center2)
        } else {
            // Vertical first, then horizontal
            let corner = Point::new(center1.x, center2.y);
            Corridor::l_shaped(center1, corner, center2)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bsp_config_default() {
        let config = BspConfig::default();
        assert_eq!(config.min_partition_width, 10);
        assert_eq!(config.min_partition_height, 10);
        assert_eq!(config.min_room_width, 4);
        assert_eq!(config.max_depth, 5);
    }

    #[test]
    fn test_bsp_config_builder() {
        let config = BspConfig::new()
            .with_min_partition_size(15, 15)
            .with_min_room_size(5, 5)
            .with_max_depth(4);

        assert_eq!(config.min_partition_width, 15);
        assert_eq!(config.min_room_width, 5);
        assert_eq!(config.max_depth, 4);
    }

    #[test]
    fn test_bsp_node_creation() {
        let bounds = Rect::new(0, 0, 100, 100);
        let node = BspNode::new(bounds);

        assert!(node.is_leaf());
        assert_eq!(node.bounds.width, 100);
        assert!(node.room.is_none());
    }

    #[test]
    fn test_bsp_node_split_vertical() {
        let bounds = Rect::new(0, 0, 100, 50);
        let mut node = BspNode::new(bounds);
        let config = BspConfig::default();

        let success = node.split(SplitDirection::Vertical, 0.5, &config);

        assert!(success);
        assert!(!node.is_leaf());
        assert!(node.left().is_some());
        assert!(node.right().is_some());

        let left = node.left().unwrap();
        let right = node.right().unwrap();

        assert_eq!(left.bounds.width, 50);
        assert_eq!(right.bounds.width, 50);
        assert_eq!(right.bounds.x, 50);
    }

    #[test]
    fn test_bsp_node_split_horizontal() {
        let bounds = Rect::new(0, 0, 50, 100);
        let mut node = BspNode::new(bounds);
        let config = BspConfig::default();

        let success = node.split(SplitDirection::Horizontal, 0.5, &config);

        assert!(success);

        let left = node.left().unwrap();
        let right = node.right().unwrap();

        assert_eq!(left.bounds.height, 50);
        assert_eq!(right.bounds.height, 50);
        assert_eq!(right.bounds.y, 50);
    }

    #[test]
    fn test_bsp_node_split_too_small() {
        let bounds = Rect::new(0, 0, 15, 15);
        let mut node = BspNode::new(bounds);
        let config = BspConfig::default(); // min_partition = 10

        // Splitting at 0.5 would create 7.5 + 7.5, both < 10
        let success = node.split(SplitDirection::Vertical, 0.5, &config);

        // Should fail because resulting partitions are too small
        assert!(!success);
        assert!(node.is_leaf());
    }

    #[test]
    fn test_dungeon_generation() {
        let config = BspConfig::default();
        let mut rng = Rng::new();

        let dungeon = DungeonGenerator::generate(80, 50, &config, &mut rng);

        assert_eq!(dungeon.width(), 80);
        assert_eq!(dungeon.height(), 50);
        assert!(!dungeon.rooms().is_empty());
        assert!(!dungeon.corridors().is_empty());
    }

    #[test]
    fn test_dungeon_has_floor_tiles() {
        let config = BspConfig::default();
        let mut rng = Rng::new();

        let dungeon = DungeonGenerator::generate(80, 50, &config, &mut rng);

        // Count floor tiles
        let floor_count: usize = dungeon.tiles().iter().filter(|&&t| t).count();

        assert!(floor_count > 0, "Dungeon should have floor tiles");
    }

    #[test]
    fn test_rooms_are_within_bounds() {
        let config = BspConfig::default();
        let mut rng = Rng::new();

        let dungeon = DungeonGenerator::generate(80, 50, &config, &mut rng);

        for room in dungeon.rooms() {
            assert!(room.x >= 0);
            assert!(room.y >= 0);
            assert!(room.x + room.width as i32 <= 80);
            assert!(room.y + room.height as i32 <= 50);
        }
    }

    #[test]
    fn test_corridor_points() {
        let start = Point::new(5, 5);
        let end = Point::new(10, 10);
        let corner = Point::new(10, 5);

        let corridor = Corridor::l_shaped(start, corner, end);
        let points = corridor.points();

        assert!(!points.is_empty());
        assert!(points.contains(&start));
        assert!(points.contains(&end));
        assert!(points.contains(&corner));
    }

    #[test]
    fn test_random_floor_position() {
        let config = BspConfig::default();
        let mut rng = Rng::new();

        let dungeon = DungeonGenerator::generate(80, 50, &config, &mut rng);

        let pos = dungeon.random_floor_position(&mut rng);
        assert!(pos.is_some());

        let pos = pos.unwrap();
        assert!(dungeon.is_floor(pos.x, pos.y));
    }

    #[test]
    fn test_small_dungeon() {
        let config = BspConfig::new()
            .with_min_partition_size(5, 5)
            .with_min_room_size(3, 3)
            .with_max_depth(2);

        let mut rng = Rng::new();
        let dungeon = DungeonGenerator::generate(30, 30, &config, &mut rng);

        assert!(!dungeon.rooms().is_empty());
    }
}
