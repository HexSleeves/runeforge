# Pathfinding

The `runeforge-pathfinding` crate helps entities navigate your map.

## Algorithms

- **A*** (A-Star): Finds the shortest path to a single destination.
- **Dijkstra Map**: Calculates distance from one or more starting points to the entire map. Useful for fleeing, chasing multiple targets, or heatmaps.

## Usage (A*)

```rust
use runeforge_pathfinding::astar;

let start = IVec2::new(0, 0);
let end = IVec2::new(10, 10);

let path = astar(
    &start,
    |p| map.get_neighbors(p), // Successors function
    |p| p.distance(&end)      // Heuristic function
    |p| p == end              // Success function
);
```
