//! Procedural noise generation for roguelike maps.
//!
//! # Overview
//!
//! `runeforge-noise` provides a simple interface for generating 2D noise maps using Perlin noise.
//! This is a fundamental tool for creating natural-looking procedural content.
//!
//! # Use Cases
//!
//! *   **Terrain:** Generating height maps for mountains and valleys.
//! *   **Biomes:** Creating regions for different environmental types.
//! *   **Caves:** As a base for carving out organic cave systems.
//! *   **Textures:** Generating procedural textures for items or effects.
//!
//! # Usage
//!
//! Add this to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! runeforge-noise = "0.1"
//! ```
//!
//! ## Basic Example
//!
//! ```rust
//! use runeforge_noise::NoiseMap;
//!
//! fn main() {
//!     // Create a 50x50 noise map with a specific seed
//!     let noise_map = NoiseMap::new(50, 50, 42);
//!
//!     // Get a noise value at a specific coordinate
//!     let value = noise_map.get(10, 20);
//!     assert!(value >= -1.0 && value <= 1.0);
//!
//!     // Use a threshold to determine tile types
//!     if noise_map.threshold(10, 20, 0.5) {
//!         // Place a mountain tile
//!     } else {
//!         // Place a grass tile
//!     }
//! }
//! ```

#![deny(missing_docs)]

use noise::{NoiseFn, Perlin};

/// A 2D grid of noise values generated using Perlin noise.
///
/// Values range from -1.0 to 1.0. Commonly used for:
/// - Terrain height maps
/// - Cave/dungeon density maps
/// - Biome distribution
/// - Moisture/temperature maps
pub struct NoiseMap {
    /// Width of the noise map
    width: usize,
    /// Height of the noise map
    height: usize,
    /// Pre-computed noise values for fast lookup
    values: Vec<f64>,
    /// Random seed used for generation
    seed: u32,
}

impl NoiseMap {
    /// Create a new noise map with the given dimensions and seed.
    ///
    /// # Arguments
    ///
    /// * `width` - Width of the noise map
    /// * `height` - Height of the noise map
    /// * `seed` - Random seed for reproducible generation
    ///
    /// # Example
    ///
    /// ```
    /// use runeforge_noise::NoiseMap;
    ///
    /// let noise_map = NoiseMap::new(100, 100, 12345);
    /// assert_eq!(noise_map.width(), 100);
    /// assert_eq!(noise_map.height(), 100);
    /// ```
    pub fn new(width: usize, height: usize, seed: u32) -> Self {
        let mut noise_map = Self {
            width,
            height,
            values: vec![0.0; width * height],
            seed,
        };
        noise_map.generate();
        noise_map
    }

    /// Generate noise values using Perlin noise.
    ///
    /// This fills the internal values array with noise in the range [-1.0, 1.0].
    /// The scale determines how "zoomed in" the noise appears - higher values
    /// create more variation over shorter distances.
    fn generate(&mut self) {
        // Create Perlin generator once for efficiency
        let perlin = Perlin::new(self.seed);

        for y in 0..self.height {
            for x in 0..self.width {
                let x_scaled = x as f64 / 20.0;
                let y_scaled = y as f64 / 20.0;
                self.values[y * self.width + x] = perlin.get([x_scaled, y_scaled]);
            }
        }
    }

    /// Get the noise value at the given coordinates.
    ///
    /// Returns a value in the range [-1.0, 1.0], or 0.0 if out of bounds.
    ///
    /// # Example
    ///
    /// ```
    /// use runeforge_noise::NoiseMap;
    ///
    /// let noise_map = NoiseMap::new(50, 50, 42);
    /// let value = noise_map.get(25, 25);
    /// assert!(value >= -1.0 && value <= 1.0);
    /// ```
    pub fn get(&self, x: usize, y: usize) -> f64 {
        if x >= self.width || y >= self.height {
            return 0.0;
        }
        self.values[y * self.width + x]
    }

    /// Get width of the noise map.
    pub fn width(&self) -> usize {
        self.width
    }

    /// Get height of the noise map.
    pub fn height(&self) -> usize {
        self.height
    }

    /// Get the seed used for generation.
    pub fn seed(&self) -> u32 {
        self.seed
    }

    /// Convert noise value to a binary threshold (true/false).
    ///
    /// This is useful for creating simple terrain features:
    /// - Values above threshold become `true` (e.g., land)
    /// - Values below threshold become `false` (e.g., water)
    ///
    /// # Example
    ///
    /// ```
    /// use runeforge_noise::NoiseMap;
    ///
    /// let noise_map = NoiseMap::new(50, 50, 42);
    ///
    /// // Create a simple land/water map using 0.0 threshold
    /// for y in 0..noise_map.height() {
    ///     for x in 0..noise_map.width() {
    ///         let is_land = noise_map.threshold(x, y, 0.0);
    ///         // Use is_land to determine tile type
    ///     }
    /// }
    /// ```
    pub fn threshold(&self, x: usize, y: usize, threshold: f64) -> bool {
        self.get(x, y) > threshold
    }

    /// Normalize noise value from [-1.0, 1.0] to [0.0, 1.0].
    ///
    /// Useful for visualization and when you need positive-only values.
    pub fn normalized(&self, x: usize, y: usize) -> f64 {
        (self.get(x, y) + 1.0) / 2.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_noise_map_creation() {
        let noise_map = NoiseMap::new(10, 10, 42);
        assert_eq!(noise_map.width(), 10);
        assert_eq!(noise_map.height(), 10);
        assert_eq!(noise_map.seed(), 42);
    }

    #[test]
    fn test_noise_map_bounds() {
        let noise_map = NoiseMap::new(10, 10, 42);
        assert_eq!(noise_map.get(100, 100), 0.0); // Out of bounds returns 0
    }

    #[test]
    fn test_normalized_range() {
        let noise_map = NoiseMap::new(10, 10, 42);
        let normalized = noise_map.normalized(5, 5);
        assert!((0.0..=1.0).contains(&normalized));
    }

    #[test]
    fn test_reproducible_generation() {
        let map1 = NoiseMap::new(20, 20, 12345);
        let map2 = NoiseMap::new(20, 20, 12345);

        // Same seed should produce identical results
        for y in 0..20 {
            for x in 0..20 {
                assert_eq!(map1.get(x, y), map2.get(x, y));
            }
        }
    }

    #[test]
    fn test_different_seeds() {
        let map1 = NoiseMap::new(20, 20, 1);
        let map2 = NoiseMap::new(20, 20, 2);

        // Different seeds should produce different results
        let mut differences = 0;
        for y in 0..20 {
            for x in 0..20 {
                if map1.get(x, y) != map2.get(x, y) {
                    differences += 1;
                }
            }
        }
        assert!(differences > 100); // Should have many differences
    }
}
