//! Font and tileset loading for roguelike games.
//!
//! This crate provides TrueType and bitmap font support, as well as
//! tileset/spritesheet loading for graphical roguelike rendering.
//!
//! # Features
//!
//! - **TrueType fonts**: Load `.ttf` and `.otf` fonts using `ab_glyph`
//! - **Bitmap fonts**: Load BDF bitmap fonts for pixel-perfect rendering
//! - **Tilesets**: Load sprite sheets from PNG images
//! - **Glyph atlas**: Pre-render characters to texture atlases for efficient rendering
//!
//! # Example
//!
//! ```no_run
//! use runeforge_tileset::{TrueTypeFont, Font, GlyphAtlas};
//!
//! // Load a TrueType font
//! let font_data = std::fs::read("assets/font.ttf").unwrap();
//! let font = TrueTypeFont::from_bytes(&font_data, 16.0).unwrap();
//!
//! // Create a glyph atlas for ASCII characters
//! let atlas = GlyphAtlas::from_font(&font, 16, 16);
//! ```

#![deny(missing_docs)]

mod atlas;
mod error;
mod font;
mod tileset;

pub use atlas::{Glyph, GlyphAtlas};
pub use error::TilesetError;
pub use font::{BitmapFont, Font, TrueTypeFont};
pub use tileset::{cp437_to_char, Tile, Tileset};
