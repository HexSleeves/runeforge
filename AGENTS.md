# AGENTS.md - Runeforge Codebase Guide

Guidelines for AI agents working in this Rust roguelike library codebase.

## Project Overview

Runeforge is a pure Rust roguelike development library inspired by libtcod. It's organized as a Cargo workspace with 14 modular crates in `crates/`, examples in `examples/`, and a main facade crate at the root.

**Rust Edition:** 2021
**MSRV:** 1.85
**Toolchain:** Nightly (required for some features)
**License:** BSD-3-Clause

## Build Commands

```bash
# Full workspace build
cargo build --workspace --all-features

# Release build (uses LTO, single codegen unit, stripped)
cargo build --workspace --release --all-features

# Build specific crate
cargo build -p runeforge-pathfinding
```

## Test Commands

```bash
# Run all tests
cargo test --workspace --all-features

# Run single test by name
cargo test --workspace test_name

# Run tests in specific crate
cargo test -p runeforge-geometry

# Run single test in specific crate
cargo test -p runeforge-fov test_basic_fov

# Run doc tests only
cargo test --workspace --doc

# Run with output
cargo test --workspace -- --nocapture
```

## Lint Commands

```bash
# Format check (CI uses nightly rustfmt)
cargo fmt --all --check

# Format fix
cargo fmt --all

# Clippy (must pass with zero warnings)
cargo clippy --workspace --all-targets --all-features -- -D warnings

# Clippy fix
cargo clippy --workspace --all-targets --all-features --fix --allow-dirty
```

## Benchmarks

```bash
# Run all benchmarks (uses Criterion)
cargo bench --workspace

# Run specific benchmark
cargo bench -p runeforge-pathfinding
cargo bench -p runeforge-fov
```

## Documentation

```bash
# Build docs
cargo doc --workspace --all-features --no-deps

# Build and open
cargo doc --workspace --all-features --no-deps --open
```

## Examples

```bash
cargo run --example hello_terminal
cargo run --example roguelike_demo
cargo run --example fov_demo
cargo run --example pathfinding_demo
cargo run --example bsp_demo
cargo run --example map_generation_demo
```

## Code Style Guidelines

### Clippy Configuration

All crates MUST use these lint settings in their `lib.rs`:

```rust
#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)]
#![warn(dbg_macro, todo, unimplemented)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::multiple_crate_versions)]
#![deny(missing_docs)]
```

### Documentation Requirements

- **All public items MUST have doc comments** (`#![deny(missing_docs)]`)
- Use `///` for item docs, `//!` for module docs
- Include usage examples in doc comments for public APIs
- Example format:

```rust
/// Creates a new rectangle from position and size.
///
/// # Examples
///
/// ```
/// use runeforge_geometry::Rect;
/// let rect = Rect::new(0, 0, 10, 10);
/// assert_eq!(rect.width(), 10);
/// ```
pub fn new(x: i32, y: i32, width: i32, height: i32) -> Self { ... }
```

### Naming Conventions

| Item | Convention | Example |
| ---- | ---------- | ------- |
| Functions/Methods | `snake_case` | `get_visible_cells()` |
| Types/Traits | `CamelCase` | `PathProvider`, `FovAlgorithm` |
| Constants | `SCREAMING_SNAKE_CASE` | `Color::RED`, `DEFAULT_SIZE` |
| Modules | `snake_case` | `shadowcast`, `cardinal_direction` |
| Type parameters | Single uppercase | `T`, `P: PathProvider` |

### Module Structure

Each crate follows this pattern:

```bash
crates/runeforge-example/
├── src/
│   ├── lib.rs          # Crate root with lints, docs, re-exports
│   ├── module.rs       # Implementation modules
│   └── subdir/
│       └── mod.rs      # Subdirectory modules
├── benches/            # Criterion benchmarks (if applicable)
├── Cargo.toml
└── README.md
```

### Prelude Pattern

Each crate SHOULD expose a prelude for convenient imports:

```rust
// In lib.rs
pub mod prelude {
    pub use crate::{MainType, ImportantTrait, helper_function};
}
```

### Imports Organization

Order imports as follows (rustfmt handles this):

1. `std` library
2. External crates
3. Workspace crates (`runeforge_*`)
4. Current crate modules (`crate::`, `super::`)

### Error Handling

- Use `thiserror` for custom error types
- Return `Result<T, Error>` for fallible operations
- Avoid `unwrap()` and `expect()` in library code
- Use `?` operator for error propagation

```rust
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TilesetError {
    #[error("Failed to load image: {0}")]
    ImageLoad(#[from] image::ImageError),
    #[error("Invalid tile dimensions: {width}x{height}")]
    InvalidDimensions { width: u32, height: u32 },
}
```

### Performance Annotations

- Use `#[inline]` on small, frequently-called functions
- Use `#[must_use]` on functions returning values that shouldn't be ignored
- Prefer `const fn` where possible

```rust
#[inline]
#[must_use]
pub const fn width(&self) -> i32 {
    self.max.x - self.min.x
}
```

### Testing

- Place unit tests in `#[cfg(test)] mod tests` at the bottom of each file
- Use descriptive test names: `test_rect_contains_point`
- Test edge cases and boundary conditions

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rect_contains_point() {
        let rect = Rect::new(0, 0, 10, 10);
        assert!(rect.contains(IVec2::new(5, 5)));
        assert!(!rect.contains(IVec2::new(15, 15)));
    }
}
```

### Trait Design

- Provide trait for abstraction points (e.g., `PathProvider`, `FovProvider`)
- Keep traits minimal and focused
- Provide default implementations where sensible

## Dependencies Policy

- **Allowed licenses:** MIT, Apache-2.0, BSD-2-Clause, BSD-3-Clause, ISC, Zlib, Unicode-3.0, CC0-1.0
- Run `cargo deny check` before adding new dependencies
- Prefer workspace dependencies defined in root `Cargo.toml`

## CI Requirements

All PRs must pass:

1. `cargo fmt --all --check`
2. `cargo clippy --workspace --all-targets --all-features -- -D warnings`
3. `cargo test --workspace --all-features`
4. `cargo build --workspace --release --all-features`
5. Cross-platform builds (Linux, macOS, Windows)

## Feature Flags

```toml
[features]
default = ["terminal", "fov", "pathfinding"]
full = ["algorithms", "color", "direction", "distance", "fov", "geometry",
        "input", "noise", "pathfinding", "random", "terminal", "tileset", "utils"]
```

Enable features explicitly when testing specific functionality.

## Common Patterns

### Type Conversions

Implement `From` traits for ergonomic conversions:

```rust
impl From<(i32, i32)> for IVec2 {
    fn from((x, y): (i32, i32)) -> Self {
        Self::new(x, y)
    }
}
```

### Builder Pattern

Use builders for complex object construction (see `Rect::from_corners`).

### Iterator Implementation

Provide iterators for collections of points/cells (see `RectIter`, `LineIter`).
