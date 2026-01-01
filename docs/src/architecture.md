# Architecture

Runeforge is designed as a **modular monorepo** inspired by [bracket-lib](https://github.com/amethyst/bracket-lib). This allows developers to pick and choose exactly which components they need, keeping binary sizes small and compile times fast.

## Workspace Structure

The project is organized into several crates within the `crates/` directory:

```text
runeforge-rl/
├── Cargo.toml                 # Workspace configuration
├── crates/
│   ├── runeforge-algorithms/ # General roguelike algorithms (drawing, flood fill)
│   ├── runeforge-color/      # RGB/HSV color manipulation
│   ├── runeforge-direction/  # Grid direction handling
│   ├── runeforge-fov/        # Field-of-view algorithms
│   ├── runeforge-geometry/   # 2D geometric primitives
│   ├── runeforge-input/      # Input handling
│   ├── runeforge-noise/      # Noise generation
│   ├── runeforge-pathfinding/# Pathfinding algorithms
│   ├── runeforge-random/     # Random number generation
│   ├── runeforge-terminal/   # Terminal/console rendering
│   ├── runeforge-tileset/    # Tileset/font loading
└── src/                      # Facade crate (runeforge-rl)
```

## Crate Dependency Graph

A simplified view of how the core crates interact:

- **Core Utilities**: `runeforge-geometry`, `runeforge-color`, `runeforge-random` are the foundation.
- **Algorithms**: `runeforge-fov`, `runeforge-pathfinding`, `runeforge-algorithms` depend on the core utilities.
- **Rendering**: `runeforge-terminal` brings everything together for display, depending on `runeforge-tileset` for fonts and `runeforge-color` for styling.
- **Input**: `runeforge-input` handles user interaction.

## Design Decisions

### 1. Pure Rust

Runeforge avoids C dependencies (unlike the original libtcod bindings) to ensure easier cross-platform builds and memory safety.

### 2. Rendering Strategy

We support multiple backends via traits:

- **GPU (wgpu)**: High performance, hardware acceleration.
- **Software**: Fallback for systems without GPU access or for simple framebuffer operations.
- **Terminal**: Direct ANSI output for CLI tools or SSH play.

### 3. Libtcod Compatibility

While idiomatic Rust is prioritized, the API structure intentionally mirrors [libtcod](https://github.com/libtcod/libtcod) where sensible to make migration easier for developers coming from Python or C++.
