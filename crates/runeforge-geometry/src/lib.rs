//! 2D geometric primitives and utilities for roguelike development.
//!
//! This crate provides fundamental 2D types like `Point` and `Rect`, along with
//! distance calculations and grid utilities commonly needed in roguelikes.

#![deny(missing_docs)]

use std::ops::{Add, AddAssign, Sub, SubAssign};

/// A 2D point with integer coordinates.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serialization", derive(serde::Serialize, serde::Deserialize))]
pub struct Point {
    /// X coordinate
    pub x: i32,
    /// Y coordinate
    pub y: i32,
}

impl Point {
    /// Creates a new point at the given coordinates.
    ///
    /// # Examples
    ///
    /// ```
    /// use runeforge_geometry::Point;
    ///
    /// let p = Point::new(10, 20);
    /// assert_eq!(p.x, 10);
    /// assert_eq!(p.y, 20);
    /// ```
    #[inline]
    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    /// The origin point (0, 0).
    pub const ZERO: Self = Self::new(0, 0);

    /// Manhattan distance to another point (|dx| + |dy|).
    ///
    /// This is the distance when you can only move in cardinal directions.
    ///
    /// # Examples
    ///
    /// ```
    /// use runeforge_geometry::Point;
    ///
    /// let a = Point::new(0, 0);
    /// let b = Point::new(3, 4);
    /// assert_eq!(a.manhattan_distance(b), 7);
    /// ```
    #[inline]
    pub fn manhattan_distance(self, other: Self) -> u32 {
        ((self.x - other.x).abs() + (self.y - other.y).abs()) as u32
    }

    /// Euclidean distance to another point (straight-line distance).
    ///
    /// # Examples
    ///
    /// ```
    /// use runeforge_geometry::Point;
    ///
    /// let a = Point::new(0, 0);
    /// let b = Point::new(3, 4);
    /// assert_eq!(a.euclidean_distance(b), 5.0);
    /// ```
    #[inline]
    pub fn euclidean_distance(self, other: Self) -> f32 {
        let dx = (self.x - other.x) as f32;
        let dy = (self.y - other.y) as f32;
        (dx * dx + dy * dy).sqrt()
    }

    /// Chebyshev distance to another point (max(|dx|, |dy|)).
    ///
    /// This is the distance when diagonal movement costs the same as cardinal movement.
    ///
    /// # Examples
    ///
    /// ```
    /// use runeforge_geometry::Point;
    ///
    /// let a = Point::new(0, 0);
    /// let b = Point::new(3, 4);
    /// assert_eq!(a.chebyshev_distance(b), 4);
    /// ```
    #[inline]
    pub fn chebyshev_distance(self, other: Self) -> u32 {
        ((self.x - other.x).abs().max((self.y - other.y).abs())) as u32
    }

    /// Returns the four cardinal neighbors (N, S, E, W).
    ///
    /// # Examples
    ///
    /// ```
    /// use runeforge_geometry::Point;
    ///
    /// let p = Point::new(5, 5);
    /// let neighbors = p.cardinal_neighbors();
    /// assert_eq!(neighbors.len(), 4);
    /// ```
    pub fn cardinal_neighbors(self) -> [Point; 4] {
        [
            Point::new(self.x, self.y - 1), // North
            Point::new(self.x, self.y + 1), // South
            Point::new(self.x + 1, self.y), // East
            Point::new(self.x - 1, self.y), // West
        ]
    }

    /// Returns all eight neighbors (cardinal + diagonal).
    ///
    /// # Examples
    ///
    /// ```
    /// use runeforge_geometry::Point;
    ///
    /// let p = Point::new(5, 5);
    /// let neighbors = p.all_neighbors();
    /// assert_eq!(neighbors.len(), 8);
    /// ```
    pub fn all_neighbors(self) -> [Point; 8] {
        [
            Point::new(self.x - 1, self.y - 1), // NW
            Point::new(self.x, self.y - 1),     // N
            Point::new(self.x + 1, self.y - 1), // NE
            Point::new(self.x - 1, self.y),     // W
            Point::new(self.x + 1, self.y),     // E
            Point::new(self.x - 1, self.y + 1), // SW
            Point::new(self.x, self.y + 1),     // S
            Point::new(self.x + 1, self.y + 1), // SE
        ]
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self::new(self.x + other.x, self.y + other.y)
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self::new(self.x - other.x, self.y - other.y)
    }
}

impl SubAssign for Point {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
    }
}

impl From<(i32, i32)> for Point {
    fn from((x, y): (i32, i32)) -> Self {
        Self::new(x, y)
    }
}

impl From<Point> for (i32, i32) {
    fn from(p: Point) -> Self {
        (p.x, p.y)
    }
}

/// A 2D rectangle with integer coordinates.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serialization", derive(serde::Serialize, serde::Deserialize))]
pub struct Rect {
    /// X coordinate of the top-left corner
    pub x: i32,
    /// Y coordinate of the top-left corner
    pub y: i32,
    /// Width of the rectangle
    pub width: u32,
    /// Height of the rectangle
    pub height: u32,
}

impl Rect {
    /// Creates a new rectangle.
    ///
    /// # Examples
    ///
    /// ```
    /// use runeforge_geometry::Rect;
    ///
    /// let r = Rect::new(10, 20, 30, 40);
    /// assert_eq!(r.x, 10);
    /// assert_eq!(r.width, 30);
    /// ```
    #[inline]
    pub const fn new(x: i32, y: i32, width: u32, height: u32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    /// Creates a rectangle from two corner points.
    ///
    /// # Examples
    ///
    /// ```
    /// use runeforge_geometry::{Rect, Point};
    ///
    /// let r = Rect::from_corners(Point::new(10, 20), Point::new(40, 60));
    /// assert_eq!(r.width, 30);
    /// assert_eq!(r.height, 40);
    /// ```
    pub fn from_corners(p1: Point, p2: Point) -> Self {
        let x = p1.x.min(p2.x);
        let y = p1.y.min(p2.y);
        let width = (p1.x - p2.x).unsigned_abs();
        let height = (p1.y - p2.y).unsigned_abs();
        Self::new(x, y, width, height)
    }

    /// Returns the center point of the rectangle.
    ///
    /// # Examples
    ///
    /// ```
    /// use runeforge_geometry::Rect;
    ///
    /// let r = Rect::new(0, 0, 10, 20);
    /// let center = r.center();
    /// assert_eq!(center.x, 5);
    /// assert_eq!(center.y, 10);
    /// ```
    #[inline]
    pub fn center(self) -> Point {
        Point::new(
            self.x + (self.width / 2) as i32,
            self.y + (self.height / 2) as i32,
        )
    }

    /// Returns the top-left corner.
    #[inline]
    pub fn top_left(self) -> Point {
        Point::new(self.x, self.y)
    }

    /// Returns the bottom-right corner (exclusive).
    #[inline]
    pub fn bottom_right(self) -> Point {
        Point::new(self.x + self.width as i32, self.y + self.height as i32)
    }

    /// Checks if a point is inside the rectangle.
    ///
    /// # Examples
    ///
    /// ```
    /// use runeforge_geometry::{Rect, Point};
    ///
    /// let r = Rect::new(0, 0, 10, 10);
    /// assert!(r.contains(Point::new(5, 5)));
    /// assert!(!r.contains(Point::new(15, 5)));
    /// ```
    #[inline]
    pub fn contains(self, point: Point) -> bool {
        point.x >= self.x
            && point.x < self.x + self.width as i32
            && point.y >= self.y
            && point.y < self.y + self.height as i32
    }

    /// Checks if this rectangle intersects with another.
    ///
    /// # Examples
    ///
    /// ```
    /// use runeforge_geometry::Rect;
    ///
    /// let r1 = Rect::new(0, 0, 10, 10);
    /// let r2 = Rect::new(5, 5, 10, 10);
    /// assert!(r1.intersects(r2));
    /// ```
    pub fn intersects(self, other: Rect) -> bool {
        self.x < other.x + other.width as i32
            && self.x + self.width as i32 > other.x
            && self.y < other.y + other.height as i32
            && self.y + self.height as i32 > other.y
    }

    /// Returns an iterator over all points in the rectangle.
    ///
    /// # Examples
    ///
    /// ```
    /// use runeforge_geometry::Rect;
    ///
    /// let r = Rect::new(0, 0, 3, 3);
    /// let points: Vec<_> = r.points().collect();
    /// assert_eq!(points.len(), 9);
    /// ```
    pub fn points(self) -> RectIterator {
        RectIterator {
            rect: self,
            current: Point::new(self.x, self.y),
        }
    }

    /// Returns the area of the rectangle.
    #[inline]
    pub fn area(self) -> u32 {
        self.width * self.height
    }
}

/// Iterator over all points in a rectangle.
pub struct RectIterator {
    rect: Rect,
    current: Point,
}

impl Iterator for RectIterator {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current.y >= self.rect.y + self.rect.height as i32 {
            return None;
        }

        let result = self.current;

        self.current.x += 1;
        if self.current.x >= self.rect.x + self.rect.width as i32 {
            self.current.x = self.rect.x;
            self.current.y += 1;
        }

        Some(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_creation() {
        let p = Point::new(10, 20);
        assert_eq!(p.x, 10);
        assert_eq!(p.y, 20);
    }

    #[test]
    fn test_manhattan_distance() {
        let a = Point::new(0, 0);
        let b = Point::new(3, 4);
        assert_eq!(a.manhattan_distance(b), 7);
    }

    #[test]
    fn test_euclidean_distance() {
        let a = Point::new(0, 0);
        let b = Point::new(3, 4);
        assert_eq!(a.euclidean_distance(b), 5.0);
    }

    #[test]
    fn test_chebyshev_distance() {
        let a = Point::new(0, 0);
        let b = Point::new(3, 4);
        assert_eq!(a.chebyshev_distance(b), 4);
    }

    #[test]
    fn test_point_arithmetic() {
        let a = Point::new(10, 20);
        let b = Point::new(5, 3);
        assert_eq!(a + b, Point::new(15, 23));
        assert_eq!(a - b, Point::new(5, 17));
    }

    #[test]
    fn test_rect_creation() {
        let r = Rect::new(10, 20, 30, 40);
        assert_eq!(r.x, 10);
        assert_eq!(r.y, 20);
        assert_eq!(r.width, 30);
        assert_eq!(r.height, 40);
    }

    #[test]
    fn test_rect_center() {
        let r = Rect::new(0, 0, 10, 20);
        let center = r.center();
        assert_eq!(center, Point::new(5, 10));
    }

    #[test]
    fn test_rect_contains() {
        let r = Rect::new(0, 0, 10, 10);
        assert!(r.contains(Point::new(5, 5)));
        assert!(r.contains(Point::new(0, 0)));
        assert!(!r.contains(Point::new(10, 10)));
        assert!(!r.contains(Point::new(-1, 5)));
    }

    #[test]
    fn test_rect_intersects() {
        let r1 = Rect::new(0, 0, 10, 10);
        let r2 = Rect::new(5, 5, 10, 10);
        let r3 = Rect::new(20, 20, 10, 10);

        assert!(r1.intersects(r2));
        assert!(!r1.intersects(r3));
    }

    #[test]
    fn test_rect_iterator() {
        let r = Rect::new(0, 0, 2, 2);
        let points: Vec<_> = r.points().collect();
        assert_eq!(points.len(), 4);
        assert!(points.contains(&Point::new(0, 0)));
        assert!(points.contains(&Point::new(1, 1)));
    }
}
