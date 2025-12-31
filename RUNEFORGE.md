# Runeforge: A Modern Rust Roguelike Library

## Project Vision

**Runeforge** is a pure Rust roguelike library inspired by libtcod, combining the familiar API developers love with modern Rust best practices and GPU-accelerated rendering.

### Unique Selling Points

1. **libtcod API Compatibility** - Easy migration for existing projects
2. **Pure Rust** - No C dependencies, easier to build and maintain
3. **Modern Graphics** - GPU-accelerated via wgpu with software fallback
4. **WebAssembly Support** - Run roguelikes in browser
5. **Modular Design** - Use only what you need
6. **Excellent Documentation** - Complete tutorials and examples
7. **Active Development** - Modern, maintained alternative

---

## 📊 Libtcod Analysis

### What is libtcod?

**libtcod** is a free, fast, portable API for roguelike developers providing:

- True color console rendering
- Field-of-view algorithms
- Pathfinding (A*, Dijkstra)
- Procedural generation utilities (BSP, heightmaps, noise)
- Random number generation
- Name generation
- Image handling and tileset support

### Core Modules in libtcod

#### Rendering & Display

- **Console** - Core text/tile rendering system
- **Context** - Rendering context management
- **Tileset** - Font/tileset loading and rendering
- **Renderer SDL2** - SDL2 backend
- **Renderer Xterm** - Terminal backend

#### Algorithms

- **FOV** - Field-of-view with 6+ algorithms (shadowcasting, raycasting, etc.)
- **Pathfinding** - A* and Dijkstra pathfinding
- **Bresenham** - Line drawing algorithm
- **BSP** - Binary space partitioning for dungeon generation

#### Procedural Generation

- **Noise** - Perlin/simplex noise generation
- **Heightmap** - 2D heightmap manipulation
- **Name Generator** - Procedural name generation

#### Utilities

- **Random** - Mersenne Twister RNG
- **Color** - RGB/HSV color manipulation
- **Image** - Image loading/saving
- **Parser** - Configuration file parsing
- **GUI** - Simple GUI widgets
- **Mouse** - Mouse input handling

### libtcod Dependencies

- **SDL2/SDL3** - Window management and rendering
- **lodepng** - PNG image support
- **zlib** - Compression for REXPaint files
- **utf8proc** - Unicode text processing
- **stb_truetype** - TrueType font rendering
- **Catch2** - Testing framework

---

## 🦀 Rust Ecosystem Mapping

### Complete Dependency Mapping

#### Window & Input Management

| libtcod | Rust Equivalent | Notes |
| --------- | ----------------- | ------- |
| SDL2/SDL3 | **[winit](https://github.com/rust-windowing/winit)** | Pure Rust, cross-platform window management |
| Mouse handling | winit events | Built into winit |
| Keyboard handling | winit events | Built into winit |

#### Graphics Rendering

| libtcod | Rust Equivalent | Notes |
| --------- | ----------------- | ------- |
| SDL2 renderer | **[wgpu](https://wgpu.rs/)** + **[pixels](https://lib.rs/crates/pixels)** or **softbuffer** | Modern GPU-accelerated rendering |
| Terminal rendering | Custom implementation | Render to texture/framebuffer |
| OpenGL backend | wgpu (Vulkan/Metal/DX12/GL) | wgpu supports multiple backends |

#### Text & Font Rendering

| libtcod | Rust Equivalent | Notes |
| --------- | ----------------- | ------- |
| stb_truetype | **[ab_glyph](https://lib.rs/crates/ab_glyph)** or **[cosmic-text](https://github.com/pop-os/cosmic-text)** | ab_glyph for simple, cosmic-text for advanced |
| Unicode support (utf8proc) | Built-in Rust `char`/`String` | Rust has native UTF-8 support |

#### Image Processing

| libtcod | Rust Equivalent | Notes |
| --------- | ----------------- | ------- |
| lodepng | **[image](https://crates.io/crates/image)** crate | Supports PNG, JPEG, and many formats |
| stb_image | image crate | More comprehensive than stb |
| REXPaint files | Custom parser or port | Could port libtcod's implementation |

#### Algorithms & Utilities

| libtcod | Rust Equivalent | Notes |
| --------- | ----------------- | ------- |
| Pathfinding (A*, Dijkstra) | **[pathfinding](https://crates.io/crates/pathfinding)** | Complete pathfinding library |
| Noise generation | **[noise-rs](https://github.com/Razaekel/noise-rs)** or **[bracket-noise](https://lib.rs/crates/bracket-noise)** | Perlin, simplex, and more |
| Random (Mersenne Twister) | **[rand](https://crates.io/crates/rand)** | Industry-standard RNG |
| BSP trees | Custom implementation | Relatively simple to implement |
| Bresenham | **[bresenham](https://crates.io/crates/bresenham)** or **[line_drawing](https://crates.io/crates/line_drawing)** | Multiple options available |
| Heightmaps | Custom implementation | Build on existing noise |
| Color manipulation | **[palette](https://crates.io/crates/palette)** | RGB, HSV, and more color spaces |
| Data structures (lists, trees) | Rust std collections | `Vec`, `HashMap`, `BTreeMap`, etc. |

#### Compression & Serialization

| libtcod | Rust Equivalent | Notes |
| --------- | ----------------- | ------- |
| zlib | **[flate2](https://crates.io/crates/flate2)** | Comprehensive compression library |
| ZIP archives | **[zip](https://crates.io/crates/zip)** | Read/write ZIP files |

---

## 🏛️ Runeforge Architecture

### Workspace Structure

Following the **modular monorepo** approach inspired by [bracket-lib](https://github.com/amethyst/bracket-lib):

```bash
runeforge-rl/
├── Cargo.toml                 # Workspace configuration
├── RUNEFORGE.md              # This document
├── README.md                 # User-facing documentation
├── LICENSE                   # BSD-3-Clause (like libtcod)
├── crates/
│   ├── runeforge-algorithms/ # BSP, Caves, Drunkard's Walk
│   ├── runeforge-color/      # Color manipulation
│   ├── runeforge-direction/  # Grid direction handling
│   ├── runeforge-fov/        # Field-of-view algorithms
│   ├── runeforge-geometry/   # 2D geometric primitives
│   ├── runeforge-input/      # Input handling
│   ├── runeforge-noise/      # Noise generation
│   ├── runeforge-pathfinding/# Pathfinding algorithms
│   ├── runeforge-random/     # Random number generation
│   ├── runeforge-terminal/   # Terminal/console rendering
│   ├── runeforge-tileset/    # Tileset/font loading
├── examples/                 # Example roguelikes
│   ├── ...
└── src/                      # Facade crate
```

### Crate Dependency Graph

```text
.
├── runeforge-algorithms
│   ├── runeforge-geometry
│   └── runeforge-random
├── runeforge-color
├── runeforge-direction
├── runeforge-fov
│   └── runeforge-geometry
├── runeforge-geometry
│   └── runeforge-direction
├── runeforge-input
│   └── runeforge-direction
├── runeforge-noise
├── runeforge-pathfinding
│   └── runeforge-geometry
├── runeforge-random
├── runeforge-terminal
│   ├── runeforge-color
│   ├── runeforge-geometry
│   └── runeforge-tileset
├── runeforge-tileset
│   └── runeforge-color
```

### Crate Descriptions

```rust
use runeforge_rl::prelude::*;

fn main() {
    let mut rng = Rng::new();
    let point = IVec2::new(10, 20);
    let color = Color::RED;

    println!("Random d6 roll: {}", rng.roll_dice(1, 6));
    println!("IVec2 at ({}, {})", point.x, point.y);
    println!("Color: {}", color);
}
```

#### runeforge-color

**Purpose:** Color manipulation and conversion.

**Features:**

- RGB and HSV color spaces
- Color blending and interpolation
- Named color constants
- Conversion utilities

**Dependencies:** `palette` (optional wrapper)

#### runeforge-geometry

**Purpose:** 2D/3D geometric primitives and utilities.

**Features:**

- `IVec2`, `Rect`, `Circle` types
- Distance calculations (Manhattan, Euclidean, Chebyshev)
- Grid utilities and iterators
- Bounds checking

#### runeforge-random

**Purpose:** Random number generation with roguelike-specific utilities.

**Features:**

- Wrapper around `rand` crate
- Dice roll notation (`3d6+12`)
- Weighted random selection
- Procedural generation seeds

#### runeforge-noise

**Purpose:** Procedural noise generation.

**Features:**

- Perlin, Simplex, Cellular noise
- Fractional Brownian Motion (FBM)
- Turbulence and ridged noise
- 2D and 3D noise

**Dependencies:** `noise-rs` wrapper

#### runeforge-pathfinding

**Purpose:** Pathfinding algorithms optimized for grids.

**Features:**

- A* pathfinding
- Dijkstra maps
- Flow fields
- Grid-specific optimizations

**Dependencies:** `pathfinding` crate

#### runeforge-fov

**Purpose:** Field-of-view algorithms.

**Features:**

- Recursive shadowcasting (most popular)
- Symmetric shadowcast
- Diamond raycasting
- Permissive FOV
- Restrictive shadowcast
- Circular raycasting

**Implementation:** Pure Rust, no external dependencies

#### runeforge-bsp

**Purpose:** Binary space partitioning for dungeon generation.

**Features:**

- BSP tree structure
- Split strategies (random, balanced, etc.)
- Dungeon room generation
- Corridor generation

#### runeforge-algorithms

**Purpose:** Miscellaneous algorithms and utilities.

**Features:**

- Bresenham line drawing
- Heightmap manipulation
- Flood fill
- Cellular automata

#### runeforge-tileset

**Purpose:** Font and tileset loading/management.

**Features:**

- TrueType font loading (`ab_glyph`)
- BDF bitmap font support
- Custom tileset formats
- Sprite sheet management
- Glyph caching

#### runeforge-terminal

**Purpose:** Core rendering system.

**Features:**

- Console abstraction (character grid)
- Multiple backends:
  - GPU-accelerated (wgpu + pixels)
  - Software rendering (CPU-based)
  - Terminal output (ANSI)
- Tileset rendering
- REXPaint file support
- Screenshot capture

**Dependencies:** `winit`, `wgpu`, `pixels`

#### runeforge-input

**Purpose:** Input handling and event processing.

**Features:**

- Keyboard event handling
- Mouse support
- Key binding system
- Input state management

**Dependencies:** `winit`

---

## 📋 Implementation Roadmap

### Phase 1: Foundation (Weeks 1-4)

**Goal:** Core infrastructure and basic rendering

#### Milestone 1.1: Project Setup

- [x] Create workspace structure
- [ ] Set up CI/CD (GitHub Actions)
  - [ ] Linux, macOS, Windows builds
  - [ ] WebAssembly target
  - [ ] Clippy and rustfmt checks
- [ ] Configure cargo-release
- [ ] Create documentation site structure (mdBook)
- [ ] Add LICENSE (BSD-3-Clause)

#### Milestone 1.2: Core Crates

- [ ] **runeforge-color**
  - [ ] RGB color struct
  - [ ] HSV color struct
  - [ ] Color conversion functions
  - [ ] Blending (lerp, multiply, etc.)
  - [ ] Named color constants
  - [ ] Unit tests

- [ ] **runeforge-geometry**
  - [ ] `IVec2` struct (x, y)
  - [ ] `Rect` struct (x, y, w, h)
  - [ ] `Circle` struct (center, radius)
  - [ ] Distance functions (Manhattan, Euclidean, Chebyshev)
  - [ ] Grid iterators
  - [ ] Bounds checking utilities
  - [ ] Unit tests

- [ ] **runeforge-random**
  - [ ] Wrapper around `rand::thread_rng()`
  - [ ] Dice roll parser (`parse_dice("3d6+2")`)
  - [ ] Weighted random selection
  - [ ] Shuffle utilities
  - [ ] Unit tests

#### Milestone 1.3: Basic Rendering

- [ ] **runeforge-terminal** (software renderer only)
  - [ ] `Console` struct with 2D char grid
  - [ ] `Cell` struct (char, fg_color, bg_color)
  - [ ] Basic drawing functions:
    - [ ] `set_char(x, y, ch, fg, bg)`
    - [ ] `clear()`
    - [ ] `print(x, y, text, fg, bg)`
  - [ ] Window creation (winit)
  - [ ] Software rendering to window
  - [ ] Frame timing
  - [ ] Event loop integration

#### Milestone 1.4: First Example

- [ ] **Hello World Example**
  - [ ] Create 80x50 console
  - [ ] Print "Hello, Runeforge!" in color
  - [ ] Handle window close event
  - [ ] Build and run successfully

**Success Criteria:**

- ✅ Workspace builds without errors
- ✅ "Hello World" example runs and displays text
- ✅ All unit tests pass
- ✅ Documentation builds

---

### Phase 2: Algorithms (Weeks 5-8)

**Goal:** Core roguelike algorithms

#### Milestone 2.1: Field of View

- [ ] **runeforge-fov**
  - [ ] `Transparency` trait for map tiles
  - [ ] Recursive shadowcasting algorithm
  - [ ] Symmetric shadowcast algorithm
  - [ ] Visibility set representation
  - [ ] Performance benchmarks
  - [ ] Unit tests with known patterns

#### Milestone 2.2: Pathfinding

- [ ] **runeforge-pathfinding**
  - [ ] Wrap `pathfinding` crate's A*
  - [ ] Grid-specific A* implementation
  - [ ] Dijkstra map generation
  - [ ] Path smoothing utilities
  - [ ] Performance benchmarks
  - [ ] Unit tests with known mazes

#### Milestone 2.3: Line Drawing

- [ ] **runeforge-algorithms**
  - [ ] Bresenham line algorithm
  - [ ] Line-of-sight using Bresenham
  - [ ] Circle drawing (midpoint algorithm)
  - [ ] Flood fill
  - [ ] Unit tests

#### Milestone 2.4: Examples

- [ ] **FOV Demo**
  - [ ] Simple dungeon map
  - [ ] Player movement with arrow keys
  - [ ] FOV visualization
  - [ ] Compare different FOV algorithms

- [ ] **Pathfinding Demo**
  - [ ] Click-to-move with A*
  - [ ] Visualize path
  - [ ] Dijkstra map for flee behavior
  - [ ] Enemy chase AI

**Success Criteria:**

- ✅ FOV demo shows correct visibility
- ✅ Pathfinding demo finds optimal paths
- ✅ Benchmarks show competitive performance
- ✅ All algorithms tested with edge cases

---

### Phase 3: Procedural Generation (Weeks 9-12)

**Goal:** Map generation tools

#### Milestone 3.1: Noise Generation

- [ ] **runeforge-noise**
  - [ ] Wrap `noise-rs` generators
  - [ ] Roguelike-specific presets
  - [ ] Heightmap utilities
  - [ ] Noise visualization example

#### Milestone 3.2: BSP Trees

- [ ] **runeforge-bsp**
  - [ ] BSP tree data structure
  - [ ] Split strategies (random, balanced, golden ratio)
  - [ ] Traversal iterators
  - [ ] Room placement utilities
  - [ ] Unit tests

#### Milestone 3.3: Map Generation

- [ ] **Dungeon Generation Examples**
  - [ ] BSP dungeon with rooms and corridors
  - [ ] Cave generation (cellular automata)
  - [ ] Drunkard's walk algorithm
  - [ ] Room-based dungeons
  - [ ] Procedural name generation

#### Milestone 3.4: Examples

- [ ] **Map Generator Browser**
  - [ ] Generate multiple map types
  - [ ] Seed-based reproducibility
  - [ ] Export to REXPaint format
  - [ ] Statistics (connectivity, room count, etc.)

**Success Criteria:**

- ✅ Generated maps are playable
- ✅ Consistent results from same seed
- ✅ Multiple generation algorithms available
- ✅ Visual quality comparable to hand-crafted maps

---

### Phase 4: Advanced Rendering (Weeks 13-16)

**Goal:** GPU acceleration and advanced features

#### Milestone 4.1: GPU Renderer

- [ ] **runeforge-terminal** (wgpu backend)
  - [ ] Shader for character rendering
  - [ ] Texture atlas for characters
  - [ ] GPU buffer management
  - [ ] Performance optimization
  - [ ] Fallback to software if GPU unavailable

#### Milestone 4.2: Tileset Support

- [ ] **runeforge-tileset**
  - [ ] TrueType font loading (ab_glyph)
  - [ ] BDF bitmap font parser
  - [ ] Custom tileset format
  - [ ] Sprite sheet support
  - [ ] Glyph atlas generation
  - [ ] Font metrics (width, height, baseline)

#### Milestone 4.3: Advanced Features

- [ ] REXPaint file format support
  - [ ] Load `.xp` files
  - [ ] Save `.xp` files
  - [ ] Layer support
- [ ] Screenshot functionality (image crate)
- [ ] Color animation/cycling
- [ ] Subcell resolution rendering
- [ ] Multiple console layers

#### Milestone 4.4: Examples

- [ ] **Font Showcase**
  - [ ] Display multiple fonts
  - [ ] Switch fonts at runtime
  - [ ] Character set browser

- [ ] **Performance Benchmark**
  - [ ] CPU vs GPU rendering
  - [ ] Frame rate comparison
  - [ ] Memory usage profiling

**Success Criteria:**

- ✅ GPU rendering significantly faster than CPU
- ✅ Multiple font formats supported
- ✅ REXPaint files load/save correctly
- ✅ No visual regressions from software renderer

---

### Phase 5: Input & Polish (Weeks 17-20)

**Goal:** User input and API polish

#### Milestone 5.1: Input Handling

- [ ] **runeforge-input**
  - [ ] Keyboard event wrapper
  - [ ] Mouse position and clicks
  - [ ] Key binding system
  - [ ] Text input handling
  - [ ] Input state queries (is_key_down, etc.)

#### Milestone 5.2: API Refinement

- [ ] **runeforge-core** (facade)
  - [ ] Re-export all public APIs
  - [ ] Feature flags for optional components
  - [ ] Prelude module
  - [ ] Builder patterns for complex objects
  - [ ] Error types with `thiserror`

#### Milestone 5.3: Documentation

- [ ] API documentation (rustdoc)
  - [ ] All public items documented
  - [ ] Code examples for major features
  - [ ] Links to related items
- [ ] Tutorial book (mdBook)
  - [ ] Getting started guide
  - [ ] Core concepts
  - [ ] Cookbook (common patterns)
  - [ ] Migration guide from libtcod

#### Milestone 5.4: Examples & Templates

- [ ] **Complete Roguelike Tutorial**
  - [ ] Follow rogueliketutorials.com structure
  - [ ] Part 1: Drawing the @ and moving around
  - [ ] Part 2: The generic entity, the render functions, and the map
  - [ ] Part 3: Generating a dungeon
  - [ ] Part 4: Field of view
  - [ ] Part 5: Placing enemies and kicking them
  - [ ] ... (continue as needed)

- [ ] **7DRL Starter Kit**
  - [ ] Project template for game jams
  - [ ] Basic UI components
  - [ ] Save/load system
  - [ ] Common roguelike patterns

**Success Criteria:**

- ✅ Input system is ergonomic and type-safe
- ✅ API is consistent and well-documented
- ✅ Tutorial is complete and tested
- ✅ Beginners can build a roguelike from scratch

---

### Phase 6: Release & Ecosystem (Weeks 21-24)

**Goal:** Production-ready 1.0 release

#### Milestone 6.1: Documentation Polish

- [ ] Complete API documentation audit
- [ ] Architecture guide
- [ ] Performance tuning guide
- [ ] Best practices document
- [ ] Migration guide from libtcod
- [ ] Comparison with bracket-lib and doryen-rs

#### Milestone 6.2: Performance

- [ ] Comprehensive benchmark suite
  - [ ] FOV algorithms
  - [ ] Pathfinding
  - [ ] Rendering (CPU vs GPU)
  - [ ] Map generation
- [ ] Optimization passes
  - [ ] Profile with `cargo flamegraph`
  - [ ] Reduce allocations
  - [ ] SIMD where applicable
- [ ] Memory profiling with `valgrind`/`heaptrack`
- [ ] Compare with libtcod and bracket-lib

#### Milestone 6.3: Cross-Platform Testing

- [ ] Linux (x86_64, aarch64)
- [ ] macOS (Intel, Apple Silicon)
- [ ] Windows (x86_64)
- [ ] WebAssembly (browser)
- [ ] Visual regression tests (screenshot comparison)

#### Milestone 6.4: Release Preparation

- [ ] Version 1.0.0 release
  - [ ] CHANGELOG.md
  - [ ] Semantic versioning commitment
  - [ ] Publish to crates.io
- [ ] Project website
  - [ ] Landing page
  - [ ] Documentation links
  - [ ] Gallery of games built with Runeforge
- [ ] Community setup
  - [ ] Discord server
  - [ ] Reddit announcement (r/roguelikedev)
  - [ ] GitHub Discussions
- [ ] Maintenance plan
  - [ ] Issue templates
  - [ ] PR templates
  - [ ] Contributing guide
  - [ ] Code of conduct
  - [ ] Roadmap for 2.0

**Success Criteria:**

- ✅ Version 1.0.0 published to crates.io
- ✅ All platforms tested and working
- ✅ Documentation is comprehensive
- ✅ At least one game built with Runeforge
- ✅ Positive community reception

---

## 🎯 Key Design Decisions

### 1. Modular Architecture

**Decision:** Use a monorepo with separate crates for each concern.

**Rationale:**

- Users can depend on only what they need (smaller binary sizes)
- Easier to maintain and test individual components
- Clear separation of concerns
- Follows bracket-lib's successful pattern

**Trade-offs:**

- More complex workspace management
- Need to coordinate versions across crates
- More crates to publish

### 2. Rendering Strategy

**Decision:** Support multiple rendering backends via traits.

**Backends:**

1. **GPU (wgpu + pixels)** - Primary, best performance
2. **Software (CPU)** - Fallback, minimal dependencies
3. **Terminal (ANSI)** - For SSH/headless environments

**Rationale:**

- Flexibility for different use cases
- WebAssembly support via wgpu
- Graceful degradation on old hardware

**Trade-offs:**

- More code to maintain
- Testing complexity

### 3. API Design Philosophy

**Principles:**

- **Rust-idiomatic:** Use `Result`, `Option`, iterators, ownership
- **Zero-cost abstractions:** Compile-time optimizations, no runtime overhead
- **Backward compatible:** Provide libtcod-like API for easy migration
- **Extensible:** Traits for custom implementations

**Examples:**

```rust
// Rust-idiomatic error handling
pub fn load_font(path: &Path) -> Result<Font, FontError> { ... }

// Iterator-based APIs
for cell in console.cells() {
    println!("{:?}", cell);
}

// Builder pattern for complex objects
let console = ConsoleBuilder::new(80, 50)
    .with_title("My Game")
    .with_font("terminal.ttf")
    .build()?;

// Trait-based extensibility
pub trait Renderer {
    fn render(&mut self, console: &Console);
}
```

### 4. Feature Flags

**Decision:** Use Cargo features for optional functionality.

**Features:**

```toml
[features]
default = ["render-wgpu", "pathfinding", "fov", "noise"]
render-wgpu = ["wgpu", "pixels"]
render-software = []
render-terminal = []
pathfinding = ["runeforge-pathfinding"]
fov = ["runeforge-fov"]
noise = ["runeforge-noise"]
bsp = ["runeforge-bsp"]
algorithms = ["runeforge-algorithms"]
serialization = ["serde"]
```

**Rationale:**

- Users can minimize dependencies
- Easier to add new features without breaking changes
- Standard Rust practice

### 5. Error Handling

**Decision:** Use `thiserror` for library errors, return `Result` for fallible operations.

**Example:**

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RuneforgeError {
    #[error("Font loading failed: {0}")]
    FontError(String),

    #[error("Invalid console dimensions: {width}x{height}")]
    InvalidDimensions { width: u32, height: u32 },

    #[error("Rendering backend not available")]
    NoBackend,
}
```

**Rationale:**

- Clear error messages
- Composable with `?` operator
- Type-safe error handling

### 6. Testing Strategy

**Levels:**

1. **Unit tests** - Per-function logic
2. **Integration tests** - Cross-crate functionality
3. **Visual regression tests** - Screenshot comparison
4. **Benchmarks** - Performance tracking
5. **Examples as tests** - Ensure examples compile and run

**Tools:**

- `cargo test` for standard tests
- `criterion` for benchmarks
- `insta` for snapshot testing
- CI/CD for automated testing

---

## 📚 Essential Dependencies

### Workspace-Level Dependencies

```toml
[workspace]
members = [
    "crates/runeforge-core",
    "crates/runeforge-color",
    "crates/runeforge-geometry",
    "crates/runeforge-noise",
    "crates/runeforge-random",
    "crates/runeforge-pathfinding",
    "crates/runeforge-fov",
    "crates/runeforge-bsp",
    "crates/runeforge-terminal",
    "crates/runeforge-tileset",
    "crates/runeforge-input",
    "crates/runeforge-algorithms",
]

[workspace.dependencies]
# Window & Events
winit = "0.30"

# Rendering
wgpu = "23.0"
pixels = "0.14"
bytemuck = "1.14"

# Text rendering
ab_glyph = "0.2"
cosmic-text = "0.14"

# Images
image = { version = "0.25", default-features = false, features = ["png"] }

# Algorithms
pathfinding = "4.11"
noise = "0.9"
bresenham = "0.1"
line_drawing = "1.0"

# Utilities
rand = "0.8"
palette = "0.7"
flate2 = "1.0"

# Serialization (optional)
serde = { version = "1.0", features = ["derive"] }

# Error handling
thiserror = "2.0"
anyhow = "1.0"

# Logging
log = "0.4"

# Testing
criterion = "0.5"
```

---

## 🔍 Competitive Analysis

### Existing Rust Alternatives

#### 1. tcod-rs

- **URL:** <https://github.com/tomassedovic/tcod-rs>
- **Status:** ❌ Archived, no longer maintained (January 2025)
- **Approach:** FFI bindings to C library
- **Pros:**
  - Feature parity with libtcod
  - Mature and stable
- **Cons:**
  - Requires libtcod installation
  - Unsafe Rust bindings
  - No active development
  - Platform-specific build issues

#### 2. bracket-lib

- **URL:** <https://github.com/amethyst/bracket-lib>
- **Status:** ✅ Active, well-maintained
- **Approach:** Pure Rust reimplementation
- **Pros:**
  - Comprehensive tutorials
  - Modular architecture
  - WASM support
  - Multiple rendering backends
  - Great documentation
- **Cons:**
  - Different API from libtcod (migration barrier)
  - Opinionated design choices
  - Larger dependency tree

#### 3. doryen-rs

- **URL:** <https://github.com/jice-nospam/doryen-rs>
- **Status:** ⚠️ Stable but slow development (last release October 2022)
- **Approach:** Pure Rust with GLSL rendering
- **Pros:**
  - By libtcod's original author
  - WASM support
  - GLSL shaders for effects
- **Cons:**
  - Less active community
  - Slower development pace
  - Less documentation

### Runeforge's Unique Position

**Runeforge aims to combine the best of all worlds:**

| Feature | tcod-rs | bracket-lib | doryen-rs | **Runeforge** |
| --------- | --------- | ------------- | ----------- | --------------- |
| Pure Rust | ❌ | ✅ | ✅ | ✅ |
| libtcod API | ✅ | ❌ | ❌ | ✅ |
| Modern Graphics | ❌ | ✅ | ✅ | ✅ |
| Active Development | ❌ | ✅ | ⚠️ | ✅ (planned) |
| Modular Design | ❌ | ✅ | ❌ | ✅ |
| WASM Support | ❌ | ✅ | ✅ | ✅ (planned) |
| Great Docs | ⚠️ | ✅ | ⚠️ | ✅ (planned) |

**Runeforge's value proposition:**
> "libtcod-compatible API with modern Rust implementation"

This makes migration from libtcod easier than bracket-lib, while providing modern Rust benefits that tcod-rs lacks.

---

## 🚀 Getting Started Guide

### For Contributors

#### Prerequisites

- Rust 1.85+ (MSRV)
- Git
- Basic Rust knowledge

#### Clone and Build

```bash
git clone https://github.com/yourusername/runeforge-rl.git
cd runeforge-rl
cargo build
```

#### Run Examples

```bash
cargo run --example hello_world
cargo run --example fov_demo
cargo run --example pathfinding_demo
```

#### Run Tests

```bash
cargo test --all
cargo test --all --all-features
```

#### Run Benchmarks

```bash
cargo bench
```

### For Users

#### Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
runeforge-rl = "1.0"
```

Or with specific features:

```toml
[dependencies]
runeforge-rl = { version = "1.0", features = ["render-wgpu", "pathfinding", "fov"] }
```

#### Quick Start

```rust
use runeforge_rl::prelude::*;

fn main() -> Result<(), RuneforgeError> {
    let mut console = Console::new(80, 50);
    console.set_title("My Roguelike");

    console.print(1, 1, "Hello, world!", Color::WHITE, Color::BLACK);
    console.set_char(40, 25, '@', Color::YELLOW, Color::BLACK);

    console.run(|ctx| {
        // Game loop
        if ctx.input.is_key_pressed(Key::Escape) {
            return Action::Quit;
        }
        Action::Continue
    })
}
```

---

## 📖 Resources

### Official Documentation

- **API Docs:** <https://docs.rs/runeforge-rl>
- **Tutorial:** <https://runeforge.rs/tutorial>
- **Examples:** <https://github.com/yourusername/runeforge-rl/tree/main/examples>

### Rust Graphics Ecosystem

- [winit](https://github.com/rust-windowing/winit) - Window management
- [wgpu](https://wgpu.rs/) - Modern graphics API
- [pixels](https://lib.rs/crates/pixels) - Pixel buffer
- [LogRocket winit tutorial](https://blog.logrocket.com/create-manage-windows-rust-app-with-winit/)

### Pathfinding & Algorithms

- [pathfinding crate](https://crates.io/crates/pathfinding)
- [LogRocket pathfinding tutorial](https://blog.logrocket.com/pathfinding-rust-tutorial-examples/)

### Noise Generation

- [noise-rs](https://github.com/Razaekel/noise-rs)
- [bracket-noise](https://lib.rs/crates/bracket-noise)

### Font Rendering

- [ab_glyph](https://lib.rs/crates/ab_glyph)
- [cosmic-text](https://github.com/pop-os/cosmic-text)

### Image Processing

- [image crate](https://crates.io/crates/image)

### Roguelike Development

- [Roguelike Tutorial](http://rogueliketutorials.com/)
- [RogueBasin](http://roguebasin.com/)
- [r/roguelikedev](https://reddit.com/r/roguelikedev)

### Existing Rust Roguelike Libraries

- [bracket-lib](https://github.com/amethyst/bracket-lib)
- [doryen-rs](https://github.com/jice-nospam/doryen-rs)
- [tcod-rs (archived)](https://github.com/tomassedovic/tcod-rs)

---

## 🤝 Contributing

We welcome contributions! See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

### Areas Where Help Is Needed

- [ ] Algorithm implementations (FOV, pathfinding)
- [ ] Renderer backends
- [ ] Documentation and tutorials
- [ ] Example games
- [ ] Performance optimization
- [ ] Testing and bug reports

---

## 📄 License

Runeforge is licensed under the **BSD-3-Clause License**, the same as libtcod.

This allows commercial use, modification, and distribution with minimal restrictions.

---

## 🗺️ Roadmap

### Version 1.0 (Q2 2025)

- ✅ Core rendering (CPU and GPU)
- ✅ FOV algorithms
- ✅ Pathfinding
- ✅ Basic procedural generation
- ✅ Complete tutorial
- ✅ Cross-platform support

### Version 1.1 (Q3 2025)

- [ ] Advanced map generation algorithms
- [ ] GUI widgets
- [ ] Save/load utilities
- [ ] More examples and templates

### Version 2.0 (Q4 2025)

- [ ] 3D visualization support
- [ ] Advanced lighting and shadows
- [ ] Particle effects
- [ ] Sound integration
- [ ] Multiplayer utilities

---

## 📊 Progress Tracking

### Current Status: Phase 1 (Foundation)

| Crate | Status | Completion |
| ----- | ------ | ---------- |
| runeforge-color | 🔄 In Progress | 0% |
| runeforge-geometry | 📋 Planned | 0% |
| runeforge-random | 📋 Planned | 0% |
| runeforge-terminal | 📋 Planned | 0% |
| runeforge-fov | 📋 Planned | 0% |
| runeforge-pathfinding | 📋 Planned | 0% |
| runeforge-noise | 📋 Planned | 0% |
| runeforge-bsp | 📋 Planned | 0% |
| runeforge-algorithms | 📋 Planned | 0% |
| runeforge-tileset | 📋 Planned | 0% |
| runeforge-input | 📋 Planned | 0% |
| runeforge-core | 📋 Planned | 0% |

### Overall Progress: 5%

---
