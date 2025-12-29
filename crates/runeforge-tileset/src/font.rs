//! Font loading and rendering.

use crate::error::{Result, TilesetError};

/// A rendered glyph with its bitmap data.
#[derive(Debug, Clone)]
pub struct RenderedGlyph {
    /// The character this glyph represents.
    pub character: char,
    /// Width of the glyph in pixels.
    pub width: u32,
    /// Height of the glyph in pixels.
    pub height: u32,
    /// Horizontal offset from the origin.
    pub bearing_x: i32,
    /// Vertical offset from the baseline.
    pub bearing_y: i32,
    /// Grayscale bitmap data (0-255 alpha values).
    pub bitmap: Vec<u8>,
}

/// Trait for font implementations.
pub trait Font {
    /// Returns the name of the font.
    fn name(&self) -> &str;

    /// Returns the recommended cell width for this font.
    fn cell_width(&self) -> u32;

    /// Returns the recommended cell height for this font.
    fn cell_height(&self) -> u32;

    /// Returns the line height (distance between baselines).
    fn line_height(&self) -> u32;

    /// Renders a character to a grayscale bitmap.
    fn render_glyph(&self, c: char) -> Option<RenderedGlyph>;

    /// Checks if the font contains a glyph for the given character.
    fn has_glyph(&self, c: char) -> bool;
}

/// TrueType font loaded from .ttf or .otf files.
#[cfg(feature = "truetype")]
pub struct TrueTypeFont {
    font: ab_glyph::FontVec,
    scale: ab_glyph::PxScale,
    name: String,
    cell_width: u32,
    cell_height: u32,
    line_height: u32,
}

#[cfg(feature = "truetype")]
impl TrueTypeFont {
    /// Loads a TrueType font from bytes.
    ///
    /// # Arguments
    ///
    /// * `data` - The font file data
    /// * `size` - The font size in pixels
    ///
    /// # Example
    ///
    /// ```no_run
    /// use runeforge_tileset::TrueTypeFont;
    ///
    /// let font_data = std::fs::read("font.ttf").unwrap();
    /// let font = TrueTypeFont::from_bytes(&font_data, 16.0).unwrap();
    /// ```
    pub fn from_bytes(data: &[u8], size: f32) -> Result<Self> {
        use ab_glyph::{Font as AbFont, FontVec, PxScale, ScaleFont};

        let font = FontVec::try_from_vec(data.to_vec())
            .map_err(|e| TilesetError::TrueTypeFontError(e.to_string()))?;

        let scale = PxScale::from(size);
        let scaled_font = font.as_scaled(scale);

        // Calculate metrics based on 'M' character (em-square)
        let m_glyph = font.glyph_id('M');
        let cell_width = scaled_font.h_advance(m_glyph).ceil() as u32;

        let line_height = scaled_font.height().ceil() as u32;
        let cell_height = line_height;

        Ok(Self {
            font,
            scale,
            name: "TrueType Font".to_string(),
            cell_width,
            cell_height,
            line_height,
        })
    }

    /// Loads a TrueType font from a file path.
    pub fn from_file(path: &str, size: f32) -> Result<Self> {
        let data = std::fs::read(path)?;
        Self::from_bytes(&data, size)
    }
}

#[cfg(feature = "truetype")]
impl Font for TrueTypeFont {
    fn name(&self) -> &str {
        &self.name
    }

    fn cell_width(&self) -> u32 {
        self.cell_width
    }

    fn cell_height(&self) -> u32 {
        self.cell_height
    }

    fn line_height(&self) -> u32 {
        self.line_height
    }

    fn render_glyph(&self, c: char) -> Option<RenderedGlyph> {
        use ab_glyph::{Font as AbFont, ScaleFont};

        let scaled_font = self.font.as_scaled(self.scale);
        let glyph_id = self.font.glyph_id(c);

        // Check if glyph exists
        if glyph_id.0 == 0 && c != '\0' {
            return None;
        }

        let glyph = glyph_id.with_scale_and_position(self.scale, ab_glyph::point(0.0, 0.0));

        if let Some(outlined) = self.font.outline_glyph(glyph) {
            let bounds = outlined.px_bounds();
            let width = bounds.width().ceil() as u32;
            let height = bounds.height().ceil() as u32;

            if width == 0 || height == 0 {
                // Space or empty glyph
                return Some(RenderedGlyph {
                    character: c,
                    width: self.cell_width,
                    height: self.cell_height,
                    bearing_x: 0,
                    bearing_y: 0,
                    bitmap: vec![0; (self.cell_width * self.cell_height) as usize],
                });
            }

            let mut bitmap = vec![0u8; (width * height) as usize];

            outlined.draw(|x, y, coverage| {
                let idx = (y * width + x) as usize;
                if idx < bitmap.len() {
                    bitmap[idx] = (coverage * 255.0) as u8;
                }
            });

            Some(RenderedGlyph {
                character: c,
                width,
                height,
                bearing_x: bounds.min.x as i32,
                bearing_y: (scaled_font.ascent() - bounds.min.y) as i32,
                bitmap,
            })
        } else {
            // Space or glyph without outline
            Some(RenderedGlyph {
                character: c,
                width: self.cell_width,
                height: self.cell_height,
                bearing_x: 0,
                bearing_y: 0,
                bitmap: vec![0; (self.cell_width * self.cell_height) as usize],
            })
        }
    }

    fn has_glyph(&self, c: char) -> bool {
        use ab_glyph::Font as AbFont;
        let glyph_id = self.font.glyph_id(c);
        glyph_id.0 != 0 || c == '\0'
    }
}

/// BDF bitmap font loaded from .bdf files.
#[cfg(feature = "bitmap")]
pub struct BitmapFont {
    glyphs: std::collections::HashMap<char, BdfGlyph>,
    name: String,
    cell_width: u32,
    cell_height: u32,
}

#[cfg(feature = "bitmap")]
struct BdfGlyph {
    width: u32,
    height: u32,
    bearing_x: i32,
    bearing_y: i32,
    bitmap: Vec<u8>,
}

#[cfg(feature = "bitmap")]
impl BitmapFont {
    /// Loads a BDF bitmap font from bytes.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use runeforge_tileset::BitmapFont;
    ///
    /// let font_data = std::fs::read("font.bdf").unwrap();
    /// let font = BitmapFont::from_bytes(&font_data).unwrap();
    /// ```
    pub fn from_bytes(data: &[u8]) -> Result<Self> {
        use bdf_parser::BdfFont;

        let bdf =
            BdfFont::parse(data).map_err(|e| TilesetError::BdfFontError(format!("{:?}", e)))?;

        let mut glyphs = std::collections::HashMap::new();
        let mut max_width = 0u32;
        let mut max_height = 0u32;

        for glyph in bdf.glyphs.iter() {
            if let Some(encoding) = glyph.encoding {
                if let Some(c) = char::from_u32(encoding as u32) {
                    let bounds = glyph.bounding_box;
                    let width = bounds.size.x as u32;
                    let height = bounds.size.y as u32;

                    max_width = max_width.max(width);
                    max_height = max_height.max(height);

                    // Convert bitmap from BDF format (1-bit packed) to grayscale
                    let mut bitmap = vec![0u8; (width * height) as usize];

                    if !glyph.bitmap.is_empty() {
                        let bytes_per_row = width.div_ceil(8) as usize; // Round up for partial bytes
                        for y in 0..height as usize {
                            for x in 0..width as usize {
                                let byte_idx = y * bytes_per_row + (x / 8);
                                let bit_idx = 7 - (x % 8);
                                if byte_idx < glyph.bitmap.len() {
                                    let bit = (glyph.bitmap[byte_idx] >> bit_idx) & 1;
                                    if bit == 1 {
                                        let idx = y * width as usize + x;
                                        if idx < bitmap.len() {
                                            bitmap[idx] = 255;
                                        }
                                    }
                                }
                            }
                        }
                    }

                    glyphs.insert(
                        c,
                        BdfGlyph {
                            width,
                            height,
                            bearing_x: bounds.offset.x,
                            bearing_y: bounds.offset.y + height as i32,
                            bitmap,
                        },
                    );
                }
            }
        }

        let name = bdf.metadata.name;

        Ok(Self {
            glyphs,
            name,
            cell_width: max_width.max(8),
            cell_height: max_height.max(8),
        })
    }

    /// Loads a BDF bitmap font from a file path.
    pub fn from_file(path: &str) -> Result<Self> {
        let data = std::fs::read(path)?;
        Self::from_bytes(&data)
    }
}

#[cfg(feature = "bitmap")]
impl Font for BitmapFont {
    fn name(&self) -> &str {
        &self.name
    }

    fn cell_width(&self) -> u32 {
        self.cell_width
    }

    fn cell_height(&self) -> u32 {
        self.cell_height
    }

    fn line_height(&self) -> u32 {
        self.cell_height
    }

    fn render_glyph(&self, c: char) -> Option<RenderedGlyph> {
        self.glyphs.get(&c).map(|g| RenderedGlyph {
            character: c,
            width: g.width,
            height: g.height,
            bearing_x: g.bearing_x,
            bearing_y: g.bearing_y,
            bitmap: g.bitmap.clone(),
        })
    }

    fn has_glyph(&self, c: char) -> bool {
        self.glyphs.contains_key(&c)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    #[cfg(feature = "truetype")]
    fn test_truetype_font_metrics() {
        // This test requires a font file, so we just verify the API compiles
        // In a real test, you'd load an actual font file
    }

    #[test]
    #[cfg(feature = "bitmap")]
    fn test_bitmap_font_placeholder() {
        // Placeholder test - real tests would require BDF font files
    }
}
