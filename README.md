# Runeforge 🔥⚔️

**A modern, modular roguelike library for Rust**

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
| `runeforge-fov` | Field-of-view algorithms | 📋 Planned (Phase 2) |
| `runeforge-pathfinding` | A* and Dijkstra pathfinding | 📋 Planned (Phase 2) |
| `runeforge-noise` | Procedural noise generation | 📋 Planned (Phase 3) |
| `runeforge-bsp` | Binary space partitioning | 📋 Planned (Phase 3) |
| `runeforge-algorithms` | Line drawing, flood fill, etc. | 📋 Planned (Phase 2) |
| `runeforge-terminal` | Console rendering (CPU/GPU) | 📋 Planned (Phase 1-4) |
| `runeforge-tileset` | Font and tileset loading | 📋 Planned (Phase 4) |
| `runeforge-input` | Keyboard and mouse input | 📋 Planned (Phase 5) |

## 🎨 Features

### Current Features (v0.1)

- ✅ **Color System**: RGB/HSV conversion, blending, named colors
- ✅ **Geometry**: Point and Rect types with distance calculations
- ✅ **Random Numbers**: Dice notation parsing (`3d6+2`), weighted selection

### Planned Features

- 🔨 **FOV Algorithms**: Shadowcasting, raycasting, and more (Phase 2)
- 🔨 **Pathfinding**: A*, Dijkstra maps (Phase 2)
- 🔨 **Procedural Generation**: Noise, BSP, cellular automata (Phase 3)
- 🔨 **Rendering**: GPU-accelerated console with multiple backends (Phase 1-4)
- 🔨 **Input Handling**: Keyboard and mouse events (Phase 5)

## 📚 Examples

Check out the `examples/` directory for complete examples:

```bash
cargo run --example dice_roller
cargo run --example color_blending
cargo run --example geometry_demo
```

## 🔧 Development Status

Runeforge is currently in **Phase 1** of development. See [RUNEFORGE.md](RUNEFORGE.md) for the complete roadmap and implementation plan.

### Current Progress: ~5%

- ✅ Project structure and workspace setup
- ✅ Core crates: color, geometry, random
- 🔨 Rendering system (in progress)
- 📋 Algorithms and utilities (planned)
- 📋 Advanced features (planned)

## 🤝 Contributing

We welcome contributions! See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

### Areas Where Help Is Needed

- Algorithm implementations (FOV, pathfinding)
- Renderer backends (wgpu, software)
- Documentation and tutorials
- Example roguelikes
- Testing and benchmarks

## 📖 Documentation

- **API Documentation**: <https://docs.rs/runeforge-core>
- **Tutorial**: Coming soon
- **Architecture Guide**: See [RUNEFORGE.md](RUNEFORGE.md)
- **Migration from libtcod**: Coming soon

## 🗺️ Roadmap

### Version 0.1 (Current) - Foundation
- ✅ Workspace structure
- ✅ Color, geometry, random crates
- 🔨 Basic documentation

### Version 0.2 - Core Algorithms
- FOV algorithms
- Pathfinding
- Line drawing

### Version 0.3 - Procedural Generation
- Noise generation
- BSP trees
- Map generation

### Version 1.0 - Full Release
- Complete rendering system
- All algorithms implemented
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
