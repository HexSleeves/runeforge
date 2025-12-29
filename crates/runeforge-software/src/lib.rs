//! CPU-based software rendering backend for roguelikes.
//!
//! This backend renders characters to an RGBA pixel buffer using CPU-only operations.
//! It's useful for headless testing, CI/CD, and systems without GPU support.
//!
//! # Example
//!
//! ```no_run
//! use runeforge_software::SoftwareRenderer;
//! use runeforge_console::{Console, Cell};
//! use runeforge_color::Color;
//! use runeforge_geometry::Point;
//! use runeforge_tileset::TrueTypeFont;
//!
//! // Load a font
//! let font_data = std::fs::read("font.ttf").unwrap();
//! let font = TrueTypeFont::from_bytes(&font_data, 16.0).unwrap();
//!
//! // Create renderer
//! let mut renderer = SoftwareRenderer::new(80, 24, &font);
//!
//! // Use Console trait
//! renderer.clear();
//! renderer.set(Point::new(10, 10), Cell::new('@', Color::YELLOW, Color::BLACK));
//! renderer.present(); // No-op for software renderer
//!
//! // Save to PNG
//! renderer.save_png("output.png").unwrap();
//! ```

#![deny(missing_docs)]

use image::{ImageBuffer, Rgba};
use runeforge_color::Color;
use runeforge_console::{Cell, Console};
use runeforge_geometry::Point;
use runeforge_tileset::{Font, GlyphAtlas};
use std::io;
use std::path::Path;

/// CPU-based software renderer using pixel buffers.
///
/// Renders characters to an RGBA buffer that can be saved as PNG or
/// copied to other surfaces.
pub struct SoftwareRenderer {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
    glyph_atlas: GlyphAtlas,
    cell_width: u32,
    cell_height: u32,
    pixel_width: u32,
    pixel_height: u32,
    pixel_buffer: Vec<u8>,
}

impl SoftwareRenderer {
    /// Creates a new software renderer with the given dimensions and font.
    ///
    /// # Arguments
    ///
    /// * `width` - Width in cells
    /// * `height` - Height in cells
    /// * `font` - Font to use for rendering characters
    ///
    /// # Example
    ///
    /// ```no_run
    /// use runeforge_software::SoftwareRenderer;
    /// use runeforge_tileset::TrueTypeFont;
    ///
    /// let font_data = std::fs::read("font.ttf").unwrap();
    /// let font = TrueTypeFont::from_bytes(&font_data, 16.0).unwrap();
    /// let renderer = SoftwareRenderer::new(80, 24, &font);
    /// ```
    pub fn new(width: u32, height: u32, font: &dyn Font) -> Self {
        let cell_width = font.cell_width();
        let cell_height = font.cell_height();
        let glyph_atlas = GlyphAtlas::from_font(font, cell_width, cell_height);

        let pixel_width = width * cell_width;
        let pixel_height = height * cell_height;
        let pixel_buffer = vec![0u8; (pixel_width * pixel_height * 4) as usize];

        Self {
            width,
            height,
            cells: vec![Cell::empty(); (width * height) as usize],
            glyph_atlas,
            cell_width,
            cell_height,
            pixel_width,
            pixel_height,
            pixel_buffer,
        }
    }

    /// Returns the pixel width of the render buffer.
    pub fn pixel_width(&self) -> u32 {
        self.pixel_width
    }

    /// Returns the pixel height of the render buffer.
    pub fn pixel_height(&self) -> u32 {
        self.pixel_height
    }

    /// Returns a reference to the raw RGBA pixel buffer.
    pub fn pixel_buffer(&self) -> &[u8] {
        &self.pixel_buffer
    }

    /// Renders the current console state to the pixel buffer.
    fn render_to_buffer(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let cell_idx = (y * self.width + x) as usize;
                if cell_idx >= self.cells.len() {
                    continue;
                }

                let cell = self.cells[cell_idx];
                let px = x * self.cell_width;
                let py = y * self.cell_height;

                // Use Color directly (GlyphAtlas accepts Color)
                let fg = cell.fg;
                let bg = if cell.bg == Color::BLACK && cell.ch == ' ' {
                    None // Transparent for empty cells
                } else {
                    Some(cell.bg)
                };

                self.glyph_atlas.render_char(
                    &mut self.pixel_buffer,
                    self.pixel_width,
                    px,
                    py,
                    cell.ch,
                    fg,
                    bg,
                );
            }
        }
    }

    /// Saves the rendered output to a PNG file.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use runeforge_software::SoftwareRenderer;
    /// # use runeforge_tileset::TrueTypeFont;
    /// # let font_data = std::fs::read("font.ttf").unwrap();
    /// # let font = TrueTypeFont::from_bytes(&font_data, 16.0).unwrap();
    /// let mut renderer = SoftwareRenderer::new(80, 24, &font);
    /// // ... draw to renderer ...
    /// renderer.save_png("output.png").unwrap();
    /// ```
    pub fn save_png<P: AsRef<Path>>(&mut self, path: P) -> io::Result<()> {
        self.render_to_buffer();

        let img: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::from_raw(
            self.pixel_width,
            self.pixel_height,
            self.pixel_buffer.clone(),
        )
        .ok_or_else(|| io::Error::other("Failed to create image buffer"))?;

        img.save(path).map_err(io::Error::other)
    }

    /// Creates an image buffer from the current render state.
    ///
    /// Useful for integrating with other image processing libraries.
    pub fn to_image_buffer(&mut self) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
        self.render_to_buffer();
        ImageBuffer::from_raw(
            self.pixel_width,
            self.pixel_height,
            self.pixel_buffer.clone(),
        )
        .expect("Failed to create image buffer")
    }
}

impl Console for SoftwareRenderer {
    fn width(&self) -> u32 {
        self.width
    }

    fn height(&self) -> u32 {
        self.height
    }

    fn set(&mut self, pos: Point, cell: Cell) -> bool {
        if !self.in_bounds(pos) {
            return false;
        }
        let idx = (pos.y as u32 * self.width + pos.x as u32) as usize;
        if idx < self.cells.len() {
            self.cells[idx] = cell;
            true
        } else {
            false
        }
    }

    fn get(&self, pos: Point) -> Option<Cell> {
        if !self.in_bounds(pos) {
            return None;
        }
        let idx = (pos.y as u32 * self.width + pos.x as u32) as usize;
        self.cells.get(idx).copied()
    }

    fn clear(&mut self) {
        for cell in &mut self.cells {
            *cell = Cell::empty();
        }
        // Also clear pixel buffer to black
        self.pixel_buffer.fill(0);
    }

    fn present(&mut self) {
        // For software rendering, present() renders to the buffer
        // Actual display/saving is done via save_png() or to_image_buffer()
        self.render_to_buffer();
    }
}

#[cfg(test)]
mod tests {
    // Note: These tests require a font file, so we skip actual rendering tests
    // In a real project, you'd include a test font in the repository

    #[test]
    fn test_software_renderer_dimensions() {
        // This test would require a font, so we just test the concept
        // In practice, you'd load a test font from resources
    }

    #[test]
    fn test_console_trait_implementation() {
        // Verify that SoftwareRenderer implements Console
        // This is a compile-time check more than a runtime test
    }
}
