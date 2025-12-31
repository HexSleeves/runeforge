//! Color manipulation and conversion for roguelike development.
//!
//! # Overview
//!
//! `runeforge-color` provides a lightweight, efficient color system designed for
//! roguelike development and terminal graphics. It supports:
//!
//! *   **RGBA Colors:** Standard 32-bit color representation.
//! *   **HSV Conversion:** Easy conversion between RGB and HSV color spaces.
//! *   **Blending & Manipulation:** Operations like linear interpolation (lerp), multiplication, and component-wise addition.
//! *   **Predefined Colors:** A set of standard terminal colors (e.g., `Color::RED`, `Color::DARK_GRAY`).
//!
//! # Usage
//!
//! Add this to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! runeforge-color = "0.1"
//! ```
//!
//! ## Basic Example
//!
//! ```rust
//! use runeforge_color::Color;
//!
//! fn main() {
//!     // Create colors
//!     let red = Color::RED;
//!     let blue = Color::rgb(0, 0, 255);
//!
//!     // Blend colors
//!     let purple = red.lerp(blue, 0.5);
//!     
//!     // Manipulate
//!     let dark_purple = purple.multiply(Color::grayscale(128));
//!     
//!     println!("Result: {}", dark_purple);
//! }
//! ```

#![deny(missing_docs)]

use std::fmt;
use std::ops::Add;

/// An RGBA color represented as four 8-bit unsigned integers.
///
/// This struct is `Copy`, `Clone`, and generally lightweight (4 bytes).
/// It is intended to be passed by value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "serialization",
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct Color {
    /// Red component (0-255)
    pub r: u8,
    /// Green component (0-255)
    pub g: u8,
    /// Blue component (0-255)
    pub b: u8,
    /// Alpha component (0-255, 255 = fully opaque)
    pub a: u8,
}

impl Color {
    /// Creates a new color from RGB components with full opacity (alpha = 255).
    ///
    /// # Arguments
    ///
    /// *   `r` - Red component (0-255)
    /// *   `g` - Green component (0-255)
    /// *   `b` - Blue component (0-255)
    ///
    /// # Returns
    ///
    /// A new `Color` instance with the specified RGB values and `a=255`.
    ///
    /// # Examples
    ///
    /// ```
    /// use runeforge_color::Color;
    ///
    /// let red = Color::rgb(255, 0, 0);
    /// assert_eq!(red.r, 255);
    /// assert_eq!(red.a, 255);
    /// ```
    #[inline]
    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 255 }
    }

    /// Creates a new color from RGBA components.
    ///
    /// # Arguments
    ///
    /// *   `r` - Red component (0-255)
    /// *   `g` - Green component (0-255)
    /// *   `b` - Blue component (0-255)
    /// *   `a` - Alpha component (0-255)
    ///
    /// # Returns
    ///
    /// A new `Color` instance with the specified RGBA values.
    ///
    /// # Examples
    ///
    /// ```
    /// use runeforge_color::Color;
    ///
    /// let semi_transparent_red = Color::rgba(255, 0, 0, 128);
    /// assert_eq!(semi_transparent_red.a, 128);
    /// ```
    #[inline]
    pub const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    /// Creates a grayscale color.
    ///
    /// This sets R, G, and B to the same value, with full opacity.
    ///
    /// # Arguments
    ///
    /// *   `value` - The intensity of the gray (0-255).
    ///
    /// # Returns
    ///
    /// A new `Color` instance where r=g=b=`value`.
    ///
    /// # Examples
    ///
    /// ```
    /// use runeforge_color::Color;
    ///
    /// let gray = Color::grayscale(128);
    /// assert_eq!(gray.r, 128);
    /// assert_eq!(gray.g, 128);
    /// assert_eq!(gray.b, 128);
    /// ```
    #[inline]
    pub const fn grayscale(value: u8) -> Self {
        Self::rgb(value, value, value)
    }

    /// Linear interpolation between two colors.
    ///
    /// # Arguments
    ///
    /// * `other` - The target color to interpolate towards.
    /// * `t` - Interpolation factor (0.0 = self, 1.0 = other). Clamped to [0.0, 1.0].
    ///
    /// # Returns
    ///
    /// A new `Color` that is a blend of `self` and `other`.
    ///
    /// # Examples
    ///
    /// ```
    /// use runeforge_color::Color;
    ///
    /// let black = Color::BLACK;
    /// let white = Color::WHITE;
    /// let gray = black.lerp(white, 0.5);
    /// assert_eq!(gray.r, 127);
    /// ```
    pub fn lerp(self, other: Self, t: f32) -> Self {
        let t = t.clamp(0.0, 1.0);
        let inv_t = 1.0 - t;

        Self::rgba(
            (self.r as f32 * inv_t + other.r as f32 * t) as u8,
            (self.g as f32 * inv_t + other.g as f32 * t) as u8,
            (self.b as f32 * inv_t + other.b as f32 * t) as u8,
            (self.a as f32 * inv_t + other.a as f32 * t) as u8,
        )
    }

    /// Multiply two colors component-wise.
    ///
    /// This is useful for applying a tint or lighting to a color.
    /// The operation is `(c1 * c2) / 255`.
    ///
    /// # Arguments
    ///
    /// *   `other` - The color to multiply with.
    ///
    /// # Returns
    ///
    /// A new `Color` representing the product of the two colors.
    ///
    /// # Examples
    ///
    /// ```
    /// use runeforge_color::Color;
    ///
    /// let red = Color::rgb(255, 0, 0);
    /// let half = Color::grayscale(128);
    /// let dark_red = red.multiply(half);
    /// assert_eq!(dark_red.r, 128);
    /// ```
    pub fn multiply(self, other: Self) -> Self {
        Self::rgba(
            ((self.r as u16 * other.r as u16) / 255) as u8,
            ((self.g as u16 * other.g as u16) / 255) as u8,
            ((self.b as u16 * other.b as u16) / 255) as u8,
            ((self.a as u16 * other.a as u16) / 255) as u8,
        )
    }

    /// Convert the color to HSV color space.
    ///
    /// # Returns
    ///
    /// A tuple containing:
    /// *   `hue`: 0.0 - 360.0
    /// *   `saturation`: 0.0 - 100.0
    /// *   `value`: 0.0 - 100.0
    ///
    /// # Examples
    ///
    /// ```
    /// use runeforge_color::Color;
    ///
    /// let red = Color::RED;
    /// let (h, s, v) = red.to_hsv();
    /// assert_eq!(h, 0.0);
    /// assert_eq!(s, 100.0);
    /// ```
    pub fn to_hsv(self) -> (f32, f32, f32) {
        let r = self.r as f32 / 255.0;
        let g = self.g as f32 / 255.0;
        let b = self.b as f32 / 255.0;

        let max = r.max(g).max(b);
        let min = r.min(g).min(b);
        let delta = max - min;

        let value = max * 100.0;

        if delta == 0.0 {
            return (0.0, 0.0, value);
        }

        let saturation = (delta / max) * 100.0;

        let hue = if max == r {
            60.0 * (((g - b) / delta) % 6.0)
        } else if max == g {
            60.0 * (((b - r) / delta) + 2.0)
        } else {
            60.0 * (((r - g) / delta) + 4.0)
        };

        let hue = if hue < 0.0 { hue + 360.0 } else { hue };

        (hue, saturation, value)
    }

    /// Create a color from HSV color space.
    ///
    /// # Arguments
    ///
    /// * `h` - Hue (0.0 - 360.0)
    /// * `s` - Saturation (0.0 - 100.0)
    /// * `v` - Value (0.0 - 100.0)
    ///
    /// # Returns
    ///
    /// A new `Color` instance.
    ///
    /// # Examples
    ///
    /// ```
    /// use runeforge_color::Color;
    ///
    /// // Create pure red from HSV
    /// let red = Color::from_hsv(0.0, 100.0, 100.0);
    /// assert_eq!(red, Color::RED);
    /// ```
    pub fn from_hsv(h: f32, s: f32, v: f32) -> Self {
        let s = (s / 100.0).clamp(0.0, 1.0);
        let v = (v / 100.0).clamp(0.0, 1.0);
        let h = h % 360.0;

        let c = v * s;
        let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
        let m = v - c;

        let (r, g, b) = if h < 60.0 {
            (c, x, 0.0)
        } else if h < 120.0 {
            (x, c, 0.0)
        } else if h < 180.0 {
            (0.0, c, x)
        } else if h < 240.0 {
            (0.0, x, c)
        } else if h < 300.0 {
            (x, 0.0, c)
        } else {
            (c, 0.0, x)
        };

        Self::rgb(
            ((r + m) * 255.0) as u8,
            ((g + m) * 255.0) as u8,
            ((b + m) * 255.0) as u8,
        )
    }

    // Named colors (common terminal colors)

    /// Black color (0, 0, 0)
    pub const BLACK: Self = Self::rgb(0, 0, 0);
    /// White color (255, 255, 255)
    pub const WHITE: Self = Self::rgb(255, 255, 255);
    /// Red color (255, 0, 0)
    pub const RED: Self = Self::rgb(255, 0, 0);
    /// Green color (0, 255, 0)
    pub const GREEN: Self = Self::rgb(0, 255, 0);
    /// Blue color (0, 0, 255)
    pub const BLUE: Self = Self::rgb(0, 0, 255);
    /// Yellow color (255, 255, 0)
    pub const YELLOW: Self = Self::rgb(255, 255, 0);
    /// Cyan color (0, 255, 255)
    pub const CYAN: Self = Self::rgb(0, 255, 255);
    /// Magenta color (255, 0, 255)
    pub const MAGENTA: Self = Self::rgb(255, 0, 255);
    /// Dark gray color (64, 64, 64)
    pub const DARK_GRAY: Self = Self::rgb(64, 64, 64);
    /// Gray color (128, 128, 128)
    pub const GRAY: Self = Self::rgb(128, 128, 128);
    /// Light gray color (192, 192, 192)
    pub const LIGHT_GRAY: Self = Self::rgb(192, 192, 192);
    /// Orange color (255, 165, 0)
    pub const ORANGE: Self = Self::rgb(255, 165, 0);
    /// Purple color (128, 0, 128)
    pub const PURPLE: Self = Self::rgb(128, 0, 128);
    /// Brown color (165, 42, 42)
    pub const BROWN: Self = Self::rgb(165, 42, 42);
    /// Pink color (255, 192, 203)
    pub const PINK: Self = Self::rgb(255, 192, 203);
}

impl Default for Color {
    fn default() -> Self {
        Self::WHITE
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.a == 255 {
            write!(f, "rgb({}, {}, {})", self.r, self.g, self.b)
        } else {
            write!(f, "rgba({}, {}, {}, {})", self.r, self.g, self.b, self.a)
        }
    }
}

impl From<(u8, u8, u8)> for Color {
    fn from((r, g, b): (u8, u8, u8)) -> Self {
        Self::rgb(r, g, b)
    }
}

impl From<(u8, u8, u8, u8)> for Color {
    fn from((r, g, b, a): (u8, u8, u8, u8)) -> Self {
        Self::rgba(r, g, b, a)
    }
}

impl From<[u8; 3]> for Color {
    fn from([r, g, b]: [u8; 3]) -> Self {
        Self::rgb(r, g, b)
    }
}

impl From<[u8; 4]> for Color {
    fn from([r, g, b, a]: [u8; 4]) -> Self {
        Self::rgba(r, g, b, a)
    }
}

impl Add for Color {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self::rgba(
            self.r.saturating_add(other.r),
            self.g.saturating_add(other.g),
            self.b.saturating_add(other.b),
            self.a.saturating_add(other.a),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_creation() {
        let c = Color::rgb(255, 128, 64);
        assert_eq!(c.r, 255);
        assert_eq!(c.g, 128);
        assert_eq!(c.b, 64);
        assert_eq!(c.a, 255);
    }

    #[test]
    fn test_grayscale() {
        let gray = Color::grayscale(100);
        assert_eq!(gray.r, 100);
        assert_eq!(gray.g, 100);
        assert_eq!(gray.b, 100);
    }

    #[test]
    fn test_lerp() {
        let black = Color::BLACK;
        let white = Color::WHITE;
        let gray = black.lerp(white, 0.5);
        assert_eq!(gray.r, 127);
        assert_eq!(gray.g, 127);
        assert_eq!(gray.b, 127);
    }

    #[test]
    fn test_hsv_conversion() {
        let red = Color::RED;
        let (h, s, v) = red.to_hsv();
        assert_eq!(h, 0.0);
        assert_eq!(s, 100.0);
        assert_eq!(v, 100.0);

        let from_hsv = Color::from_hsv(h, s, v);
        assert_eq!(from_hsv.r, red.r);
        assert_eq!(from_hsv.g, red.g);
        assert_eq!(from_hsv.b, red.b);
    }

    #[test]
    fn test_multiply() {
        let red = Color::rgb(255, 0, 0);
        let half = Color::grayscale(128);
        let dark_red = red.multiply(half);
        assert_eq!(dark_red.r, 128); // (255 * 128) / 255 = 128
        assert_eq!(dark_red.g, 0);
        assert_eq!(dark_red.b, 0);
    }
}
