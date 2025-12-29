# Runeforge Product Guide

## Overview

Runeforge is a modern, modular roguelike library for Rust, designed as a pure Rust replacement for libtcod. It provides experienced roguelike developers with a familiar API while leveraging modern Rust best practices and GPU-accelerated rendering.

## Target Audience

### Primary Users: Experienced Roguelike Developers Migrating from libtcod

Runeforge is built for developers who have existing experience with libtcod and are seeking a modern, maintained alternative in the Rust ecosystem. These users:

- Have existing roguelike projects built with libtcod or tcod-rs
- Are familiar with roguelike development patterns (FOV, pathfinding, dungeon generation)
- Want to leverage Rust's safety guarantees and performance
- Need a library that "just works" without C dependency headaches
- Value API familiarity to minimize migration effort

## Problem Statement

The Rust roguelike ecosystem faces a critical gap: **tcod-rs, the primary Rust bindings for libtcod, has been archived and is no longer maintained**. This leaves developers with few options:

- Continue using unmaintained bindings with potential security and compatibility issues
- Migrate to bracket-lib, which has a different API requiring significant code rewrites
- Build custom solutions from scratch

Runeforge addresses this by providing an actively developed, pure Rust library with a libtcod-compatible API.

## Key Differentiators

### 1. libtcod API Compatibility

Runeforge provides a familiar API for developers migrating from libtcod, minimizing the learning curve and reducing migration effort. Core concepts like Console, FOV algorithms, and pathfinding work similarly to their libtcod counterparts.

### 2. Modular Architecture

Unlike monolithic alternatives, Runeforge is organized as a Cargo workspace with 15 independent crates:

- **runeforge-color** — RGB/HSV color manipulation
- **runeforge-geometry** — 2D primitives (Point, Rect)
- **runeforge-random** — RNG with dice notation
- **runeforge-fov** — Field-of-view algorithms
- **runeforge-pathfinding** — A* and Dijkstra pathfinding
- **runeforge-bsp** — Binary space partitioning for dungeons
- **runeforge-console** — Backend-agnostic console abstraction
- **runeforge-terminal** — ANSI terminal rendering
- **runeforge-software** — CPU-based rendering
- **runeforge-pixels** — GPU-accelerated rendering

Developers can depend on only the crates they need, reducing binary size and compile times.

### 3. Multiple Rendering Backends

Runeforge supports three rendering backends through a unified Console trait:

- **GPU (pixels/wgpu)** — Hardware-accelerated rendering at 60+ FPS
- **Software (CPU)** — Portable rendering with PNG export
- **Terminal (ANSI)** — 24-bit RGB color output for terminal environments

Games can switch backends without code changes, enabling deployment across different platforms and use cases.

## Success Metrics

### Primary Metric: Migration Success

Runeforge will be considered successful when developers can port existing libtcod projects with minimal code changes. This is measured by:

- API coverage of core libtcod functionality
- Migration guide completeness and accuracy
- Community feedback on migration experience
- Number of successfully migrated projects

## Current Status & Roadmap

### Current Phase: Completing Core Algorithms (Phase 3)

**Completed:**

- Phase 1: Foundation (color, geometry, random) ✅
- Phase 2: Core Algorithms (FOV, pathfinding, BSP) ✅
- Phase 4: Rendering Backends (terminal, software, GPU) ✅

**In Progress:**

- Phase 3: Procedural Generation (noise, cellular automata, map algorithms)

**Upcoming:**

- Phase 5: Input Handling (keyboard, mouse)
- Phase 6: Documentation & Release (migration guide, crates.io publish)

### Near-Term Priority

Complete the procedural generation toolkit (Phase 3) to provide developers with:

- Perlin and Simplex noise generation
- Cellular automata for cave generation
- Drunkard's walk algorithm
- Comprehensive dungeon generation utilities

## Technical Foundation

- **Language:** Rust (Edition 2021, MSRV 1.85)
- **License:** BSD-3-Clause (same as libtcod)
- **Architecture:** Cargo workspace with 15 modular crates
- **Rendering:** winit 0.30, wgpu 28.0, pixels 0.15
- **Target Platforms:** Windows, macOS, Linux, WebAssembly
