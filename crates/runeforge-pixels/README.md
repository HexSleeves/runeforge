# runeforge-pixels

GPU-accelerated rendering backend for Runeforge using the `pixels` crate and `wgpu`.

## Features

- Hardware-accelerated pixel buffer rendering
- Backend-agnostic `Console` trait implementation
- Compatible with winit 0.30 for window management
- Efficient GPU texture uploads
- Low-latency frame rendering

## Usage

### Basic Setup

```rust
use runeforge_pixels::PixelsRenderer;
use runeforge_console::Console;
use runeforge_tileset::TrueTypeFont;

// Load a font
let font_data = std::fs::read("font.ttf")?;
let font = TrueTypeFont::from_bytes(&font_data, 16.0)?;

// Create renderer (80x25 console)
let mut renderer = PixelsRenderer::new(80, 25, &font);

// Use Console trait methods
renderer.clear();
renderer.draw_char(Point::new(10, 10), '@', Color::CYAN, Color::BLACK);
renderer.present();

// Get pixel buffer for GPU upload
let buffer = renderer.pixel_buffer();
```

### Windowed Application with Winit 0.30

The `windowed_roguelike` example demonstrates the complete integration pattern:

```bash
cargo run --example windowed_roguelike --package runeforge-pixels
```

**Key integration points:**

1. **Window and Pixels Lifetime Management**: Uses `Box::leak()` to create static references for the ApplicationHandler pattern:

```rust
struct Game {
    window: Option<&'static Window>,
    pixels: Option<Pixels<'static>>,
    renderer: PixelsRenderer,
}
```

1. **Window Creation**: In the `ApplicationHandler::resumed()` method:

```rust
fn resumed(&mut self, event_loop: &ActiveEventLoop) {
    let window = event_loop.create_window(attributes)?;

    // Convert to static lifetime - intentional leak for GUI apps
    let window_ref: &'static Window = Box::leak(Box::new(window));

    let surface = SurfaceTexture::new(width, height, window_ref);
    let pixels = Pixels::new(width, height, surface)?;

    self.window = Some(window_ref);
    self.pixels = Some(pixels);
}
```

1. **Frame Rendering**: Copy renderer buffer to pixels frame:

```rust
fn update_pixels(&mut self) {
    if let (Some(window), Some(pixels)) = (&self.window, &mut self.pixels) {
        // Copy runeforge buffer to pixels frame
        pixels.frame_mut().copy_from_slice(self.renderer.pixel_buffer());

        // Render to GPU
        pixels.render()?;
        window.request_redraw();
    }
}
```

## Cargo.toml Configuration

For winit 0.30 compatibility, enable the `rwh_06` feature:

```toml
[dependencies]
pixels = "0.15.0"
winit = { version = "0.30", features = ["rwh_06"] }
runeforge-pixels = "0.1.0"
runeforge-console = "0.1.0"
runeforge-tileset = "0.1.0"
```

## Architecture

The `PixelsRenderer` maintains an internal RGBA pixel buffer that's rendered using the `GlyphAtlas` system. This buffer can be efficiently uploaded to the GPU via the `pixels` crate.

**Rendering Pipeline:**

1. Draw to console cells using `Console` trait methods
2. Call `present()` to render cells to pixel buffer
3. Copy pixel buffer to `pixels` frame
4. `pixels` uploads to GPU and renders to window

## Examples

### windowed_roguelike (TrueType font)

Complete roguelike using TrueType fonts for rendering:

```bash
cargo run --example windowed_roguelike --package runeforge-pixels
```

- Arrow keys to move, ESC to exit
- Uses TrueType font for crisp text rendering
- Demonstrates full winit 0.30 + pixels integration

### windowed_tileset_roguelike (PNG tileset)

Classic roguelike using CP437 tileset (sprite sheet):

```bash
cargo run --example windowed_tileset_roguelike --package runeforge-pixels
```

- Arrow keys to move, ESC to exit
- Uses CP437 PNG tileset (16x16 grid of 8x8 pixel tiles)
- Perfect for retro/classic roguelike aesthetics
- Demonstrates tileset Font trait implementation

## Performance

GPU-accelerated rendering provides:

- 60+ FPS for typical roguelike scenes
- Minimal CPU usage for rendering
- Instant frame updates with no tearing

## Comparison with Other Backends

| Backend | Use Case | Performance | Dependencies |
| ------- | -------- | ----------- | ------------ |
| **pixels** | Windowed games | Excellent | winit, wgpu |
| **software** | Testing, CI/CD, screenshots | Good | None (CPU only) |
| **terminal** | SSH, console apps | N/A | None |

## Notes

- The `Box::leak()` pattern for window lifetime is intentional and standard for winit 0.30 ApplicationHandler implementations
- For GUI applications that run until exit, the memory leak is negligible and simplifies lifetime management
- See [Building Conway's Game of Life in Rust](https://www.40tude.fr/docs/06_programmation/rust/017_game_of_life/game_of_life_00.html) for more details on this pattern

## License

BSD-3-Clause (see LICENSE in repository root)
