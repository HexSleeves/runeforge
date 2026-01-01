# Runeforge 🔥⚔️

A modern, modular roguelike library for Rust

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
| ----- | ----------- | ------ |
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

## 📄 License

Runeforge is licensed under the **BSD-3-Clause License**, the same as libtcod.

This allows commercial use, modification, and distribution with minimal restrictions.
