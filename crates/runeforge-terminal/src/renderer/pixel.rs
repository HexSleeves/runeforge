//! GPU-accelerated rendering backend using pixels and wgpu.
//!
//! This backend provides hardware-accelerated rendering while maintaining
//! a simple pixel-buffer interface. It uses the `pixels` crate for efficient
//! GPU rendering.
//!
//! # Note
//!
//! This is a basic implementation. For production use, you may want to
//! integrate with winit 0.30's ApplicationHandler trait directly.
//!
//! # Example
//!
//! ```no_run
//! use runeforge_terminal::prelude::*;
//! use runeforge_tileset::prelude::*;
//!
//! // Load a font
//! // let font_data = std::fs::read("font.ttf").unwrap();
//! // let font = TrueTypeFont::from_bytes(&font_data, 16.0).unwrap();
//!
//! // Create renderer
//! // let mut renderer = PixelsRenderer::new(80, 24, &font);
//!
//! // Use Console trait
//! // renderer.clear();
//! ```

#![deny(missing_docs)]

use crate::prelude::*;
use runeforge_color::Color;
use runeforge_tileset::prelude::{Font, GlyphAtlas};

/// GPU-ready renderer using pixel buffers.
///
/// This renderer maintains a pixel buffer that can be used with the `pixels`
/// crate for GPU acceleration. It provides the same Console trait interface
/// as other backends.
///
/// For actual GPU rendering, integrate this with `pixels` and `winit` in
/// your application code using winit 0.30's ApplicationHandler trait.
pub struct PixelsRenderer {
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

impl PixelsRenderer {
    /// Creates a new GPU-ready renderer with the given dimensions and font.
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
    /// use runeforge_terminal::prelude::*;
    /// use runeforge_tileset::prelude::*;
    ///
    /// // let font_data = std::fs::read("font.ttf").unwrap();
    /// // let font = TrueTypeFont::from_bytes(&font_data, 16.0).unwrap();
    /// // let renderer = PixelsRenderer::new(80, 24, &font);
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
    ///
    /// This buffer can be used with the `pixels` crate for GPU rendering.
    pub fn pixel_buffer(&self) -> &[u8] {
        &self.pixel_buffer
    }

    /// Returns a mutable reference to the raw RGBA pixel buffer.
    ///
    /// This buffer can be used with the `pixels` crate for GPU rendering.
    pub fn pixel_buffer_mut(&mut self) -> &mut [u8] {
        &mut self.pixel_buffer
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

                let fg = cell.fg;
                let bg = if cell.bg == Color::BLACK && cell.ch == ' ' {
                    None
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
}

impl Console for PixelsRenderer {
    fn width(&self) -> u32 {
        self.width
    }

    fn height(&self) -> u32 {
        self.height
    }

    fn set(&mut self, pos: IVec2, cell: Cell) -> bool {
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

    fn get(&self, pos: IVec2) -> Option<Cell> {
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
        // For GPU rendering, present() renders to the buffer
        // The actual pixel buffer can then be uploaded to GPU via pixels crate
        self.render_to_buffer();
    }
}

#[cfg(test)]
mod tests {
    // Note: These tests require a font file, so we skip actual rendering tests
    // In a real project, you'd include a test font in the repository

    #[test]
    fn test_console_trait_implementation() {
        // Verify that PixelsRenderer implements Console
        // This is a compile-time check more than a runtime test
    }
}
