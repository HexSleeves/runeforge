# Runeforge Demo Game

A complete playable roguelike demo showcasing all Runeforge systems working together with pixel-perfect rendering.

## Features

This demo integrates:

- **Procedural Map Generation**: BSP algorithm creates structured dungeons with rooms and corridors
- **Field of View**: Symmetric shadowcasting reveals only visible areas
- **Player Movement**: Smooth 8-directional movement with multiple control schemes
- **Memory System**: Explored areas remain visible when out of sight
- **Pixel Rendering**: Hardware-accelerated rendering via wgpu (Vulkan/Metal/DX12)
- **Input Abstraction**: Unified input system supporting vi-keys, arrows, WASD, and numpad

## How to Play

### Running the Game

```bash
cargo run --package demo-game
```

Or run the compiled binary directly:

```bash
./target/debug/demo-game
```

### Controls

The game supports multiple control schemes via `runeforge-input`:

**Arrow Keys**
- ↑ ↓ ← → : Move in cardinal directions

**WASD**
- W A S D : Move in cardinal directions

**Vi-keys** (for roguelike purists)
- h j k l : Move left, down, up, right
- y u b n : Move diagonally (NW, NE, SW, SE)

**Numpad**
- 8 4 6 2 : Cardinal movement
- 7 9 1 3 : Diagonal movement

**Other**
- ESC : Quit the game

### Gameplay

- You appear as a yellow '@' symbol in the center of the first room
- Explore the procedurally generated dungeon by moving around
- Your field of view extends 8 tiles in all directions
- Walls are rendered as '#' symbols, floors as '.' symbols
- Characters are rendered using the classic terminal8x8 tileset
- Areas you've explored but can't currently see appear dimmed
- Unexplored areas are completely black

## Architecture

The game demonstrates clean integration of Runeforge crates with modern graphics:

```rust
// Window and rendering
winit 0.30 - Window creation and event loop
pixels 0.15 - GPU-accelerated pixel buffer (via wgpu)

// Game systems
runeforge-algorithms - BSP dungeon generation
runeforge-fov - Symmetric shadowcasting
runeforge-input - Virtual key mapping
runeforge-geometry - Point-based positioning
runeforge-random - Deterministic RNG
```

## Technical Details

### Rendering Pipeline

1. **Window**: Created via winit's `ApplicationHandler` trait (1920x960 pixels)
2. **Pixel Buffer**: 640x320 (80×40 tiles at 8×8 pixels each)
3. **Tileset**: terminal8x8_gs_ro.png (classic libtcod font, CP437 encoding)
4. **Scaling**: 3x upscaling via GPU (crisp pixel art scaling)
5. **Performance**: Hardware-accelerated via wgpu

### Input Pipeline

1. **Physical Key**: winit provides `KeyCode` events
2. **Translation**: `InputMap` converts to `VirtualKey` actions
3. **State Tracking**: `InputState` maintains pressed keys
4. **Game Logic**: Processes `VirtualKey::Move(Direction)`

### Code Structure

- **Game State**: Holds map, player position, visibility data
- **Tileset**: Loads and manages the terminal8x8 font bitmap
- **ApplicationHandler**: Manages window lifecycle and events
- **Game Loop**: Event-driven rendering with `RedrawRequested`
- **FOV System**: Recalculated on every move
- **Glyph Rendering**: Copies 8x8 character glyphs from tileset with tinting

## Performance

- Map generation: < 1ms (80x40 BSP dungeon)
- FOV calculation: Real-time per move
- Rendering: 60+ FPS hardware-accelerated
- Input lag: Sub-millisecond response

## Graphics Backend

The demo uses **pixels** which provides:
- **wgpu** for modern graphics APIs (Vulkan, Metal, DX12, OpenGL ES3)
- **Automatic scaling** from pixel buffer to window
- **Custom shaders** support (not used in this demo)
- **Cross-platform** rendering

Sources:
- [Pixels Documentation](https://docs.rs/crate/pixels/latest)
- [Pixels GitHub](https://github.com/parasyte/pixels)
- [Building Conway's Game of Life in Rust](https://www.40tude.fr/docs/06_programmation/rust/017_game_of_life/game_of_life_00.html)

## Tileset

The demo uses **terminal8x8_gs_ro.png** from the libtcod library:
- **Size**: 8x8 pixels per character
- **Format**: Greyscale, row layout, CP437 encoding
- **Grid**: 16x16 characters (256 glyphs total)
- **License**: Part of libtcod (BSD-3-Clause)

Source:
- [libtcod Python TCOD Fonts](https://github.com/libtcod/python-tcod/tree/main/fonts/libtcod)

## Future Enhancements

Potential additions to showcase more Runeforge features:
- Enemies with AI pathfinding
- Items and inventory system
- Combat mechanics
- Multiple dungeon levels with stairs
- Different map generation algorithms (caves, drunkard's walk)
- Color variations for different tile types
- Mouse-based UI

## License

This demo is part of the Runeforge library and shares its license.
