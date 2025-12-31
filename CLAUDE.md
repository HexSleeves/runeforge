# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

**Runeforge** is a modern, modular, pure Rust roguelike library inspired by libtcod. It provides comprehensive tools for roguelike development including rendering (terminal, software, GPU), field-of-view, pathfinding, input handling, and procedural map generation.

**Key Design Principles:**

- **Modular Architecture**: Functionality split into focused crates
- **Pure Rust**: No C dependencies, easier builds and maintenance
- **libtcod API Compatibility**: Easy migration path for existing projects
- **Multiple Rendering Backends**: GPU (wgpu/pixels), Software (CPU), Terminal (ANSI)
- **Cross-Platform**: Windows, macOS, Linux, WebAssembly support

## Development Commands

### Building

```bash
# Build entire workspace
cargo build --workspace --all-features

# Build specific crate
cargo build -p runeforge-fov

# Build with optimizations (dev profile has opt-level=1)
cargo build --release

# Build for WebAssembly
cargo build --target wasm32-unknown-unknown
```

### Testing

```bash
# Run all tests
cargo test --workspace --all-features

# Run tests for specific crate
cargo test -p runeforge-pathfinding

# Run specific test
cargo test -p runeforge-fov shadowcast

# Run tests with output
cargo test -- --nocapture
```

### Linting & Formatting

```bash
# Run clippy (strict lints enforced)
cargo clippy --workspace --all-targets --all-features -- -D warnings

# Format all code
cargo fmt --all

# Check formatting without modifying
cargo fmt --all --check
```

### Documentation

```bash
# Build documentation for entire workspace
cargo doc --workspace --all-features --no-deps

# Build and open documentation
cargo doc --workspace --all-features --no-deps --open

# Document private items (for development)
cargo doc --workspace --all-features --document-private-items
```

### Benchmarks

```bash
# Run all benchmarks
cargo bench --workspace

# Run specific benchmark
cargo bench -p runeforge-fov
cargo bench -p runeforge-pathfinding

# Benchmark with criterion features
cargo bench --workspace -- --save-baseline main
```

### Running Examples

```bash
# Core examples
cargo run --example hello_terminal
cargo run --example roguelike_demo
cargo run --example fov_demo
cargo run --example pathfinding_demo
cargo run --example bsp_demo
cargo run --example map_generation_demo

# Windowed examples (GPU rendering)
cargo run --example windowed_roguelike
cargo run --example windowed_tileset_roguelike

# Demo game (comprehensive integration example)
cargo run -p demo-game
```

## Workspace Architecture

### Crate Organization

The project uses a **modular monorepo** pattern with 13+ specialized crates:

#### Core Crates (Always Available)

- **`runeforge-color`**: RGB/HSV color manipulation, blending, named constants
- **`runeforge-geometry`**: 2D primitives (IVec2, Rect, shapes), grid utilities
- **`runeforge-random`**: RNG with dice notation (`3d6+2`), weighted selection

#### Utility Crates

- **`runeforge-direction`**: Cardinal/ordinal/vertical directions, iteration

#### Algorithm Crates

- **`runeforge-fov`**: Field-of-view algorithms (Symmetric Shadowcasting, Adams FOV)
- **`runeforge-pathfinding`**: A*, Dijkstra, BFS, DFS, iterative deepening variants
- **`runeforge-algorithms`**: Map generation (BSP, Cellular Automata, Drunkard's Walk, Caves)
- **`runeforge-noise`**: Procedural noise generation (Perlin, Simplex)

#### System Crates

- **`runeforge-terminal`**: Console abstraction, multiple rendering backends
- **`runeforge-tileset`**: TrueType font and bitmap tileset loading
- **`runeforge-input`**: Keyboard/mouse input with action mapping

#### Facade Crate

- **`runeforge-rl`** (root): Re-exports all crates with feature flags

### Key Architectural Patterns

#### 1. Facade Pattern with Feature Flags

The root `src/lib.rs` re-exports all sub-crates:

```rust
// Core types always available
pub use runeforge_color as color;
pub use runeforge_geometry as geometry;
pub use runeforge_random as random;

// Optional feature-gated crates
#[cfg(feature = "fov")]
pub use runeforge_fov as fov;

#[cfg(feature = "terminal")]
pub use runeforge_terminal as terminal;
```

Users can opt-in to only what they need:

```toml
runeforge-rl = { version = "0.1", features = ["fov", "pathfinding"] }
# or use "full" for everything
runeforge-rl = { version = "0.1", features = ["full"] }
```

#### 2. Trait-Based Backend Abstraction

The rendering system uses traits to support multiple backends:

- **Terminal Backend**: ANSI escape codes for console rendering
- **Pixel Backend**: GPU-accelerated rendering via `pixels` crate
- **Software Backend**: CPU-based framebuffer rendering

All backends implement common traits in `runeforge-terminal`.

#### 3. Prelude Modules

Each crate exposes a `prelude` module for convenient imports:

```rust
use runeforge_rl::prelude::*;

// Now you have access to common types:
// Color, IVec2, Rect, Rng, etc.
```

#### 4. Zero-Cost Abstractions

The library uses Rust's type system and generics to provide abstractions with no runtime overhead:

- Generic pathfinding over any grid type implementing `PathProvider`
- Generic FOV over any transparency map
- Inline functions and compile-time optimizations

### Dependency Graph

```bash
runeforge-rl (facade)
├── runeforge-color (no deps)
├── runeforge-geometry
│   └── runeforge-direction
├── runeforge-random (rand)
├── runeforge-fov
│   ├── runeforge-geometry
│   └── runeforge-direction
├── runeforge-pathfinding
│   ├── runeforge-geometry
│   ├── runeforge-direction
├── runeforge-algorithms
│   ├── runeforge-geometry
│   └── runeforge-random
├── runeforge-terminal
│   ├── runeforge-color
│   ├── runeforge-geometry
│   └── runeforge-tileset
├── runeforge-tileset (ab_glyph, image)
├── runeforge-input (winit)
└── runeforge-noise (noise crate)
```

## Code Conventions

### Clippy Configuration

The project enforces **strict** clippy lints:

```rust
#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)]
#![warn(clippy::dbg_macro, clippy::todo, clippy::unimplemented)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::multiple_crate_versions)] // wgpu/pixels transitive deps
```

**Important**: When adding code, ensure it passes `cargo clippy -- -D warnings`.

### Documentation Requirements

All public APIs require documentation with examples:

```rust
/// Calculates field-of-view using symmetric shadowcasting.
///
/// # Arguments
///
/// * `origin` - The center point of the FOV calculation
/// * `radius` - Maximum visibility radius
/// * `transparency` - Map defining which tiles block vision
///
/// # Examples
///
/// ```
/// use runeforge_fov::prelude::*;
/// let origin = IVec2::new(10, 10);
/// let visible = shadowcast(origin, 8, &map);
/// ```
pub fn shadowcast(/* ... */) { }
```

### Module Organization Pattern

Each crate follows this structure:

```bash
crates/runeforge-foo/
├── src/
│   ├── lib.rs           # Public API, re-exports
│   ├── prelude.rs       # Convenience module
│   ├── algorithm1.rs    # Implementation modules
│   ├── algorithm2.rs
│   └── types.rs         # Common types
├── benches/
│   └── foo_bench.rs     # Criterion benchmarks
└── Cargo.toml
```

## Performance Optimization

### Build Profiles

The workspace defines optimized build profiles:

```toml
[profile.dev]
opt-level = 1              # Faster debug builds
debug = 1                  # Reduced debug info

[profile.dev.package."*"]
opt-level = 3              # Optimize dependencies even in debug

[profile.release]
lto = "thin"               # Link-time optimization
codegen-units = 1          # Better optimization
opt-level = 3
strip = true               # Remove debug symbols
```

### Benchmarking

Benchmarks use `criterion` and are located in `crates/*/benches/`:

- `runeforge-fov/benches/fov_bench.rs`: FOV algorithm performance
- `runeforge-pathfinding/benches/pathfinding_bench.rs`: Pathfinding performance

## Testing Patterns

### Unit Tests

Located in `#[cfg(test)] mod tests` within each module:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shadowcast_basic() {
        let origin = IVec2::new(5, 5);
        let visible = shadowcast(origin, 3, &simple_map());
        assert!(visible.contains(&origin));
    }
}
```

### Integration Examples

Examples in `examples/` serve as integration tests and usage demonstrations. They should compile and run successfully.

## Common Development Tasks

### Adding a New Algorithm

1. Choose appropriate crate (or create new one in `crates/`)
2. Implement algorithm with public API
3. Add to crate's `lib.rs` and `prelude.rs`
4. Write unit tests
5. Add benchmark if performance-critical
6. Create example demonstrating usage
7. Document with rustdoc comments

### Adding a New Crate

1. Create directory in `crates/runeforge-newfeature/`
2. Add to workspace members in root `Cargo.toml`
3. Define workspace dependencies in `[workspace.dependencies]`
4. Add feature flag in root `Cargo.toml` features section
5. Re-export in `src/lib.rs` with `#[cfg(feature = "newfeature")]`
6. Add to prelude if commonly used

### Implementing a New Rendering Backend

New backends should implement the traits defined in `runeforge-terminal`:

- Implement console rendering trait
- Handle font/tileset loading
- Manage window/event loop if applicable
- Add feature flag for optional inclusion

## External Dependencies

### Key External Crates

- **`glam`**: Fast linear algebra (Vec2, IVec2, matrices)
- **`rand`**: Random number generation
- **`winit`**: Cross-platform window management
- **`wgpu`**: Modern graphics API (Vulkan/Metal/DX12/GL backend)
- **`pixels`**: GPU-accelerated pixel buffer
- **`ab_glyph`**: TrueType font rendering
- **`image`**: PNG/image loading
- **`hashbrown`**: Fast hash maps/sets
- **`criterion`**: Benchmarking framework

### Avoiding Dependency Bloat

When adding dependencies:

- Prefer `default-features = false` and enable only needed features
- Use workspace dependencies for version consistency
- Consider feature flags to make dependencies optional
- Avoid duplicating functionality already in the standard library

## Project Status & Roadmap

**Current Phase**: 5 (Input & Integration) - ~70% complete

**Completed:**

- ✅ Core crates (color, geometry, random, direction, distance)
- ✅ FOV algorithms (Symmetric Shadowcasting, Adams)
- ✅ Pathfinding (A*, Dijkstra, BFS, DFS variants)
- ✅ Map generation (BSP, Cellular Automata, Drunkard's Walk, Caves)
- ✅ Rendering system (Terminal, GPU, Software backends)
- ✅ Input handling (keyboard/mouse with action mapping)
- ✅ Complete demo game integration

**In Progress/Planned:**

- 🔨 Advanced noise generation algorithms
- 🔨 Additional map generation patterns
- 🔨 UI framework/widgets
- 🔨 WebAssembly optimization

See `RUNEFORGE.md` for detailed roadmap and design decisions.
