# Field of View (FOV)

The `runeforge-fov` crate provides algorithms to calculate visible areas on a grid.

## Algorithms

- **Recursive Shadowcasting**: Efficient and visually pleasing.
- **Symmetric Shadowcasting**: Ensures symmetry (if A sees B, B sees A).

## Usage

You must implement the `FovMap` trait (or similar interface) to provide transparency information about your map to the algorithm.

```rust
use runeforge_fov::{fov_shadowcast, FovMap};

// Implement the trait for your map struct
impl FovMap for MyMap {
    fn is_transparent(&self, x: i32, y: i32) -> bool {
        // Return true if light passes through this tile
        self.tiles[x][y].transparent
    }
}

// ... later ...
let mut visible_tiles = HashSet::new();
fov_shadowcast(&map, player_x, player_y, radius, &mut |x, y| {
    visible_tiles.insert((x, y));
});
```
