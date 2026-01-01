# Runeforge Geometry

The `runeforge-geometry` crate provides 2D primitives essential for grid-based games.

## Key Types

### `IVec2`

Represents a 2D coordinate on a grid (x, y).

```rust
use runeforge_geometry::IVec2;

let p1 = IVec2::new(10, 5);
let p2 = IVec2::new(12, 5);
let distance = p1.distance(p2); // Euclidean distance
```

### `Rect`

Represents a rectangle, useful for rooms and bounds.

```rust
use runeforge_geometry::Rect;

let room = Rect::with_size(10, 10, 5, 5); // x, y, width, height
let center = room.center();
```

### `Circle`

Represents a circle, useful for radial effects.

## Iterators

Most shapes implement iterators to allow you to loop over every point contained within them.

```rust
for point in room.iter() {
    // Do something at each point in the room
}
```
