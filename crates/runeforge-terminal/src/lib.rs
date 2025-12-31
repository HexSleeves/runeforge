//! Terminal rendering backend for roguelike games.
//!
//! # Overview
//!
//! `runeforge-terminal` provides a simple, ANSI-based terminal renderer. It allows you to draw
//! colored characters to a virtual console and then present them to the standard output.
//! This is ideal for:
//!
//! *   **Prototyping:** Quickly testing algorithms without setting up a window.
//! *   **CLI Games:** creating pure terminal roguelikes.
//! *   **Debugging:** Visualizing maps or FOV directly in the console.
//!
//! # Key Components
//!
//! *   **`Terminal`**: The main rendering context. Handles grid storage and output.
//! *   **`Cell`**: A single character with foreground and background colors.
//!
//! # Usage
//!
//! Add this to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! runeforge-terminal = "0.1"
//! ```
//!
//! ## Basic Example
//!
//! ```no_run
//! use runeforge_terminal::prelude::*;
//! use runeforge_color::Color;
//! use runeforge_geometry::prelude::IVec2;
//!
//! fn main() -> std::io::Result<()> {
//!     // 1. Create a terminal (80x24)
//!     let mut term = Terminal::new(80, 24);
//!
//!     // 2. Clear screen
//!     term.clear();
//!
//!     // 3. Draw something
//!     term.put_char(IVec2::new(10, 10), '@', Color::YELLOW, Color::BLACK);
//!     term.put_string(IVec2::new(12, 10), "Player", Color::WHITE, Color::BLACK);
//!
//!     // 4. Render to stdout
//!     term.present()?;
//!     
//!     Ok(())
//! }
//! ```

pub mod cell;
pub mod console;
pub mod renderer;
pub mod terminal;

pub mod prelude {
    pub use runeforge_geometry::prelude::IVec2;

    pub use crate::cell::*;
    pub use crate::console::*;
    pub use crate::renderer::*;
    pub use crate::terminal::*;
}
