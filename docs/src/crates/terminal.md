# Terminal

The `runeforge-terminal` crate is the primary rendering interface. It provides a virtual console abstraction.

## The Console

The `Console` struct represents a grid of cells. Each cell has:
- A character glyph
- A foreground color
- A background color

## Backends

Runeforge supports multiple backends:
- **WGPU**: Hardware accelerated (default).
- **Software**: CPU-based rendering.
- **Terminal**: Direct output to standard output (ANSI).

## Usage

```rust
let mut console = Console::new(80, 50);

console.print(5, 5, "Hello World", Color::WHITE, Color::BLACK);
console.set(10, 10, '@', Color::YELLOW, Color::BLACK);
```
