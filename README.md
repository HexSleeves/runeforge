# Runeforge 🔥⚔️

## A modern, modular roguelike library for Rust

[![Crates.io](https://img.shields.io/crates/v/runeforge-rl)](https://crates.io/crates/runeforge-rl)
[![Documentation](https://docs.rs/runeforge-rl/badge.svg)](https://docs.rs/runeforge-rl)
[![License](https://img.shields.io/crates/l/runeforge-rl)](https://github.com/yourusername/runeforge-rl/blob/main/LICENSE)

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
runeforge-rl = "0.1"
```

### Hello World

```rust
use runeforge_rl::prelude::*;

fn main() {
    let mut rng = Rng::new();
    let point = IVec2::new(10, 20);
    let color = Color::RED;

    println!("Random d6 roll: {}", rng.roll_dice(1, 6));
    println!("IVec2 at ({:?})", point);
    println!("Color: {}", color);
}
```

## 📦 Crate Structure

Runeforge is organized as a modular workspace:

| Crate | Description | Status |
|-------|-------------|--------|
| `runeforge-algorithms` | Procedural map generation (BSP, Caves, etc.) | ✅ Complete |
| `runeforge-color` | RGB/HSV color manipulation | ✅ Complete |
| `runeforge-direction` | Grid-based direction handling | ✅ Complete |
| `runeforge-fov` | Field-of-view algorithms | ✅ Complete |
| `runeforge-geometry` | 2D primitives (IVec2, Rect) | ✅ Complete |
| `runeforge-input` | Keyboard and mouse input | ✅ Complete |
| `runeforge-noise` | Procedural noise generation | ✅ Complete |
| `runeforge-pathfinding` | A* and Dijkstra pathfinding | ✅ Complete |
| `runeforge-random` | RNG with dice notation | ✅ Complete |
| `runeforge-terminal` | Console rendering (CPU/GPU/ANSI) | ✅ Complete |
| `runeforge-tileset` | Font and tileset loading | ✅ Complete |

## 🎨 Features

### Current Features (v0.1)

- ✅ **Color System**: RGB/HSV conversion, blending, named colors
- ✅ **Geometry**: IVec2 and Rect types with iterators
- ✅ **Random Numbers**: Seedable RNG, dice notation parsing (`3d6+2`), weighted selection
- ✅ **FOV Algorithms**: Symmetric shadowcasting with precise fraction-based calculation
- ✅ **Pathfinding**: A* and Dijkstra pathfinding via the `pathfinding` crate
- ✅ **Procedural Generation**: BSP dungeons, Cellular Automata caves, and Drunkard's Walk tunnels
- ✅ **Noise Generation**: 2D Perlin noise maps
- ✅ **Rendering**:
  - Abstract `Console` trait for backend-agnostic code
  - Software backend (CPU buffer, PNG export)
  - Terminal backend (ANSI escape codes)
- ✅ **Tilesets**: Support for TrueType/OpenType fonts and bitmap tilesets
- ✅ **Input**: Action-based input mapping (keyboard/mouse) with support for vi-keys, WASD, arrows

### Planned Features

- 🔨 **GPU Renderer**: A `wgpu`-based renderer is available but needs integration into the main library.
- 🔨 **UI Framework**: A simple UI toolkit for buttons, text boxes, etc. is a long-term goal.
- 🔨 **Advanced Algorithms**: More procedural generation and pathfinding options.

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

Runeforge is now largely complete and documented. The core modules are stable and ready for use.

### Current Progress: ~90%

- ✅ Core crates: color, geometry, random, direction, distance, utils
- ✅ Core Algorithms: FOV, Pathfinding, BSP, Caves, Drunkard's Walk, Noise
- ✅ Rendering system: Software and Terminal backends are stable.
- ✅ Input system: Action mapping for keyboard/mouse.
- ✅ Documentation: All public APIs are now documented with examples.
- 🔨 Example Game: A complete roguelike demo is in progress.

## 🤝 Contributing

We welcome contributions! See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

### Areas Where Help Is Needed

- GPU renderer integration and examples.
- More advanced map generation algorithms.
- UI framework design and implementation.
- Example roguelikes and demos.
- Performance benchmarks.

## 📖 Documentation

- **API Documentation**: <https://docs.rs/runeforge-rl>
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

- **GitHub**: <https://github.com/yourusername/runeforge-rl>
- **Crates.io**: <https://crates.io/crates/runeforge-rl>
- **Discord**: Coming soon
- **r/roguelikedev**: <https://reddit.com/r/roguelikedev>

---

*Built with ❤️ in Rust*
