use crate::prelude::*;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// A 2D rectangle aligned to the grid axes.
///
/// Rectangles are defined by their minimum (top-left inclusive) and maximum (bottom-right inclusive) coordinates.
///
/// # Examples
///
/// ```
/// use runeforge_geometry::prelude::*;
/// use glam::IVec2;
///
/// let rect = Rect::new_xywh(0, 0, 10, 5);
/// assert_eq!(rect.width(), 10);
/// assert_eq!(rect.height(), 5);
/// ```
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Rect {
    /// The minimum coordinates of the rectangle.
    pub min: IVec2,
    /// The maximum coordinates of the rectangle.
    pub max: IVec2,
    /// The width of the rectangle.
    pub width: i32,
    /// The height of the rectangle.
    pub height: i32,
}

impl Default for Rect {
    fn default() -> Self {
        Self::new_with_size(IVec2::ZERO, UVec2::ONE)
    }
}

impl Rect {
    /// Creates a new rectangle from two points.
    ///
    /// The order of points does not matter; the constructor will automatically determine
    /// the min and max bounds.
    #[inline]
    pub fn new(min: IVec2, max: IVec2) -> Self {
        let min = min.min(max);
        let max = min.max(max);
        let width = max.x - min.x;
        let height = max.y - min.y;

        Self {
            width,
            height,
            min,
            max,
        }
    }

    /// Creates a new rectangle from position (x, y) and dimensions (width, height).
    ///
    /// # Arguments
    ///
    /// * `x` - X coordinate of top-left corner
    /// * `y` - Y coordinate of top-left corner
    /// * `width` - Width of the rectangle
    /// * `height` - Height of the rectangle
    #[inline]
    pub fn new_xywh(x: i32, y: i32, width: i32, height: i32) -> Self {
        Self::new(IVec2::new(x, y), IVec2::new(x + width, y + height))
    }

    /// Creates a new rectangle with the given size.
    #[inline]
    pub fn new_with_size(min: IVec2, dimensions: UVec2) -> Self {
        Self::new(min, min + dimensions.as_ivec2())
    }
}

impl Rect {
    /// Get the width of the rectangle.
    #[inline]
    pub const fn width(&self) -> i32 {
        self.width
    }

    /// Get the height of the rectangle.
    #[inline]
    pub const fn height(&self) -> i32 {
        self.height
    }

    /// Get the minimum point of the rectangle (top-left).
    #[inline]
    pub const fn min(&self) -> IVec2 {
        self.min
    }

    /// Get the maximum point of the rectangle (bottom-right).
    #[inline]
    pub const fn max(&self) -> IVec2 {
        self.max
    }

    /// Check if the rectangle is square (width == height).
    #[inline]
    pub fn is_square(&self) -> bool {
        let diff = self.max - self.min;
        diff.x == diff.y
    }
}

impl Rect {
    /// Returns the center point of the rectangle.
    #[inline]
    pub fn center(&self) -> IVec2 {
        self.min.mid_point(self.max)
    }

    /// Returns the X coordinate of the left edge.
    #[inline]
    pub fn left(&self) -> i32 {
        self.min.x.min(self.max.x)
    }

    /// Returns the X coordinate of the right edge.
    #[inline]
    pub fn right(&self) -> i32 {
        self.min.x.max(self.max.x)
    }

    /// Returns the Y coordinate of the top edge.
    #[inline]
    pub fn top(&self) -> i32 {
        self.min.y.max(self.max.y)
    }

    /// Returns the Y coordinate of the bottom edge.
    #[inline]
    pub fn bottom(&self) -> i32 {
        self.min.y.min(self.max.y)
    }

    /// Check if this rectangle intersects another rectangle.
    ///
    /// # Returns
    ///
    /// `true` if the rectangles overlap, `false` otherwise.
    #[inline]
    pub const fn intersects(&self, other: Self) -> bool {
        // (self.min.cmple(other.max) & self.max.cmpge(other.min)).all()
        self.min.x <= other.max.x
            && self.max.x >= other.min.x
            && self.min.y <= other.max.y
            && self.max.y >= other.min.y
    }

    /// Returns an iterator over all points in the rectangle.
    pub fn points(self) -> RectIter {
        RectIter::new(self.min, self.max)
    }

    /// Calls a function for each x/y point in the rectangle
    pub fn for_each<F>(&self, f: F)
    where
        F: FnMut(IVec2),
    {
        RectIter::new(self.min, self.max).for_each(f);
    }
}

impl IntoIterator for Rect {
    type IntoIter = RectIter;
    type Item = IVec2;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        RectIter::new(self.min, self.max)
    }
}
