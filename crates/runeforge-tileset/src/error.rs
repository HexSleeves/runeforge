//! Error types for tileset operations.

use thiserror::Error;

/// Errors that can occur when loading fonts and tilesets.
#[derive(Error, Debug)]
pub enum TilesetError {
    /// Failed to load or parse a TrueType font.
    #[error("Failed to load TrueType font: {0}")]
    TrueTypeFontError(String),

    /// Failed to load or parse a BDF bitmap font.
    #[error("Failed to load BDF font: {0}")]
    BdfFontError(String),

    /// Failed to load an image file.
    #[error("Failed to load image: {0}")]
    ImageError(#[from] image::ImageError),

    /// Invalid tileset dimensions.
    #[error("Invalid tileset dimensions: {0}")]
    InvalidDimensions(String),

    /// Character not found in font.
    #[error("Character '{0}' not found in font")]
    CharacterNotFound(char),

    /// I/O error.
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),
}

/// Result type alias for tileset operations.
pub type Result<T> = std::result::Result<T, TilesetError>;
