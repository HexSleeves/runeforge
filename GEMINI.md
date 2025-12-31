# Runeforge Context for Gemini

## Project Overview

**Runeforge** is a modern, modular, pure Rust roguelike library inspired by `libtcod`. It provides a suite of tools for roguelike development, including rendering (terminal, software, GPU), field-of-view (FOV), pathfinding, input handling, and map generation.

**Key Features:**

* **Modular Architecture:** Functionality is split into granular crates (e.g., `runeforge-fov`, `runeforge-terminal`).
* **Cross-Platform:** Supports Windows, macOS, Linux, and WebAssembly.
* **Rendering:** Multiple backends including GPU-accelerated (`wgpu`/`pixels`), software, and traditional terminal.
* **Algorithms:** Includes A*, Dijkstra, Shadowcasting FOV, BSP dungeon generation, and more.

## Architecture & Directory Structure

The project is organized as a Rust Workspace.

### Core Structure

* `Cargo.toml`: The workspace root configuration.
* `src/lib.rs`: The top-level `runeforge-rl` crate, acting as a facade for the sub-crates.
* `crates/`: Contains the individual feature crates.
* `examples/`: Usage examples demonstrating various features.

### Workspace Members (Crates)

* **Core:**
  * `runeforge-color`: RGB/HSV color manipulation.
  * `runeforge-geometry`: 2D primitives (IVec2, Rect).
  * `runeforge-random`: RNG with dice notation support.
  * `runeforge-direction`: Directional enums and logic.
* **Algorithms:**
  * `runeforge-fov`: Field-of-view algorithms (Shadowcasting).
  * `runeforge-pathfinding`: Pathfinding (A*, Dijkstra).
  * `runeforge-algorithms`: General roguelike algorithms (drawing, flood fill).
  * `runeforge-noise`: Procedural noise generation (Planned/In-progress).
* **System & IO:**
  * `runeforge-terminal`: Abstract console trait and terminal rendering.
  * `runeforge-input`: Input handling (Keyboard/Mouse actions).
  * `runeforge-tileset`: Font and tileset loading.

## Building and Running

### Prerequisites

* Rust Toolchain (Stable or Nightly as per `rust-toolchain.toml` or CI).
* System dependencies for `wgpu`/`winit` (platform specific, usually standard graphics libs).

### Common Commands

* **Build Workspace:**

    ```bash
    cargo build --workspace --all-features
    ```

* **Run Tests:**

    ```bash
    cargo test --workspace --all-features
    ```

* **Run Linter (Clippy):**

    ```bash
    cargo clippy --workspace --all-targets --all-features -- -D warnings
    ```

* **Check Formatting:**

    ```bash
    cargo fmt --all --check
    ```

* **Build Documentation:**

    ```bash
    cargo doc --workspace --all-features --no-deps
    ```

### Running Examples

Run examples to verify functionality or learn the API:

```bash
cargo run --example hello_terminal
cargo run --example roguelike_demo
cargo run --example fov_demo
cargo run --example bsp_demo
```

## Development Conventions

* **Style:** Follow standard Rust conventions (`rustfmt`).
* **Linting:** The project enforces strict clippy lints (`-D warnings`).
* **Testing:** All features should have accompanying unit tests. Benchmarks are located in `benches/`.
* **Documentation:** Public APIs must be documented.
* **Commits:** Use clear, descriptive commit messages.

## Current Development Status

* **Phase:** 5 (Input & Integration).
* **Overall Progress:** ~70%.
* **Missing/Planned:** Advanced procedural generation (Cellular Automata), more complex noise, UI framework.
