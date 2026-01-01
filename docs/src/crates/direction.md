# Runeforge Direction

The `runeforge-direction` crate helps manage movement on a grid.

## Features

- **Enumerations**: `Direction` enum covering Cardinal (N, S, E, W) and Ordinal (NE, NW, SE, SW) directions.
- **Conversion**: Convert between vectors and direction enums.
- **Iterators**: Iterate over adjacent neighbors.

## Usage

```rust
use runeforge_direction::Direction;

let dir = Direction::North;
let delta = dir.as_vec(); // Returns IVec2(0, -1) typically (depending on coordinate system)
```
