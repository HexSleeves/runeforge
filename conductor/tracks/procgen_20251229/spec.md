# Track Specification: Procedural Generation Toolkit

## Track ID
`procgen_20251229`

## Overview

This track implements Phase 3 of the Runeforge roadmap: a comprehensive procedural generation toolkit for roguelike map generation. The track delivers noise generation capabilities and map generation algorithms that work seamlessly with existing Runeforge crates.

## Goals

1. **Noise Generation** — Implement `runeforge-noise` crate with Perlin and Simplex noise
2. **Map Algorithms** — Extend `runeforge-algorithms` with cellular automata and drunkard's walk
3. **Integration** — Ensure seamless integration with existing crates (geometry, random, BSP)
4. **Examples** — Provide working examples demonstrating each algorithm
5. **Performance** — Maintain competitive performance with benchmarks

## Target Users

Roguelike developers who need:
- Procedural cave generation using cellular automata
- Organic dungeon layouts using drunkard's walk
- Terrain generation using noise functions
- Flexible, composable map generation tools

## Success Criteria

- [ ] `runeforge-noise` crate implements 2D/3D Perlin and Simplex noise
- [ ] `runeforge-algorithms` includes cellular automata and drunkard's walk
- [ ] All algorithms have >80% test coverage
- [ ] Examples demonstrate each algorithm with visual output
- [ ] Benchmarks show competitive performance
- [ ] Documentation includes usage examples and algorithm explanations
- [ ] All code follows Rust style guidelines
- [ ] No compiler warnings or Clippy errors

## Technical Requirements

### Noise Generation (`runeforge-noise`)

**Dependencies:**
- `noise` crate (0.9) for underlying noise implementations
- `runeforge-geometry` for Point/Rect types

**Public API:**
```rust
pub struct NoiseGenerator {
    // Wraps noise-rs generators
}

impl NoiseGenerator {
    pub fn perlin(seed: u32) -> Self;
    pub fn simplex(seed: u32) -> Self;
    pub fn sample_2d(&self, x: f64, y: f64) -> f64;
    pub fn sample_3d(&self, x: f64, y: f64, z: f64) -> f64;
}

// Convenience functions
pub fn generate_heightmap(width: u32, height: u32, generator: &NoiseGenerator) -> Vec<Vec<f64>>;
```

**Features:**
- Perlin noise (2D and 3D)
- Simplex noise (2D and 3D)
- Configurable frequency, amplitude, octaves
- Seed-based reproducibility
- Heightmap generation utilities

### Map Generation Algorithms (`runeforge-algorithms`)

**Dependencies:**
- `runeforge-geometry` for Point/Rect types
- `runeforge-random` for RNG

**Cellular Automata:**
```rust
pub struct CellularAutomata {
    width: u32,
    height: u32,
    map: Vec<Vec<bool>>, // true = wall, false = floor
}

impl CellularAutomata {
    pub fn new(width: u32, height: u32, initial_wall_probability: f64) -> Self;
    pub fn step(&mut self, birth_limit: u8, death_limit: u8);
    pub fn run(&mut self, iterations: u32, birth_limit: u8, death_limit: u8);
    pub fn get(&self, x: u32, y: u32) -> bool;
}
```

**Drunkard's Walk:**
```rust
pub struct DrunkardWalk {
    width: u32,
    height: u32,
    map: Vec<Vec<bool>>, // true = floor, false = wall
}

impl DrunkardWalk {
    pub fn new(width: u32, height: u32) -> Self;
    pub fn carve(&mut self, start: Point, target_floor_percentage: f64);
    pub fn get(&self, x: u32, y: u32) -> bool;
}
```

### Examples

1. **Noise Visualization** — Generate and export heightmap as PNG
2. **Cave Generation** — Cellular automata cave with configurable parameters
3. **Organic Dungeon** — Drunkard's walk dungeon with multiple starting points
4. **Combined Generation** — Use noise + cellular automata for varied terrain

## Non-Goals

- Advanced noise types (Worley, Voronoi) — deferred to future versions
- 3D map generation — focus on 2D for roguelikes
- Real-time noise generation — optimization deferred
- GUI tools for map editing — command-line examples only

## Dependencies

### Existing Crates (Already Implemented)
- `runeforge-geometry` — Point, Rect types
- `runeforge-random` — RNG with seeding
- `runeforge-bsp` — BSP dungeon generation (for comparison)

### External Crates
- `noise` 0.9 — Noise generation algorithms
- `image` 0.25 — PNG export for examples (already in workspace)

### New Crates (To Be Implemented)
- `runeforge-noise` — Noise generation wrapper
- `runeforge-algorithms` — Map generation algorithms (extend existing stub)

## Testing Strategy

### Unit Tests
- Noise generator produces consistent output for same seed
- Cellular automata follows birth/death rules correctly
- Drunkard's walk respects boundaries
- Edge cases (0x0 maps, 100% wall probability, etc.)

### Integration Tests
- Noise + cellular automata produces valid caves
- Drunkard's walk + BSP produces connected dungeons
- All algorithms work with existing geometry types

### Benchmarks
- Noise generation (1000x1000 heightmap)
- Cellular automata (100x100 map, 10 iterations)
- Drunkard's walk (80x50 map, 50% floor coverage)

### Visual Tests
- Examples generate valid PNG output
- Maps are visually inspectable for correctness

## Documentation Requirements

### API Documentation
- All public functions have rustdoc comments
- Examples for complex APIs (noise configuration, CA parameters)
- Links to algorithm explanations (Wikipedia, RogueBasin)

### Examples
- Each algorithm has a standalone example
- Examples include comments explaining parameters
- Visual output (PNG or terminal) for inspection

### Algorithm Guides
- Brief explanation of each algorithm in crate README
- Parameter tuning guidelines
- Common use cases and patterns

## Performance Targets

Based on existing benchmarks:
- Noise generation: <10ms for 1000x1000 heightmap
- Cellular automata: <5ms for 100x100 map, 10 iterations
- Drunkard's walk: <2ms for 80x50 map, 50% coverage

## Migration Path

N/A — This is new functionality, not a breaking change.

## Risks and Mitigations

| Risk | Impact | Mitigation |
|------|--------|------------|
| `noise` crate API changes | High | Pin to specific version (0.9), document upgrade path |
| Performance issues with large maps | Medium | Benchmark early, optimize hot paths, document limits |
| Complex parameter tuning | Low | Provide sensible defaults, document parameter effects |
| Integration issues with existing crates | Medium | Write integration tests early, follow existing patterns |

## Timeline Estimate

- **Phase 1 (Noise):** 2-3 days
- **Phase 2 (Cellular Automata):** 2-3 days
- **Phase 3 (Drunkard's Walk):** 2-3 days
- **Phase 4 (Examples & Docs):** 2-3 days
- **Total:** 8-12 days

## References

- [RogueBasin: Cellular Automata](http://roguebasin.com/index.php?title=Cellular_Automata_Method_for_Generating_Random_Cave-Like_Levels)
- [RogueBasin: Drunkard's Walk](http://roguebasin.com/index.php?title=Random_Walk_Cave_Generation)
- [noise-rs Documentation](https://docs.rs/noise/)
- [Perlin Noise (Wikipedia)](https://en.wikipedia.org/wiki/Perlin_noise)
