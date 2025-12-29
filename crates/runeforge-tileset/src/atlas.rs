//! Glyph atlas for efficient text rendering.

use crate::font::Font;
use runeforge_color::Color;

/// A single glyph with its position in the atlas.
#[derive(Debug, Clone)]
pub struct Glyph {
    /// The character this glyph represents.
    pub character: char,
    /// X position in the atlas texture.
    pub atlas_x: u32,
    /// Y position in the atlas texture.
    pub atlas_y: u32,
    /// Width of the glyph.
    pub width: u32,
    /// Height of the glyph.
    pub height: u32,
    /// Horizontal bearing.
    pub bearing_x: i32,
    /// Vertical bearing.
    pub bearing_y: i32,
}

/// A texture atlas containing pre-rendered glyphs.
///
/// The atlas stores glyphs in a grid layout for efficient GPU texture sampling.
/// Each cell has fixed dimensions, and glyphs are centered within their cells.
#[derive(Debug)]
pub struct GlyphAtlas {
    /// The atlas texture data in RGBA format.
    pub texture: Vec<u8>,
    /// Width of the atlas texture in pixels.
    pub texture_width: u32,
    /// Height of the atlas texture in pixels.
    pub texture_height: u32,
    /// Width of each cell in the grid.
    pub cell_width: u32,
    /// Height of each cell in the grid.
    pub cell_height: u32,
    /// Number of columns in the atlas grid.
    pub columns: u32,
    /// Number of rows in the atlas grid.
    pub rows: u32,
    /// Mapping from character to glyph info.
    glyphs: std::collections::HashMap<char, Glyph>,
}

impl GlyphAtlas {
    /// Creates a new glyph atlas from a font.
    ///
    /// By default, includes ASCII printable characters (32-126) plus some
    /// common box-drawing characters used in roguelikes.
    ///
    /// # Arguments
    ///
    /// * `font` - The font to render glyphs from
    /// * `cell_width` - Width of each cell in the atlas
    /// * `cell_height` - Height of each cell in the atlas
    ///
    /// # Example
    ///
    /// ```no_run
    /// use runeforge_tileset::{TrueTypeFont, Font, GlyphAtlas};
    ///
    /// let font_data = std::fs::read("font.ttf").unwrap();
    /// let font = TrueTypeFont::from_bytes(&font_data, 16.0).unwrap();
    /// let atlas = GlyphAtlas::from_font(&font, 16, 16);
    /// ```
    pub fn from_font(font: &dyn Font, cell_width: u32, cell_height: u32) -> Self {
        Self::from_font_with_chars(font, cell_width, cell_height, Self::default_charset())
    }

    /// Creates a glyph atlas with a custom character set.
    pub fn from_font_with_chars(
        font: &dyn Font,
        cell_width: u32,
        cell_height: u32,
        chars: impl IntoIterator<Item = char>,
    ) -> Self {
        let chars: Vec<char> = chars.into_iter().collect();
        let char_count = chars.len() as u32;

        // Calculate atlas dimensions (try to make it roughly square)
        let columns = (char_count as f32).sqrt().ceil() as u32;
        let rows = char_count.div_ceil(columns);

        let texture_width = columns * cell_width;
        let texture_height = rows * cell_height;

        // Create RGBA texture (initialized to transparent black)
        let mut texture = vec![0u8; (texture_width * texture_height * 4) as usize];
        let mut glyphs = std::collections::HashMap::new();

        for (idx, c) in chars.iter().enumerate() {
            let col = idx as u32 % columns;
            let row = idx as u32 / columns;
            let atlas_x = col * cell_width;
            let atlas_y = row * cell_height;

            if let Some(rendered) = font.render_glyph(*c) {
                // Calculate centering offset
                let offset_x = (cell_width.saturating_sub(rendered.width)) / 2;
                let offset_y = (cell_height.saturating_sub(rendered.height)) / 2;

                // Copy glyph bitmap to atlas (convert grayscale to RGBA white)
                for gy in 0..rendered.height {
                    for gx in 0..rendered.width {
                        let src_idx = (gy * rendered.width + gx) as usize;
                        if src_idx < rendered.bitmap.len() {
                            let alpha = rendered.bitmap[src_idx];
                            if alpha > 0 {
                                let tx = atlas_x + offset_x + gx;
                                let ty = atlas_y + offset_y + gy;
                                if tx < texture_width && ty < texture_height {
                                    let dst_idx = ((ty * texture_width + tx) * 4) as usize;
                                    // White color with alpha from glyph
                                    texture[dst_idx] = 255; // R
                                    texture[dst_idx + 1] = 255; // G
                                    texture[dst_idx + 2] = 255; // B
                                    texture[dst_idx + 3] = alpha; // A
                                }
                            }
                        }
                    }
                }

                glyphs.insert(
                    *c,
                    Glyph {
                        character: *c,
                        atlas_x,
                        atlas_y,
                        width: rendered.width,
                        height: rendered.height,
                        bearing_x: rendered.bearing_x,
                        bearing_y: rendered.bearing_y,
                    },
                );
            }
        }

        Self {
            texture,
            texture_width,
            texture_height,
            cell_width,
            cell_height,
            columns,
            rows,
            glyphs,
        }
    }

    /// Returns the default character set for roguelikes.
    ///
    /// Includes ASCII printable characters and common box-drawing characters.
    pub fn default_charset() -> Vec<char> {
        let mut chars = Vec::new();

        // ASCII printable characters (32-126)
        for c in 32u8..=126u8 {
            chars.push(c as char);
        }

        // Box-drawing characters (commonly used in roguelikes)
        // Single line box drawing
        chars.extend(['─', '│', '┌', '┐', '└', '┘', '├', '┤', '┬', '┴', '┼']);

        // Double line box drawing
        chars.extend(['═', '║', '╔', '╗', '╚', '╝', '╠', '╣', '╦', '╩', '╬']);

        // Block elements
        chars.extend(['░', '▒', '▓', '█', '▄', '▀', '▌', '▐']);

        // Common roguelike symbols
        chars.extend([
            '●', '○', '◆', '◇', '★', '☆', '♠', '♣', '♥', '♦', '←', '→', '↑', '↓', '↔', '↕',
        ]);

        chars
    }

    /// Gets glyph information for a character.
    pub fn get_glyph(&self, c: char) -> Option<&Glyph> {
        self.glyphs.get(&c)
    }

    /// Gets the UV coordinates for a character (normalized 0.0-1.0).
    ///
    /// Returns (u_min, v_min, u_max, v_max).
    pub fn get_uv(&self, c: char) -> Option<(f32, f32, f32, f32)> {
        self.glyphs.get(&c).map(|g| {
            let u_min = g.atlas_x as f32 / self.texture_width as f32;
            let v_min = g.atlas_y as f32 / self.texture_height as f32;
            let u_max = (g.atlas_x + self.cell_width) as f32 / self.texture_width as f32;
            let v_max = (g.atlas_y + self.cell_height) as f32 / self.texture_height as f32;
            (u_min, v_min, u_max, v_max)
        })
    }

    /// Renders a character directly to an RGBA buffer at the specified position.
    ///
    /// # Arguments
    ///
    /// * `buffer` - The target RGBA buffer
    /// * `buffer_width` - Width of the target buffer
    /// * `x` - X position in the buffer
    /// * `y` - Y position in the buffer
    /// * `c` - Character to render
    /// * `fg` - Foreground color
    /// * `bg` - Background color (None for transparent)
    #[allow(clippy::too_many_arguments)]
    pub fn render_char(
        &self,
        buffer: &mut [u8],
        buffer_width: u32,
        x: u32,
        y: u32,
        c: char,
        fg: Color,
        bg: Option<Color>,
    ) {
        // Fill background if specified
        if let Some(bg_color) = bg {
            for cy in 0..self.cell_height {
                for cx in 0..self.cell_width {
                    let bx = x + cx;
                    let by = y + cy;
                    let idx = ((by * buffer_width + bx) * 4) as usize;
                    if idx + 3 < buffer.len() {
                        buffer[idx] = bg_color.r;
                        buffer[idx + 1] = bg_color.g;
                        buffer[idx + 2] = bg_color.b;
                        buffer[idx + 3] = 255;
                    }
                }
            }
        }

        // Render character glyph
        if let Some(glyph) = self.glyphs.get(&c) {
            for cy in 0..self.cell_height {
                for cx in 0..self.cell_width {
                    let atlas_idx =
                        (((glyph.atlas_y + cy) * self.texture_width + glyph.atlas_x + cx) * 4)
                            as usize;
                    if atlas_idx + 3 >= self.texture.len() {
                        continue;
                    }

                    let alpha = self.texture[atlas_idx + 3];
                    if alpha > 0 {
                        let bx = x + cx;
                        let by = y + cy;
                        let buf_idx = ((by * buffer_width + bx) * 4) as usize;
                        if buf_idx + 3 < buffer.len() {
                            // Alpha blend foreground color
                            let alpha_f = alpha as f32 / 255.0;
                            let inv_alpha = 1.0 - alpha_f;

                            buffer[buf_idx] =
                                (fg.r as f32 * alpha_f + buffer[buf_idx] as f32 * inv_alpha) as u8;
                            buffer[buf_idx + 1] = (fg.g as f32 * alpha_f
                                + buffer[buf_idx + 1] as f32 * inv_alpha)
                                as u8;
                            buffer[buf_idx + 2] = (fg.b as f32 * alpha_f
                                + buffer[buf_idx + 2] as f32 * inv_alpha)
                                as u8;
                            buffer[buf_idx + 3] = 255;
                        }
                    }
                }
            }
        }
    }

    /// Returns the number of characters in the atlas.
    pub fn len(&self) -> usize {
        self.glyphs.len()
    }

    /// Returns true if the atlas is empty.
    pub fn is_empty(&self) -> bool {
        self.glyphs.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_charset() {
        let chars = GlyphAtlas::default_charset();
        // Should include ASCII printable (95) + box drawing + block elements + symbols
        assert!(chars.len() >= 95);
        // Check some expected characters
        assert!(chars.contains(&'A'));
        assert!(chars.contains(&'@'));
        assert!(chars.contains(&'#'));
        assert!(chars.contains(&' '));
    }
}
