//! Cellular automata cave generation.
//!
//! This module provides organic cave generation using cellular automata.
//! The algorithm works by:
//! 1. Random initialization (40-55% walls)
//! 2. Iterative smoothing using neighbor counts
//! 3. Birth/survival rules (e.g., "become wall if 5+ neighbors are walls")
//!
//! # Example
//!
//! ```
//! use runeforge_algorithms::caves::{CaveConfig, CaveGenerator};
//! use runeforge_random::Rng;
//!
//! let config = CaveConfig::default();
//! let mut rng = Rng::new();
//! let cave = CaveGenerator::generate(80, 50, &config, &mut rng).unwrap();
//!
//! // Check if a tile is floor or wall
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

/// Errors that can occur during cave generation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CaveError {
    /// Invalid dimensions (width or height is zero).
    InvalidDimensions {
        /// Map width
        width: u32,
        /// Map height
        height: u32,
    },
}

impl fmt::Display for CaveError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CaveError::InvalidDimensions { width, height } => {
                write!(
                    f,
                    "Invalid cave dimensions: {}x{} (both must be > 0)",
                    width, height
                )
            }
        }
    }
}

impl std::error::Error for CaveError {}

/// Configuration for cellular automata cave generation.
#[derive(Debug, Clone)]
pub struct CaveConfig {
    /// Initial wall probability (0.0 to 1.0).
    pub initial_wall_probability: f32,
    /// Number of simulation iterations.
    pub iterations: u32,
    /// Neighbor threshold for becoming a wall (0-8 for Moore neighborhood).
    /// A cell becomes a wall if it has >= this many wall neighbors.
    pub wall_threshold: u32,
}

impl Default for CaveConfig {
    fn default() -> Self {
        Self {
            initial_wall_probability: 0.45,
            iterations: 5,
            wall_threshold: 5,
        }
    }
}

impl CaveConfig {
    /// Creates a new configuration with default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the initial wall probability.
    pub fn with_initial_wall_probability(mut self, probability: f32) -> Self {
        self.initial_wall_probability = probability.clamp(0.0, 1.0);
        self
    }

    /// Sets the number of iterations.
    pub fn with_iterations(mut self, iterations: u32) -> Self {
        self.iterations = iterations;
        self
    }

    /// Sets the wall threshold (clamped to 0-8 for Moore neighborhood).
    pub fn with_wall_threshold(mut self, threshold: u32) -> Self {
        self.wall_threshold = threshold.clamp(0, 8);
        self
    }
}

/// A generated cave map.
#[derive(Debug, Clone)]
pub struct CaveMap {
    width: u32,
    height: u32,
    /// Tile data: true = floor, false = wall.
    tiles: Vec<bool>,
}

impl CaveMap {
    /// Creates a new cave map filled with walls.
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            tiles: vec![false; (width * height) as usize],
        }
    }

    /// Returns the cave width.
    pub fn width(&self) -> u32 {
        self.width
    }

    /// Returns the cave height.
    pub fn height(&self) -> u32 {
        self.height
    }

    /// Returns true if the position is a floor tile.
    /// Out-of-bounds positions return false (treated as walls).
    pub fn is_floor(&self, x: u32, y: u32) -> bool {
        if x >= self.width || y >= self.height {
            return false;
        }
        self.tiles[self.index(x, y)]
    }

    /// Returns true if the position is a wall tile.
    /// Out-of-bounds positions return true (treated as walls).
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

    /// Calculates the linear index for (x, y) coordinates.
    #[inline]
    fn index(&self, x: u32, y: u32) -> usize {
        (y * self.width + x) as usize
    }

    /// Prints the cave to the console.
    pub fn print(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                print!("{}", if self.is_floor(x, y) { '.' } else { '#' });
            }
            println!();
        }
    }

    /// Returns the percentage of tiles that are floors.
    pub fn floor_percentage(&self) -> f32 {
        if self.tiles.is_empty() {
            return 0.0;
        }
        let floor_count = self.tiles.iter().filter(|&&is_floor| is_floor).count();
        floor_count as f32 / self.tiles.len() as f32
    }
}

/// Generator for cellular automata caves.
pub struct CaveGenerator;

impl CaveGenerator {
    /// Generates a cave using cellular automata.
    ///
    /// # Errors
    ///
    /// Returns `CaveError::InvalidDimensions` if width or height is zero.
    pub fn generate(
        width: u32,
        height: u32,
        config: &CaveConfig,
        rng: &mut Rng,
    ) -> Result<CaveMap, CaveError> {
        if width == 0 || height == 0 {
            return Err(CaveError::InvalidDimensions { width, height });
        }

        let mut cave = CaveMap::new(width, height);
        Self::initialize_random(&mut cave, config.initial_wall_probability, rng);
        Self::run_simulation(&mut cave, config.iterations, config.wall_threshold);

        Ok(cave)
    }

    /// Initializes the cave with random floor/wall tiles.
    fn initialize_random(cave: &mut CaveMap, wall_probability: f32, rng: &mut Rng) {
        let floor_probability = 1.0 - wall_probability;
        for y in 0..cave.height() {
            for x in 0..cave.width() {
                let is_floor = rng.chance(floor_probability);
                cave.set_tile(x, y, is_floor);
            }
        }
    }

    /// Runs the cellular automata simulation for the specified iterations.
    fn run_simulation(cave: &mut CaveMap, iterations: u32, wall_threshold: u32) {
        let size = (cave.width() * cave.height()) as usize;
        let mut buffer = vec![false; size];

        for _ in 0..iterations {
            for y in 0..cave.height() {
                for x in 0..cave.width() {
                    let idx = (y * cave.width() + x) as usize;
                    buffer[idx] = Self::should_be_floor(cave, x, y, wall_threshold);
                }
            }
            std::mem::swap(&mut cave.tiles, &mut buffer);
        }
    }

    /// Counts the number of wall neighbors around a position (Moore neighborhood).
    ///
    /// Uses the 8 surrounding cells. Tiles outside the map are treated as walls.
    fn count_wall_neighbors(cave: &CaveMap, x: u32, y: u32) -> u32 {
        let mut count = 0;

        for dy in -1..=1i32 {
            for dx in -1..=1i32 {
                if dx == 0 && dy == 0 {
                    continue;
                }

                let nx = x as i32 + dx;
                let ny = y as i32 + dy;

                // Bounds check first - out of bounds counts as wall
                if nx < 0 || ny < 0 || nx >= cave.width() as i32 || ny >= cave.height() as i32 {
                    count += 1;
                    continue;
                }

                if cave.is_wall(nx as u32, ny as u32) {
                    count += 1;
                }
            }
        }

        count
    }

    /// Determines if a cell should become a floor based on neighbor count.
    ///
    /// A cell becomes a floor if it has fewer than `threshold` wall neighbors.
    fn should_be_floor(cave: &CaveMap, x: u32, y: u32, threshold: u32) -> bool {
        let wall_count = Self::count_wall_neighbors(cave, x, y);
        wall_count < threshold
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_seeded_rng(seed: u64) -> Rng {
        Rng::with_seed(seed)
    }

    #[test]
    fn test_cave_config_default() {
        let config = CaveConfig::default();
        assert_eq!(config.iterations, 5);
        assert!((config.initial_wall_probability - 0.45).abs() < f32::EPSILON);
        assert_eq!(config.wall_threshold, 5);
    }

    #[test]
    fn test_cave_config_builder() {
        let config = CaveConfig::new()
            .with_initial_wall_probability(0.6)
            .with_iterations(10)
            .with_wall_threshold(4);

        assert!((config.initial_wall_probability - 0.6).abs() < f32::EPSILON);
        assert_eq!(config.iterations, 10);
        assert_eq!(config.wall_threshold, 4);
    }

    #[test]
    fn test_cave_config_clamping() {
        let config = CaveConfig::new()
            .with_initial_wall_probability(1.5)
            .with_wall_threshold(10);

        assert!((config.initial_wall_probability - 1.0).abs() < f32::EPSILON);
        assert_eq!(config.wall_threshold, 8);
    }

    #[test]
    fn test_cave_map_creation() {
        let cave = CaveMap::new(80, 50);
        assert_eq!(cave.width(), 80);
        assert_eq!(cave.height(), 50);

        // All tiles should be walls initially
        for y in 0..50 {
            for x in 0..80 {
                assert!(cave.is_wall(x, y));
                assert!(!cave.is_floor(x, y));
            }
        }
    }

    #[test]
    fn test_cave_map_set_tile() {
        let mut cave = CaveMap::new(10, 10);

        cave.set_tile(5, 5, true);
        assert!(cave.is_floor(5, 5));
        assert!(!cave.is_wall(5, 5));

        cave.set_tile(5, 5, false);
        assert!(!cave.is_floor(5, 5));
        assert!(cave.is_wall(5, 5));
    }

    #[test]
    fn test_cave_map_out_of_bounds() {
        let cave = CaveMap::new(10, 10);

        // Out of bounds should be treated as walls
        assert!(cave.is_wall(100, 100));
        assert!(!cave.is_floor(100, 100));
    }

    #[test]
    fn test_cave_generation_dimensions() {
        let config = CaveConfig::default();
        let mut rng = create_seeded_rng(12345);

        let cave = CaveGenerator::generate(80, 50, &config, &mut rng).unwrap();

        assert_eq!(cave.width(), 80);
        assert_eq!(cave.height(), 50);
    }

    #[test]
    fn test_cave_generation_invalid_dimensions() {
        let config = CaveConfig::default();
        let mut rng = create_seeded_rng(12345);

        let result = CaveGenerator::generate(0, 50, &config, &mut rng);
        assert!(matches!(result, Err(CaveError::InvalidDimensions { .. })));

        let result = CaveGenerator::generate(80, 0, &config, &mut rng);
        assert!(matches!(result, Err(CaveError::InvalidDimensions { .. })));
    }

    #[test]
    fn test_cave_generation_deterministic() {
        let config = CaveConfig::default();

        let mut rng1 = create_seeded_rng(12345);
        let cave1 = CaveGenerator::generate(40, 30, &config, &mut rng1).unwrap();

        let mut rng2 = create_seeded_rng(12345);
        let cave2 = CaveGenerator::generate(40, 30, &config, &mut rng2).unwrap();

        assert_eq!(cave1.tiles(), cave2.tiles());
    }

    #[test]
    fn test_cave_generation_produces_mixed_terrain() {
        let config = CaveConfig::default();
        let mut rng = create_seeded_rng(12345);

        let cave = CaveGenerator::generate(80, 50, &config, &mut rng).unwrap();

        let floor_pct = cave.floor_percentage();

        // Should have a reasonable mix of floors and walls (not all one type)
        assert!(floor_pct > 0.1, "Too few floors: {}", floor_pct);
        assert!(floor_pct < 0.9, "Too few walls: {}", floor_pct);
    }

    #[test]
    fn test_count_wall_neighbors_corner() {
        let cave = CaveMap::new(10, 10);
        // All walls, corner has 3 in-bounds neighbors + 5 out-of-bounds = 8
        let count = CaveGenerator::count_wall_neighbors(&cave, 0, 0);
        assert_eq!(count, 8);
    }

    #[test]
    fn test_count_wall_neighbors_center() {
        let mut cave = CaveMap::new(10, 10);
        // Set center to floor, surrounded by walls
        cave.set_tile(5, 5, true);
        let count = CaveGenerator::count_wall_neighbors(&cave, 5, 5);
        assert_eq!(count, 8);
    }

    #[test]
    fn test_count_wall_neighbors_all_floors() {
        let mut cave = CaveMap::new(10, 10);
        // Set all tiles to floor
        for y in 0..10 {
            for x in 0..10 {
                cave.set_tile(x, y, true);
            }
        }
        // Center tile should have 0 wall neighbors
        let count = CaveGenerator::count_wall_neighbors(&cave, 5, 5);
        assert_eq!(count, 0);
    }

    #[test]
    fn test_floor_percentage() {
        let mut cave = CaveMap::new(10, 10);
        assert!((cave.floor_percentage() - 0.0).abs() < f32::EPSILON);

        // Set half to floor
        for y in 0..5 {
            for x in 0..10 {
                cave.set_tile(x, y, true);
            }
        }
        assert!((cave.floor_percentage() - 0.5).abs() < f32::EPSILON);
    }
}
