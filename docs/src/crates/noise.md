# Noise

The `runeforge-noise` crate wraps noise generation libraries to provide procedural content generation tools.

## Features

- **Perlin Noise**: Smooth, continuous noise useful for terrain heightmaps.
- **Simplex Noise**: Improved version of Perlin noise.
- **Fractal Noise**: Combining multiple layers (octaves) of noise for detail.

## Usage

```rust
use runeforge_noise::{Perlin, NoiseFn};

let perlin = Perlin::new();
let val = perlin.get([x as f64 * 0.1, y as f64 * 0.1]);
```
