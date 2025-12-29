# Runeforge 🔥⚔️

## A modern, modular roguelike library for Rust

[![Crates.io](https://img.shields.io/crates/v/runeforge)](https://crates.io/crates/runeforge)
[![Documentation](https://docs.rs/runeforge/badge.svg)](https://docs.rs/runeforge)
[![License](https://img.shields.io/crates/l/runeforge)](https://github.com/yourusername/runeforge/blob/main/LICENSE)

## 🎯 About

Runeforge is a pure Rust roguelike library inspired by [libtcod](https://github.com/libtcod/libtcod), combining a familiar API with modern Rust best practices and GPU-accelerated rendering.

### Why Runeforge?

- **🦀 Pure Rust** - No C dependencies, easier builds
- **📦 Modular** - Use only what you need
- **⚡ Fast** - GPU-accelerated rendering with wgpu
- **🌐 Cross-platform** - Windows, macOS, Linux, and WebAssembly
- **📚 Well-documented** - Complete API docs and tutorials
- **🎮 libtcod-compatible** - Familiar API for easy migration

## 🚀 Quick Start

### Installation

Add Runeforge to your `Cargo.toml`:

```toml
[dependencies]
runeforge-core = "0.1"
```

### Hello World

```rust
use runeforge_core::prelude::*;

fn main() {
    let mut rng = Rng::new();
    let point = Point::new(10, 20);
    let color = Color::RED;

    println!("Random d6 roll: {}", rng.roll_dice(1, 6));
    println!("Point at ({}, {})", point.x, point.y);
    println!("Color: {}", color);
}
```

## 📦 Crate Structure

Runeforge is organized as a modular workspace:

| Crate | Description | Status |
|-------|-------------|--------|
| `runeforge-core` | Unified facade, re-exports all crates | ✅ Basic |
| `runeforge-color` | RGB/HSV color manipulation | ✅ Complete |
| `runeforge-geometry` | 2D primitives (Point, Rect) | ✅ Complete |
| `runeforge-random` | RNG with dice notation | ✅ Complete |
| `runeforge-fov` | Field-of-view algorithms | ✅ Complete |
| `runeforge-pathfinding` | A* and Dijkstra pathfinding | ✅ Complete |
| `runeforge-bsp` | Binary space partitioning | ✅ Complete |
| `runeforge-terminal` | Console rendering (CPU/GPU) | ✅ Complete |
| `runeforge-tileset` | Font and tileset loading | ✅ Complete |
| `runeforge-input` | Keyboard and mouse input | ✅ Complete |
| `runeforge-pixels` | GPU backend using `pixels` | ✅ Complete |
| `runeforge-software` | CPU backend (software rendering) | ✅ Complete |
| `runeforge-console` | Abstract console trait | ✅ Complete |
| `runeforge-noise` | Procedural noise generation | 📋 Planned (Phase 3) |
| `runeforge-algorithms` | Line drawing, flood fill, etc. | 📋 Planned (Phase 3) |

## 🎨 Features

### Current Features (v0.1)

- ✅ **Color System**: RGB/HSV conversion, blending, named colors
- ✅ **Geometry**: Point and Rect types with distance calculations
- ✅ **Random Numbers**: Dice notation parsing (`3d6+2`), weighted selection
- ✅ **FOV Algorithms**: Symmetric shadowcasting with precise fraction-based calculation
- ✅ **Pathfinding**: A* algorithm with 4-way and 8-way movement
- ✅ **Procedural Generation**: BSP dungeon generation (rooms and corridors)
- ✅ **Rendering**: 
  - Abstract `Console` trait for backend-agnostic code
  - GPU-accelerated backend (wgpu/pixels)
  - Software backend (CPU buffer, PNG export)
  - Terminal backend (ANSI escape codes, truecolor)
- ✅ **Tilesets**: Support for TrueType/OpenType fonts and bitmap tilesets
- ✅ **Input**: Action-based input mapping (keyboard/mouse) with support for vi-keys, WASD, arrows

### Planned Features

- 🔨 **Procedural Generation**: Noise (Perlin/Simplex), cellular automata (Phase 3)
- 🔨 **Algorithms**: Line drawing, flood fill (Phase 3)
- 🔨 **Advanced Input**: Gestures, gamepad support (Phase 5)
- 🔨 **UI Framework**: Widgets, layouts (Future)

## 📚 Examples

Check out the `examples/` directory for complete examples:

```bash
cargo run --example hello_terminal
cargo run --example roguelike_demo
cargo run --example windowed_roguelike
cargo run --example fov_demo
cargo run --example pathfinding_demo
cargo run --example bsp_demo
```

## 🔧 Development Status

Runeforge is currently in **Phase 5** of development (Input & Integration). See [RUNEFORGE.md](RUNEFORGE.md) for the complete roadmap.

### Current Progress: ~70%

- ✅ Project structure and workspace setup
- ✅ Core crates: color, geometry, random
- ✅ Core Algorithms: FOV, Pathfinding, BSP
- ✅ Rendering system: GPU, Software, Terminal backends
- ✅ Input system: Action mapping, keyboard/mouse
- 🔨 Procedural Generation: Noise, advanced map gen (Next)
- 📋 Example Game: Complete roguelike integration (In Progress)

## 🤝 Contributing

We welcome contributions! See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

### Areas Where Help Is Needed

- Algorithm implementations (Noise, Line drawing)
- Advanced map generation (Cellular Automata, Drunkard's Walk)
- Documentation and tutorials
- Example roguelikes
- Testing and benchmarks

## 📖 Documentation

- **API Documentation**: <https://docs.rs/runeforge-core>
- **Tutorial**: Coming soon
- **Architecture Guide**: See [RUNEFORGE.md](RUNEFORGE.md)
- **Migration from libtcod**: Coming soon

## 🗺️ Roadmap

### Version 0.1 (Foundation) - Completed

- ✅ Workspace structure
- ✅ Color, geometry, random crates
- ✅ Basic documentation

### Version 0.2 (Algorithms) - Completed

- ✅ FOV algorithms
- ✅ Pathfinding
- ✅ BSP trees

### Version 0.3 (Rendering & Input) - Completed

- ✅ Rendering backends (GPU, Soft, Term)
- ✅ Tileset/Font loading
- ✅ Input handling

### Version 0.4 (Procedural Gen) - Planned

- Noise generation
- Advanced map generation
- Line drawing / Geom utils

### Version 1.0 (Full Release)

- Complete example game
- Comprehensive documentation
- Tutorial and examples

See [RUNEFORGE.md](RUNEFORGE.md) for the detailed roadmap.

## 📄 License

Runeforge is licensed under the **BSD-3-Clause License**, the same as libtcod.

This allows commercial use, modification, and distribution with minimal restrictions.

See [LICENSE](LICENSE) for details.

## 🙏 Acknowledgments

- [libtcod](https://github.com/libtcod/libtcod) - The inspiration for this library
- [bracket-lib](https://github.com/amethyst/bracket-lib) - Modular architecture inspiration
- [doryen-rs](https://github.com/jice-nospam/doryen-rs) - Pure Rust roguelike library by libtcod's creator
- The Rust gamedev community

## 🔗 Links

- **GitHub**: <https://github.com/yourusername/runeforge>
- **Crates.io**: <https://crates.io/crates/runeforge-core>
- **Discord**: Coming soon
- **r/roguelikedev**: <https://reddit.com/r/roguelikedev>

---

*Built with ❤️ in Rust*
