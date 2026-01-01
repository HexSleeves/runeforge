# Runeforge Color

The `runeforge-color` crate provides tools for color manipulation.

## Features

- **RGB & HSV**: Support for both Red-Green-Blue and Hue-Saturation-Value color models.
- **Blending**: Utilities to blend colors (lerp, multiply, add).
- **Constants**: Predefined named colors (e.g., `Color::RED`, `Color::AZURE`).
- **Palette Integration**: Optional integration with the `palette` crate for advanced color science.

## Usage

```rust
use runeforge_color::Color;

let red = Color::new(255, 0, 0);
let blue = Color::BLUE;

// Linear interpolation (blend)
let purple = red.lerp(blue, 0.5);
```
