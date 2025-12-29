# Runeforge Technology Stack

## Overview

This document defines the complete technology stack for the Runeforge roguelike library. All dependencies are carefully selected to balance functionality, performance, and maintainability.

---

## Core Language & Build System

| Technology | Version | Purpose |
| ---------- | ------- | ------- |
| **Rust** | Edition 2021 | Primary programming language |
| **MSRV** | 1.85 | Minimum Supported Rust Version |
| **Cargo** | Workspace | Build system and package manager |

### Workspace Structure

Runeforge is organized as a Cargo workspace with **15 modular crates**:

- `runeforge-core` — Unified facade crate
- `runeforge-color` — RGB/HSV color manipulation
- `runeforge-geometry` — 2D primitives (Point, Rect)
- `runeforge-random` — RNG with dice notation
- `runeforge-fov` — Field-of-view algorithms
- `runeforge-pathfinding` — A* and Dijkstra pathfinding
- `runeforge-bsp` — Binary space partitioning
- `runeforge-console` — Backend-agnostic console abstraction
- `runeforge-terminal` — ANSI terminal rendering
- `runeforge-software` — CPU-based rendering
- `runeforge-pixels` — GPU-accelerated rendering
- `runeforge-tileset` — Font and tileset loading
- `runeforge-input` — Keyboard and mouse input
- `runeforge-algorithms` — Line drawing, flood fill, etc.
- `runeforge-noise` — Procedural noise generation

---

## Window Management & Input

| Dependency | Version | Purpose |
| ---------- | ------- | ------- |
| **winit** | 0.30 | Cross-platform window creation and event handling |

**Features:** `rwh_06` (raw window handle support for wgpu integration)

---

## Rendering Stack

### GPU Rendering

| Dependency | Version | Purpose |
| ---------- | ------- | ------- |
| **wgpu** | 28.0.0 | Modern graphics API (Vulkan/Metal/DX12/WebGL) |
| **pixels** | 0.15.0 | Pixel buffer abstraction for 2D rendering |
| **bytemuck** | 1.24.0 | Safe byte casting for GPU buffers |

### Font & Text Rendering

| Dependency | Version | Purpose |
| ---------- | ------- | ------- |
| **ab_glyph** | 0.2 | TrueType font loading and rasterization |
| **cosmic-text** | 0.15 | Advanced text layout and shaping |
| **bdf-parser** | 0.1 | Bitmap Distribution Format (BDF) font parsing |

### Image Processing

| Dependency | Version | Purpose |
| ---------- | ------- | ------- |
| **image** | 0.25 | Image loading/saving (PNG support) |

**Features:** `png` only (minimal feature set)

---

## Algorithms & Utilities

### Pathfinding & FOV

| Dependency | Version | Purpose |
| ---------- | ------- | ------- |
| **rustc-hash** | 2.0 | Fast hash maps for integer keys (pathfinding) |
| **smallvec** | 1.13 | Stack-allocated small vectors |

**Note:** Both FOV and pathfinding algorithms are implemented natively in `runeforge-fov` and `runeforge-pathfinding` (no external algorithm dependencies).

### Procedural Generation

| Dependency | Version | Purpose |
| ---------- | ------- | ------- |
| **noise** | 0.9 | Perlin, Simplex, and other noise functions |
| **bresenham** | 0.1 | Bresenham line drawing algorithm |
| **line_drawing** | 1.0 | Additional line drawing algorithms |

**Note:** BSP algorithms are implemented natively in `runeforge-bsp`.

### Random Number Generation

| Dependency | Version | Purpose |
| ---------- | ------- | ------- |
| **rand** | 0.9 | Random number generation and distributions |

### Color Manipulation

| Dependency | Version | Purpose |
| ---------- | ------- | ------- |
| **palette** | 0.7 | Color space conversions (RGB, HSV, etc.) |

### Compression

| Dependency | Version | Purpose |
| ---------- | ------- | ------- |
| **flate2** | 1.1 | DEFLATE compression (for REXPaint files) |

---

## Error Handling & Logging

| Dependency | Version | Purpose |
| ---------- | ------- | ------- |
| **thiserror** | 2.0 | Derive macro for error types |
| **anyhow** | 1.0 | Flexible error handling for applications |
| **log** | 0.4 | Logging facade |

---

## Serialization (Optional)

| Dependency | Version | Purpose |
| ---------- | ------- | ------- |
| **serde** | 1.0 | Serialization framework |

**Features:** `derive` (derive macros for Serialize/Deserialize)

**Usage:** Feature-gated for optional save/load functionality.

---

## Development & Testing

| Dependency | Version | Purpose |
| ---------- | ------- | ------- |
| **criterion** | 0.8 | Statistical benchmarking framework |

---

## Target Platforms

Runeforge targets the following platforms:

| Platform | Status | Notes |
| -------- | ------ | ----- |
| **Linux (x86_64)** | ✅ Supported | Primary development platform |
| **macOS (Intel)** | ✅ Supported | Tested on Intel Macs |
| **macOS (Apple Silicon)** | ✅ Supported | Native ARM64 support |
| **Windows (x86_64)** | ✅ Supported | Windows 10+ |
| **WebAssembly** | 🔄 Planned | Via wgpu WebGL backend |

---

## Dependency Philosophy

### Minimal Dependencies

Each crate minimizes external dependencies:

- Prefer standard library solutions where reasonable
- Use feature flags for optional heavy dependencies
- Regularly audit dependencies with `cargo deny`

### Feature Flags

Heavy dependencies are feature-gated:

```toml
[features]
default = ["render-pixels"]
render-pixels = ["pixels", "wgpu", "winit"]
render-software = ["image"]
render-terminal = []
serialization = ["serde"]
```

### Dependency Audit

Runeforge uses `cargo deny` to enforce:

- No unmaintained dependencies
- No dependencies with known security vulnerabilities
- License compatibility (BSD-3-Clause)

---

## Build Configuration

### Development Profile

```toml
[profile.dev]
opt-level = 1  # Faster debug builds
```

### Release Profile

```toml
[profile.release]
lto = true           # Link-time optimization
codegen-units = 1    # Single codegen unit for better optimization
opt-level = 3        # Maximum optimization
```

---

## Summary

| Category | Key Technologies |
| -------- | ---------------- |
| **Language** | Rust 2021 (MSRV 1.85) |
| **Build** | Cargo workspace (15 crates) |
| **Windowing** | winit 0.30 |
| **Rendering** | wgpu 28.0, pixels 0.15 |
| **Fonts** | ab_glyph, bdf-parser |
| **Algorithms** | Native A*/FOV, noise, bresenham |
| **Utilities** | rand, palette, thiserror |
| **Testing** | criterion benchmarks |
