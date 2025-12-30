//! Drunkard's Walk cave generation.
//!
//! This module provides irregular cave generation using a random walker algorithm.
//! The algorithm works by:
//! 1. Starting at a random or center position
//! 2. Taking random steps in cardinal directions
//! 3. Carving floor tiles along the path
//! 4. Continuing until a target percentage of the map is floor
//!
//! # Example
//!
//! ```
//! use runeforge_algorithms::drunkard::{DrunkardConfig, DrunkardGenerator};
//! use runeforge_random::Rng;
//!
//! let config = DrunkardConfig::default();
//! let mut rng = Rng::new();
//! let cave = DrunkardGenerator::generate(80, 50, &config, &mut rng).unwrap();
//!
//! for y in 0..cave.height() {
//!     for x in 0..cave.width() {
//!         let tile = if cave.is_floor(x, y) { '.' } else { '#' };
//!         print!("{}", tile);
//!     }
//!     println!();
//! }
//! ```

use runeforge_random::Rng;
use std::fmt;

/// Errors that can occur during drunkard's walk generation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DrunkardError {
    /// Invalid dimensions (width or height is zero).
    InvalidDimensions {
        /// Map width
        width: u32,
        /// Map height
        height: u32,
    },
}

impl fmt::Display for DrunkardError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DrunkardError::InvalidDimensions { width, height } => {
                write!(
                    f,
                    "Invalid map dimensions: {}x{} (both must be > 0)",
                    width, height
                )
            }
        }
    }
}

impl std::error::Error for DrunkardError {}

/// Configuration for drunkard's walk cave generation.
#[derive(Debug, Clone)]
pub struct DrunkardConfig {
    /// Target percentage of map that should be floor (0.0 to 1.0).
    pub target_floor_percentage: f32,
    /// Where the drunkard starts walking.
    pub start_position: StartPosition,
    /// Maximum steps before giving up (prevents infinite loops on small maps).
    pub max_steps: u32,
}

/// Where the drunkard starts walking.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StartPosition {
    /// Start at the center of the map.
    Center,
    /// Start at a random position.
    Random,
}

impl Default for DrunkardConfig {
    fn default() -> Self {
        Self {
            target_floor_percentage: 0.4,
            start_position: StartPosition::Center,
            max_steps: 100_000,
        }
    }
}

impl DrunkardConfig {
    /// Creates a new configuration with default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the target floor percentage (clamped to 0.0-1.0).
    pub fn with_target_floor_percentage(mut self, percentage: f32) -> Self {
        self.target_floor_percentage = percentage.clamp(0.0, 1.0);
        self
    }

    /// Sets the start position.
    pub fn with_start_position(mut self, position: StartPosition) -> Self {
        self.start_position = position;
        self
    }

    /// Sets the maximum number of steps before termination.
    pub fn with_max_steps(mut self, max_steps: u32) -> Self {
        self.max_steps = max_steps;
        self
    }
}

/// A generated cave map using drunkard's walk.
#[derive(Debug, Clone)]
pub struct DrunkardMap {
    width: u32,
    height: u32,
    tiles: Vec<bool>,
}

impl DrunkardMap {
    /// Creates a new drunkard map filled with walls.
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            tiles: vec![false; (width * height) as usize],
        }
    }

    /// Returns the map width.
    pub fn width(&self) -> u32 {
        self.width
    }

    /// Returns the map height.
    pub fn height(&self) -> u32 {
        self.height
    }

    /// Returns true if the position is a floor tile.
    /// Out-of-bounds positions return false.
    pub fn is_floor(&self, x: u32, y: u32) -> bool {
        if x >= self.width || y >= self.height {
            return false;
        }
        self.tiles[self.index(x, y)]
    }

    /// Returns true if the position is a wall tile.
    pub fn is_wall(&self, x: u32, y: u32) -> bool {
        !self.is_floor(x, y)
    }

    /// Sets a tile to floor or wall.
    pub fn set_tile(&mut self, x: u32, y: u32, is_floor: bool) {
        if x < self.width && y < self.height {
            let idx = self.index(x, y);
            self.tiles[idx] = is_floor;
        }
    }

    /// Returns the tiles as a slice.
    pub fn tiles(&self) -> &[bool] {
        &self.tiles
    }

    /// Counts the number of floor tiles.
    pub fn floor_count(&self) -> usize {
        self.tiles.iter().filter(|&&t| t).count()
    }

    /// Returns the current floor percentage (0.0 to 1.0).
    pub fn floor_percentage(&self) -> f32 {
        if self.tiles.is_empty() {
            return 0.0;
        }
        self.floor_count() as f32 / self.tiles.len() as f32
    }

    /// Calculates the linear index for (x, y) coordinates.
    #[inline]
    fn index(&self, x: u32, y: u32) -> usize {
        (y * self.width + x) as usize
    }

    /// Prints the map to the console.
    pub fn print(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                print!("{}", if self.is_floor(x, y) { '.' } else { '#' });
            }
            println!();
        }
    }
}

/// Cardinal directions for movement.
#[derive(Debug, Clone, Copy)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    /// Returns a random direction.
    fn random(rng: &mut Rng) -> Self {
        match rng.range(0, 3) {
            0 => Direction::North,
            1 => Direction::South,
            2 => Direction::East,
            _ => Direction::West,
        }
    }

    /// Returns the delta (dx, dy) for this direction.
    fn delta(self) -> (i32, i32) {
        match self {
            Direction::North => (0, -1),
            Direction::South => (0, 1),
            Direction::East => (1, 0),
            Direction::West => (-1, 0),
        }
    }
}

/// Generator for drunkard's walk caves.
pub struct DrunkardGenerator;

impl DrunkardGenerator {
    /// Generates a cave using the drunkard's walk algorithm.
    ///
    /// # Errors
    ///
    /// Returns `DrunkardError::InvalidDimensions` if width or height is zero.
    pub fn generate(
        width: u32,
        height: u32,
        config: &DrunkardConfig,
        rng: &mut Rng,
    ) -> Result<DrunkardMap, DrunkardError> {
        if width == 0 || height == 0 {
            return Err(DrunkardError::InvalidDimensions { width, height });
        }

        let mut map = DrunkardMap::new(width, height);

        let (mut x, mut y) = match config.start_position {
            StartPosition::Center => (width / 2, height / 2),
            StartPosition::Random => (
                rng.range(0, (width - 1) as i32) as u32,
                rng.range(0, (height - 1) as i32) as u32,
            ),
        };

        map.set_tile(x, y, true);

        let mut steps = 0;
        while map.floor_percentage() < config.target_floor_percentage && steps < config.max_steps {
            (x, y) = Self::take_step(&map, x, y, rng);
            map.set_tile(x, y, true);
            steps += 1;
        }

        Ok(map)
    }

    /// Takes a single step in a random direction.
    ///
    /// Returns the new position after the step, clamped to map bounds.
    fn take_step(map: &DrunkardMap, x: u32, y: u32, rng: &mut Rng) -> (u32, u32) {
        let direction = Direction::random(rng);
        let (dx, dy) = direction.delta();

        // Use signed arithmetic to avoid underflow
        let new_x = (x as i32 + dx).clamp(0, (map.width() - 1) as i32) as u32;
        let new_y = (y as i32 + dy).clamp(0, (map.height() - 1) as i32) as u32;

        (new_x, new_y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn seeded_rng(seed: u64) -> Rng {
        Rng::with_seed(seed)
    }

    #[test]
    fn test_drunkard_config_default() {
        let config = DrunkardConfig::default();
        assert!((config.target_floor_percentage - 0.4).abs() < f32::EPSILON);
        assert_eq!(config.start_position, StartPosition::Center);
        assert_eq!(config.max_steps, 100_000);
    }

    #[test]
    fn test_drunkard_config_builder() {
        let config = DrunkardConfig::new()
            .with_target_floor_percentage(0.6)
            .with_start_position(StartPosition::Random)
            .with_max_steps(50_000);

        assert!((config.target_floor_percentage - 0.6).abs() < f32::EPSILON);
        assert_eq!(config.start_position, StartPosition::Random);
        assert_eq!(config.max_steps, 50_000);
    }

    #[test]
    fn test_drunkard_config_clamping() {
        let config = DrunkardConfig::new().with_target_floor_percentage(1.5);
        assert!((config.target_floor_percentage - 1.0).abs() < f32::EPSILON);

        let config = DrunkardConfig::new().with_target_floor_percentage(-0.5);
        assert!((config.target_floor_percentage - 0.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_drunkard_map_creation() {
        let map = DrunkardMap::new(80, 50);
        assert_eq!(map.width(), 80);
        assert_eq!(map.height(), 50);
        assert!((map.floor_percentage() - 0.0).abs() < f32::EPSILON);

        // All tiles should be walls initially
        for y in 0..50 {
            for x in 0..80 {
                assert!(map.is_wall(x, y));
            }
        }
    }

    #[test]
    fn test_drunkard_map_out_of_bounds() {
        let map = DrunkardMap::new(10, 10);
        assert!(!map.is_floor(100, 100));
        assert!(map.is_wall(100, 100));
    }

    #[test]
    fn test_drunkard_generation_invalid_dimensions() {
        let config = DrunkardConfig::default();
        let mut rng = seeded_rng(12345);

        let result = DrunkardGenerator::generate(0, 50, &config, &mut rng);
        assert!(matches!(
            result,
            Err(DrunkardError::InvalidDimensions { .. })
        ));

        let result = DrunkardGenerator::generate(80, 0, &config, &mut rng);
        assert!(matches!(
            result,
            Err(DrunkardError::InvalidDimensions { .. })
        ));
    }

    #[test]
    fn test_drunkard_generation_dimensions() {
        let config = DrunkardConfig::default();
        let mut rng = seeded_rng(12345);

        let map = DrunkardGenerator::generate(80, 50, &config, &mut rng).unwrap();

        assert_eq!(map.width(), 80);
        assert_eq!(map.height(), 50);
    }

    #[test]
    fn test_drunkard_generation_deterministic() {
        let config = DrunkardConfig::default();

        let mut rng1 = seeded_rng(42);
        let map1 = DrunkardGenerator::generate(40, 30, &config, &mut rng1).unwrap();

        let mut rng2 = seeded_rng(42);
        let map2 = DrunkardGenerator::generate(40, 30, &config, &mut rng2).unwrap();

        assert_eq!(map1.tiles(), map2.tiles());
    }

    #[test]
    fn test_drunkard_generation_reaches_target() {
        let config = DrunkardConfig::new().with_target_floor_percentage(0.3);
        let mut rng = seeded_rng(12345);

        let map = DrunkardGenerator::generate(80, 50, &config, &mut rng).unwrap();

        assert!(
            map.floor_percentage() >= 0.3,
            "Floor percentage {} should be >= 0.3",
            map.floor_percentage()
        );
    }

    #[test]
    fn test_drunkard_generation_center_start() {
        let config = DrunkardConfig::new()
            .with_start_position(StartPosition::Center)
            .with_target_floor_percentage(0.01);
        let mut rng = seeded_rng(12345);

        let map = DrunkardGenerator::generate(80, 50, &config, &mut rng).unwrap();

        // Center tile should definitely be floor
        assert!(map.is_floor(40, 25));
    }

    #[test]
    fn test_drunkard_max_steps_termination() {
        let config = DrunkardConfig::new()
            .with_target_floor_percentage(1.0) // Impossible to reach
            .with_max_steps(100);
        let mut rng = seeded_rng(12345);

        // Should terminate due to max_steps, not hang
        let map = DrunkardGenerator::generate(80, 50, &config, &mut rng).unwrap();

        // Should have carved some tiles but not reached 100%
        assert!(map.floor_percentage() < 1.0);
        assert!(map.floor_count() > 0);
    }

    #[test]
    fn test_direction_deltas() {
        assert_eq!(Direction::North.delta(), (0, -1));
        assert_eq!(Direction::South.delta(), (0, 1));
        assert_eq!(Direction::East.delta(), (1, 0));
        assert_eq!(Direction::West.delta(), (-1, 0));
    }

    #[test]
    fn test_take_step_bounds_clamping() {
        let map = DrunkardMap::new(10, 10);
        let mut rng = seeded_rng(12345);

        // Test many steps from corner - should never go out of bounds
        let mut x = 0u32;
        let mut y = 0u32;
        for _ in 0..1000 {
            (x, y) = DrunkardGenerator::take_step(&map, x, y, &mut rng);
            assert!(x < map.width(), "x={} out of bounds", x);
            assert!(y < map.height(), "y={} out of bounds", y);
        }
    }
}
