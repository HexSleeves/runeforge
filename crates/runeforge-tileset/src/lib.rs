//! Font and tileset loading for graphical roguelikes.
//!
//! # Overview
//!
//! `runeforge-tileset` handles loading and processing of visual assets for rendering.
//! It supports two main types of assets:
//!
//! 1.  **Tilesets:** Grids of sprites (usually PNG) used for map tiles, characters, and UI elements.
//! 2.  **Fonts:** TrueType (`.ttf`, `.otf`) and Bitmap (`.bdf`) fonts for text rendering.
//!
//! # Features
//!
//! *   **Format Support:**
//!     *   **Images:** PNG, JPEG, etc. (via `image` crate).
//!     *   **Fonts:** TrueType/OpenType (via `ab_glyph`) and BDF Bitmap fonts (via `bdf-parser`).
//! *   **CP437 Mapping:** Built-in utilities to map characters to standard roguelike tilesets (e.g., Dwarf Fortress style).
//! *   **Atlas Generation:** Helper to create texture atlases from fonts.
//!
//! # Usage
//!
//! Add this to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! runeforge-tileset = { version = "0.1", features = ["truetype", "bitmap"] }
//! ```
//!
//! ## Tileset Example
//!
//! ```no_run
//! use runeforge_tileset::prelude::*;
//!
//! fn main() {
//!     // Load a 16x16 tileset from a PNG file
//!     // (Requires a valid file at "assets/tileset.png")
//!     let tileset = Tileset::from_file("assets/tileset.png", 16, 16).unwrap();
//!
//!     println!("Loaded {} tiles", tileset.len());
//!     
//!     // Access a specific tile (e.g., '@' is index 64 in standard sets)
//!     if let Some(tile) = tileset.get_tile(64) {
//!         println!("Tile 64 is {}x{} pixels", tile.width, tile.height);
//!     }
//! }
//! ```
//!
//! ## Font Example
//!
//! ```no_run
//! use runeforge_tileset::prelude::*;
//!
//! fn main() {
//!     #[cfg(feature = "truetype")]
//!     {
//!         let font_data = std::fs::read("assets/font.ttf").unwrap();
//!         let font = TrueTypeFont::from_bytes(&font_data, 16.0).unwrap();
//!         
//!         if font.has_glyph('A') {
//!             let glyph = font.render_glyph('A').unwrap();
//!             println!("Rendered 'A': {}x{}", glyph.width, glyph.height);
//!         }
//!     }
//! }
//! ```

pub mod atlas;
pub mod error;
pub mod font;
pub mod tileset;

pub mod prelude {
    pub use crate::atlas::*;
    pub use crate::error::*;
    pub use crate::font::*;
    pub use crate::tileset::*;
}
