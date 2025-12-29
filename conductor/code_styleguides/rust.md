# Rust Code Style Guide

## Overview

This guide defines the coding standards for Runeforge. All Rust code must follow these conventions to ensure consistency, readability, and maintainability.

---

## Formatting

### Use `rustfmt`

All code must be formatted with `rustfmt` using the default configuration.

```bash
cargo fmt --all
```

### Line Length

- Maximum line length: **100 characters** (rustfmt default)
- Break long lines at logical boundaries

### Indentation

- Use **4 spaces** for indentation (never tabs)
- rustfmt handles this automatically

---

## Naming Conventions

Follow the [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/naming.html):

| Item | Convention | Example |
|------|------------|---------|
| Crates | `snake_case` | `runeforge_fov` |
| Modules | `snake_case` | `mod shadowcast;` |
| Types | `PascalCase` | `struct FovMap` |
| Traits | `PascalCase` | `trait Console` |
| Enums | `PascalCase` | `enum RenderBackend` |
| Enum variants | `PascalCase` | `RenderBackend::Pixels` |
| Functions | `snake_case` | `fn compute_fov()` |
| Methods | `snake_case` | `fn put_char()` |
| Local variables | `snake_case` | `let fov_map = ...;` |
| Constants | `SCREAMING_SNAKE_CASE` | `const MAX_RADIUS: i32 = 100;` |
| Static variables | `SCREAMING_SNAKE_CASE` | `static INSTANCE: ...;` |
| Type parameters | `PascalCase` (single letter for simple) | `T`, `E`, `ConsoleBackend` |
| Lifetimes | `'lowercase` (short, descriptive) | `'a`, `'static`, `'src` |

### Specific Naming Guidelines

- **Conversions:** Use `as_`, `to_`, `into_` prefixes
  - `as_ref()` — Cheap reference-to-reference conversion
  - `to_string()` — Expensive conversion
  - `into_iter()` — Consuming conversion
- **Getters:** No `get_` prefix (e.g., `width()` not `get_width()`)
- **Setters:** Use `set_` prefix (e.g., `set_width()`)
- **Predicates:** Use `is_`, `has_`, `can_` prefixes (e.g., `is_visible()`)

---

## Code Organization

### Module Structure

```rust
// Imports (grouped and sorted)
use std::collections::HashMap;
use std::fmt;

use thiserror::Error;

use crate::geometry::Point;

// Type definitions
pub struct FovMap { /* ... */ }

// Trait implementations
impl FovMap { /* ... */ }

// Trait implementations for external traits
impl fmt::Display for FovMap { /* ... */ }

// Tests
#[cfg(test)]
mod tests { /* ... */ }
```

### Import Grouping

Group imports in this order (separated by blank lines):

1. Standard library (`std`, `core`, `alloc`)
2. External crates
3. Internal crates (`crate::`, workspace crates)
4. `super` and `self` imports

```rust
use std::collections::HashMap;
use std::fmt;

use rand::Rng;
use thiserror::Error;

use crate::geometry::Point;
use crate::color::Color;

use super::shadowcast;
```

### File Organization

- One primary public type per file
- Related helper types can be in the same file
- Large modules should be split into submodules

---

## Documentation

### Public Items

Every public item must have documentation:

```rust
/// Computes the field of view from an origin point.
///
/// Uses recursive shadowcasting to determine visible tiles
/// within the specified radius.
///
/// # Arguments
///
/// * `origin` - The center point for FOV calculation
/// * `radius` - Maximum visibility distance
/// * `transparent` - Callback returning true if a tile is transparent
///
/// # Returns
///
/// A `FovMap` containing visibility data.
///
/// # Examples
///
/// ```
/// use runeforge_fov::compute_fov;
/// use runeforge_geometry::Point;
///
/// let fov = compute_fov(Point::new(5, 5), 10, |p| true);
/// assert!(fov.is_visible(Point::new(5, 5)));
/// ```
pub fn compute_fov<F>(origin: Point, radius: i32, transparent: F) -> FovMap
where
    F: Fn(Point) -> bool,
{
    // Implementation
}
```

### Documentation Sections

Use these standard sections when applicable:

- `# Arguments` — Parameter descriptions
- `# Returns` — Return value description
- `# Errors` — Error conditions (for `Result` types)
- `# Panics` — Panic conditions
- `# Safety` — Safety invariants (for `unsafe` code)
- `# Examples` — Usage examples

### Comments

- Use `//` for line comments
- Use `///` for doc comments
- Use `//!` for module-level documentation
- Explain **why**, not **what** (code should be self-explanatory)

```rust
// Good: Explains why
// Use symmetric shadowcasting for better visual consistency
let fov = compute_symmetric_fov(origin, radius);

// Bad: Explains what (obvious from code)
// Call compute_symmetric_fov function
let fov = compute_symmetric_fov(origin, radius);
```

---

## Error Handling

### Use `Result` for Fallible Operations

```rust
use thiserror::Error;

#[derive(Debug, Error)]
pub enum FovError {
    #[error("radius {0} exceeds maximum allowed radius {1}")]
    RadiusTooLarge(i32, i32),
    
    #[error("origin point ({x}, {y}) is out of bounds")]
    OriginOutOfBounds { x: i32, y: i32 },
}

pub fn compute_fov(origin: Point, radius: i32) -> Result<FovMap, FovError> {
    if radius > MAX_RADIUS {
        return Err(FovError::RadiusTooLarge(radius, MAX_RADIUS));
    }
    // ...
}
```

### Panic Guidelines

Only panic for programmer errors:

```rust
impl FovMap {
    /// Returns whether a point is visible.
    ///
    /// # Panics
    ///
    /// Panics if the point is outside the map bounds.
    pub fn is_visible(&self, point: Point) -> bool {
        assert!(self.contains(point), "point {:?} is out of bounds", point);
        self.data[point.y as usize][point.x as usize]
    }
}
```

### Use `?` Operator

Prefer `?` over explicit `match` for error propagation:

```rust
// Good
pub fn load_font(path: &Path) -> Result<Font, FontError> {
    let data = std::fs::read(path)?;
    Font::parse(&data)
}

// Bad
pub fn load_font(path: &Path) -> Result<Font, FontError> {
    let data = match std::fs::read(path) {
        Ok(d) => d,
        Err(e) => return Err(e.into()),
    };
    Font::parse(&data)
}
```

---

## Types and Traits

### Prefer Owned Types in Public APIs

```rust
// Good: Clear ownership
pub fn set_title(&mut self, title: String) { /* ... */ }

// Also good: Flexible with Into
pub fn set_title(&mut self, title: impl Into<String>) { /* ... */ }

// Avoid: Ambiguous lifetime
pub fn set_title(&mut self, title: &str) { /* ... */ }
```

### Use `impl Trait` for Return Types

```rust
// Good: Hides implementation details
pub fn visible_points(&self) -> impl Iterator<Item = Point> + '_ {
    self.data.iter().filter(|p| p.visible).map(|p| p.position)
}
```

### Derive Common Traits

Derive standard traits when possible:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}
```

### Trait Bounds

Place trait bounds on `impl` blocks, not struct definitions:

```rust
// Good
pub struct Grid<T> {
    data: Vec<T>,
}

impl<T: Clone> Grid<T> {
    pub fn fill(&mut self, value: T) { /* ... */ }
}

// Bad
pub struct Grid<T: Clone> {
    data: Vec<T>,
}
```

---

## Performance

### Avoid Unnecessary Allocations

```rust
// Good: Reuse buffer
pub fn compute_fov_into(&self, origin: Point, radius: i32, output: &mut FovMap) {
    output.clear();
    // ...
}

// Also provide convenience method
pub fn compute_fov(&self, origin: Point, radius: i32) -> FovMap {
    let mut output = FovMap::new(self.width, self.height);
    self.compute_fov_into(origin, radius, &mut output);
    output
}
```

### Use Iterators

Prefer iterators over manual loops:

```rust
// Good
let visible_count = fov.iter().filter(|p| p.visible).count();

// Bad
let mut visible_count = 0;
for p in &fov {
    if p.visible {
        visible_count += 1;
    }
}
```

### Inline Hot Paths

Use `#[inline]` for small, frequently called functions:

```rust
#[inline]
pub fn distance_squared(a: Point, b: Point) -> i32 {
    let dx = a.x - b.x;
    let dy = a.y - b.y;
    dx * dx + dy * dy
}
```

---

## Testing

### Unit Tests

Place unit tests in a `tests` module at the end of each file:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fov_origin_always_visible() {
        let origin = Point::new(5, 5);
        let fov = compute_fov(origin, 10, |_| true);
        assert!(fov.is_visible(origin));
    }

    #[test]
    fn test_fov_blocked_by_wall() {
        let origin = Point::new(5, 5);
        let wall = Point::new(6, 5);
        let behind_wall = Point::new(7, 5);
        
        let fov = compute_fov(origin, 10, |p| p != wall);
        assert!(!fov.is_visible(behind_wall));
    }
}
```

### Integration Tests

Place integration tests in `tests/` directory:

```
crates/runeforge-fov/
├── src/
│   └── lib.rs
└── tests/
    └── integration_test.rs
```

### Benchmarks

Use `criterion` for benchmarks:

```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_fov(c: &mut Criterion) {
    c.bench_function("fov_radius_10", |b| {
        b.iter(|| {
            compute_fov(black_box(Point::new(5, 5)), black_box(10), |_| true)
        });
    });
}

criterion_group!(benches, bench_fov);
criterion_main!(benches);
```

---

## Linting

### Use Clippy

Run Clippy on all code:

```bash
cargo clippy --all-targets --all-features -- -D warnings
```

### Allowed Lints

Suppress lints only when necessary with justification:

```rust
// Allow: This is a public API that matches libtcod conventions
#[allow(clippy::too_many_arguments)]
pub fn console_print_ex(/* ... */) { /* ... */ }
```

---

## Summary

| Aspect | Standard |
|--------|----------|
| **Formatting** | `rustfmt` with defaults |
| **Naming** | Follow Rust API Guidelines |
| **Documentation** | All public items with examples for complex APIs |
| **Error Handling** | `Result` with `thiserror`, panic only for programmer errors |
| **Performance** | Avoid allocations, use iterators, inline hot paths |
| **Testing** | Unit tests in `mod tests`, integration tests in `tests/` |
| **Linting** | `clippy` with `-D warnings` |
