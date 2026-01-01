# Runeforge Random

The `runeforge-random` crate wraps `rand` to provide roguelike-specific utilities.

## Features

- **Rng Wrapper**: Easy access to a seeded random number generator.
- **Dice Rolling**: Support for parsing and rolling standard RPG dice notation (e.g., "3d6+2").
- **Weighted Selection**: Randomly choose items from a list with associated weights.

## Usage

```rust
use runeforge_random::Rng;

let mut rng = Rng::new(); // Random seed
// let mut rng = Rng::seeded(12345); // Fixed seed

// Basic random
let val: f32 = rng.random();

// Dice rolling
let damage = rng.roll_dice(3, 6); // 3d6
// let damage = rng.roll_str("3d6+2").unwrap(); 
```
