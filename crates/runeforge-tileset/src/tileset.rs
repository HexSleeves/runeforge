//! Tileset/spritesheet loading for roguelike games.
//!
//! Supports loading PNG tilesets (sprite sheets) commonly used in graphical roguelikes.

use crate::error::{Result, TilesetError};
use image::{GenericImageView, ImageReader};
use runeforge_color::Color;

/// A single tile extracted from a tileset.
#[derive(Debug, Clone)]
pub struct Tile {
    /// Tile index in the tileset (row-major order).
    pub index: u32,
    /// Width of the tile in pixels.
    pub width: u32,
    /// Height of the tile in pixels.
    pub height: u32,
    /// X position in the source tileset texture.
    pub src_x: u32,
    /// Y position in the source tileset texture.
    pub src_y: u32,
    /// RGBA pixel data for this tile.
    pub pixels: Vec<u8>,
}

impl Tile {
    /// Gets the pixel at (x, y) as RGBA.
    ///
    /// Returns None if coordinates are out of bounds.
    pub fn get_pixel(&self, x: u32, y: u32) -> Option<[u8; 4]> {
        if x >= self.width || y >= self.height {
            return None;
        }
        let idx = ((y * self.width + x) * 4) as usize;
        if idx + 3 < self.pixels.len() {
            Some([
                self.pixels[idx],
                self.pixels[idx + 1],
                self.pixels[idx + 2],
                self.pixels[idx + 3],
            ])
        } else {
            None
        }
    }

    /// Gets the pixel at (x, y) as a Color.
    pub fn get_color(&self, x: u32, y: u32) -> Option<Color> {
        self.get_pixel(x, y)
            .map(|[r, g, b, a]| Color::rgba(r, g, b, a))
    }
}

/// A tileset loaded from an image file.
///
/// Tilesets are sprite sheets containing a grid of fixed-size tiles.
/// Common in roguelikes for graphical tile-based rendering (e.g., Dwarf Fortress tilesets).
///
/// # Example
///
/// ```no_run
/// use runeforge_tileset::Tileset;
///
/// // Load a 16x16 tileset from a PNG
/// let tileset = Tileset::from_file("cp437_16x16.png", 16, 16).unwrap();
///
/// // Access individual tiles
/// let at_sign = tileset.get_tile(64); // '@' in CP437
/// ```
#[derive(Debug, Clone)]
pub struct Tileset {
    /// All tiles extracted from the tileset image.
    tiles: Vec<Tile>,
    /// Width of each tile in pixels.
    pub tile_width: u32,
    /// Height of each tile in pixels.
    pub tile_height: u32,
    /// Number of tile columns in the tileset.
    pub columns: u32,
    /// Number of tile rows in the tileset.
    pub rows: u32,
    /// Full texture data in RGBA format.
    pub texture: Vec<u8>,
    /// Width of the full texture.
    pub texture_width: u32,
    /// Height of the full texture.
    pub texture_height: u32,
}

impl Tileset {
    /// Loads a tileset from an image file.
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the image file (PNG, JPEG, etc.)
    /// * `tile_width` - Width of each tile in pixels
    /// * `tile_height` - Height of each tile in pixels
    ///
    /// # Example
    ///
    /// ```no_run
    /// use runeforge_tileset::Tileset;
    ///
    /// let tileset = Tileset::from_file("tiles.png", 16, 16).unwrap();
    /// println!("Loaded {} tiles", tileset.len());
    /// ```
    pub fn from_file(path: &str, tile_width: u32, tile_height: u32) -> Result<Self> {
        let img = ImageReader::open(path)?.decode()?;
        Self::from_image(img, tile_width, tile_height)
    }

    /// Loads a tileset from raw image bytes.
    ///
    /// # Arguments
    ///
    /// * `data` - Raw image file data (PNG, JPEG, etc.)
    /// * `tile_width` - Width of each tile in pixels
    /// * `tile_height` - Height of each tile in pixels
    pub fn from_bytes(data: &[u8], tile_width: u32, tile_height: u32) -> Result<Self> {
        let img = image::load_from_memory(data)?;
        Self::from_image(img, tile_width, tile_height)
    }

    /// Loads a tileset from a DynamicImage.
    fn from_image(img: image::DynamicImage, tile_width: u32, tile_height: u32) -> Result<Self> {
        let (img_width, img_height) = img.dimensions();

        if tile_width == 0 || tile_height == 0 {
            return Err(TilesetError::InvalidDimensions(
                "Tile dimensions must be non-zero".to_string(),
            ));
        }

        if img_width < tile_width || img_height < tile_height {
            return Err(TilesetError::InvalidDimensions(format!(
                "Image {}x{} is smaller than tile size {}x{}",
                img_width, img_height, tile_width, tile_height
            )));
        }

        let columns = img_width / tile_width;
        let rows = img_height / tile_height;

        if columns == 0 || rows == 0 {
            return Err(TilesetError::InvalidDimensions(
                "Image too small for given tile dimensions".to_string(),
            ));
        }

        // Convert to RGBA8
        let rgba_img = img.to_rgba8();
        let texture = rgba_img.to_vec();

        // Extract individual tiles
        let mut tiles = Vec::with_capacity((columns * rows) as usize);

        for row in 0..rows {
            for col in 0..columns {
                let index = row * columns + col;
                let src_x = col * tile_width;
                let src_y = row * tile_height;

                // Extract tile pixels
                let mut pixels = vec![0u8; (tile_width * tile_height * 4) as usize];
                for ty in 0..tile_height {
                    for tx in 0..tile_width {
                        let src_idx = (((src_y + ty) * img_width + src_x + tx) * 4) as usize;
                        let dst_idx = ((ty * tile_width + tx) * 4) as usize;
                        if src_idx + 3 < texture.len() && dst_idx + 3 < pixels.len() {
                            pixels[dst_idx] = texture[src_idx];
                            pixels[dst_idx + 1] = texture[src_idx + 1];
                            pixels[dst_idx + 2] = texture[src_idx + 2];
                            pixels[dst_idx + 3] = texture[src_idx + 3];
                        }
                    }
                }

                tiles.push(Tile {
                    index,
                    width: tile_width,
                    height: tile_height,
                    src_x,
                    src_y,
                    pixels,
                });
            }
        }

        Ok(Self {
            tiles,
            tile_width,
            tile_height,
            columns,
            rows,
            texture,
            texture_width: img_width,
            texture_height: img_height,
        })
    }

    /// Gets a tile by its index (row-major order).
    ///
    /// Index 0 is top-left, increases left-to-right then top-to-bottom.
    pub fn get_tile(&self, index: u32) -> Option<&Tile> {
        self.tiles.get(index as usize)
    }

    /// Gets a tile by its grid position.
    ///
    /// # Arguments
    ///
    /// * `col` - Column (0-indexed from left)
    /// * `row` - Row (0-indexed from top)
    pub fn get_tile_at(&self, col: u32, row: u32) -> Option<&Tile> {
        if col >= self.columns || row >= self.rows {
            return None;
        }
        let index = row * self.columns + col;
        self.tiles.get(index as usize)
    }

    /// Gets a tile for a CP437 character.
    ///
    /// This assumes a standard CP437 tileset layout (16x16 grid, 256 tiles).
    /// Returns the tile corresponding to the CP437 code point.
    pub fn get_cp437_tile(&self, c: char) -> Option<&Tile> {
        let cp437_index = char_to_cp437(c)?;
        self.get_tile(cp437_index as u32)
    }

    /// Returns the UV coordinates for a tile index (normalized 0.0-1.0).
    ///
    /// Returns (u_min, v_min, u_max, v_max).
    pub fn get_tile_uv(&self, index: u32) -> Option<(f32, f32, f32, f32)> {
        if index >= self.tiles.len() as u32 {
            return None;
        }
        let col = index % self.columns;
        let row = index / self.columns;
        let u_min = (col * self.tile_width) as f32 / self.texture_width as f32;
        let v_min = (row * self.tile_height) as f32 / self.texture_height as f32;
        let u_max = ((col + 1) * self.tile_width) as f32 / self.texture_width as f32;
        let v_max = ((row + 1) * self.tile_height) as f32 / self.texture_height as f32;
        Some((u_min, v_min, u_max, v_max))
    }

    /// Returns the number of tiles in the tileset.
    pub fn len(&self) -> usize {
        self.tiles.len()
    }

    /// Returns true if the tileset is empty.
    pub fn is_empty(&self) -> bool {
        self.tiles.is_empty()
    }

    /// Returns an iterator over all tiles.
    pub fn iter(&self) -> impl Iterator<Item = &Tile> {
        self.tiles.iter()
    }
}

/// Converts a Unicode character to its CP437 code point.
///
/// CP437 is the classic IBM PC character set used by many roguelikes.
/// Returns None for characters not in CP437.
fn char_to_cp437(c: char) -> Option<u8> {
    // Standard ASCII range (0x20-0x7E maps directly)
    if c as u32 >= 0x20 && c as u32 <= 0x7E {
        return Some(c as u8);
    }

    // Special mappings for common CP437 characters
    match c {
        // Control character graphics (0x00-0x1F)
        '\0' => Some(0),
        '☺' => Some(1),
        '☻' => Some(2),
        '♥' => Some(3),
        '♦' => Some(4),
        '♣' => Some(5),
        '♠' => Some(6),
        '•' => Some(7),
        '◘' => Some(8),
        '○' => Some(9),
        '◙' => Some(10),
        '♂' => Some(11),
        '♀' => Some(12),
        '♪' => Some(13),
        '♫' => Some(14),
        '☼' => Some(15),
        '►' => Some(16),
        '◄' => Some(17),
        '↕' => Some(18),
        '‼' => Some(19),
        '¶' => Some(20),
        '§' => Some(21),
        '▬' => Some(22),
        '↨' => Some(23),
        '↑' => Some(24),
        '↓' => Some(25),
        '→' => Some(26),
        '←' => Some(27),
        '∟' => Some(28),
        '↔' => Some(29),
        '▲' => Some(30),
        '▼' => Some(31),

        // Extended ASCII (0x80-0xFF)
        '░' => Some(176),
        '▒' => Some(177),
        '▓' => Some(178),
        '│' => Some(179),
        '┤' => Some(180),
        '╡' => Some(181),
        '╢' => Some(182),
        '╖' => Some(183),
        '╕' => Some(184),
        '╣' => Some(185),
        '║' => Some(186),
        '╗' => Some(187),
        '╝' => Some(188),
        '╜' => Some(189),
        '╛' => Some(190),
        '┐' => Some(191),
        '└' => Some(192),
        '┴' => Some(193),
        '┬' => Some(194),
        '├' => Some(195),
        '─' => Some(196),
        '┼' => Some(197),
        '╞' => Some(198),
        '╟' => Some(199),
        '╚' => Some(200),
        '╔' => Some(201),
        '╩' => Some(202),
        '╦' => Some(203),
        '╠' => Some(204),
        '═' => Some(205),
        '╬' => Some(206),
        '╧' => Some(207),
        '╨' => Some(208),
        '╤' => Some(209),
        '╥' => Some(210),
        '╙' => Some(211),
        '╘' => Some(212),
        '╒' => Some(213),
        '╓' => Some(214),
        '╫' => Some(215),
        '╪' => Some(216),
        '┘' => Some(217),
        '┌' => Some(218),
        '█' => Some(219),
        '▄' => Some(220),
        '▌' => Some(221),
        '▐' => Some(222),
        '▀' => Some(223),

        // Some common extended characters
        'α' => Some(224),
        'ß' => Some(225),
        'Γ' => Some(226),
        'π' => Some(227),
        'Σ' => Some(228),
        'σ' => Some(229),
        'µ' => Some(230),
        'τ' => Some(231),
        'Φ' => Some(232),
        'Θ' => Some(233),
        'Ω' => Some(234),
        'δ' => Some(235),
        '∞' => Some(236),
        'φ' => Some(237),
        'ε' => Some(238),
        '∩' => Some(239),
        '≡' => Some(240),
        '±' => Some(241),
        '≥' => Some(242),
        '≤' => Some(243),
        '⌠' => Some(244),
        '⌡' => Some(245),
        '÷' => Some(246),
        '≈' => Some(247),
        '°' => Some(248),
        '∙' => Some(249),
        '·' => Some(250),
        '√' => Some(251),
        'ⁿ' => Some(252),
        '²' => Some(253),
        '■' => Some(254),

        _ => None,
    }
}

/// Converts a CP437 code point to its Unicode character.
pub fn cp437_to_char(code: u8) -> char {
    // This is the full CP437 to Unicode mapping
    const CP437_TABLE: [char; 256] = [
        // 0x00-0x0F
        '\0', '☺', '☻', '♥', '♦', '♣', '♠', '•', '◘', '○', '◙', '♂', '♀', '♪', '♫', '☼',
        // 0x10-0x1F
        '►', '◄', '↕', '‼', '¶', '§', '▬', '↨', '↑', '↓', '→', '←', '∟', '↔', '▲', '▼',
        // 0x20-0x2F (standard ASCII)
        ' ', '!', '"', '#', '$', '%', '&', '\'', '(', ')', '*', '+', ',', '-', '.', '/',
        // 0x30-0x3F
        '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', ':', ';', '<', '=', '>', '?',
        // 0x40-0x4F
        '@', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O',
        // 0x50-0x5F
        'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '[', '\\', ']', '^', '_',
        // 0x60-0x6F
        '`', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o',
        // 0x70-0x7F
        'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '{', '|', '}', '~', '⌂',
        // 0x80-0x8F
        'Ç', 'ü', 'é', 'â', 'ä', 'à', 'å', 'ç', 'ê', 'ë', 'è', 'ï', 'î', 'ì', 'Ä', 'Å',
        // 0x90-0x9F
        'É', 'æ', 'Æ', 'ô', 'ö', 'ò', 'û', 'ù', 'ÿ', 'Ö', 'Ü', '¢', '£', '¥', '₧', 'ƒ',
        // 0xA0-0xAF
        'á', 'í', 'ó', 'ú', 'ñ', 'Ñ', 'ª', 'º', '¿', '⌐', '¬', '½', '¼', '¡', '«', '»',
        // 0xB0-0xBF
        '░', '▒', '▓', '│', '┤', '╡', '╢', '╖', '╕', '╣', '║', '╗', '╝', '╜', '╛', '┐',
        // 0xC0-0xCF
        '└', '┴', '┬', '├', '─', '┼', '╞', '╟', '╚', '╔', '╩', '╦', '╠', '═', '╬', '╧',
        // 0xD0-0xDF
        '╨', '╤', '╥', '╙', '╘', '╒', '╓', '╫', '╪', '┘', '┌', '█', '▄', '▌', '▐', '▀',
        // 0xE0-0xEF
        'α', 'ß', 'Γ', 'π', 'Σ', 'σ', 'µ', 'τ', 'Φ', 'Θ', 'Ω', 'δ', '∞', 'φ', 'ε', '∩',
        // 0xF0-0xFF
        '≡', '±', '≥', '≤', '⌠', '⌡', '÷', '≈', '°', '∙', '·', '√', 'ⁿ', '²', '■', ' ',
    ];
    CP437_TABLE[code as usize]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_char_to_cp437_ascii() {
        assert_eq!(char_to_cp437('@'), Some(64));
        assert_eq!(char_to_cp437('A'), Some(65));
        assert_eq!(char_to_cp437(' '), Some(32));
        assert_eq!(char_to_cp437('#'), Some(35));
    }

    #[test]
    fn test_char_to_cp437_box_drawing() {
        assert_eq!(char_to_cp437('─'), Some(196));
        assert_eq!(char_to_cp437('│'), Some(179));
        assert_eq!(char_to_cp437('┌'), Some(218));
        assert_eq!(char_to_cp437('█'), Some(219));
    }

    #[test]
    fn test_char_to_cp437_special() {
        assert_eq!(char_to_cp437('☺'), Some(1));
        assert_eq!(char_to_cp437('☻'), Some(2));
        assert_eq!(char_to_cp437('♥'), Some(3));
        assert_eq!(char_to_cp437('░'), Some(176));
    }

    #[test]
    fn test_cp437_roundtrip() {
        // Test that common characters roundtrip correctly
        for c in ['@', '#', '.', ' ', 'A', 'z', '0', '9'] {
            if let Some(cp) = char_to_cp437(c) {
                assert_eq!(cp437_to_char(cp), c);
            }
        }
    }

    #[test]
    fn test_tile_dimensions() {
        // Create a minimal 2x2 pixel RGBA image (will be 1 tile of 2x2)
        let pixels: Vec<u8> = vec![
            255, 0, 0, 255, // red
            0, 255, 0, 255, // green
            0, 0, 255, 255, // blue
            255, 255, 0, 255, // yellow
        ];

        let img = image::RgbaImage::from_raw(2, 2, pixels).unwrap();
        let tileset = Tileset::from_image(image::DynamicImage::ImageRgba8(img), 2, 2).unwrap();

        assert_eq!(tileset.tile_width, 2);
        assert_eq!(tileset.tile_height, 2);
        assert_eq!(tileset.columns, 1);
        assert_eq!(tileset.rows, 1);
        assert_eq!(tileset.len(), 1);
    }

    #[test]
    fn test_tile_grid() {
        // Create a 4x4 image with 2x2 tiles (4 tiles total)
        let mut pixels = vec![0u8; 4 * 4 * 4];
        // Fill each quadrant with a different color
        for y in 0..4 {
            for x in 0..4 {
                let idx = (y * 4 + x) * 4;
                let (r, g, b) = match (x / 2, y / 2) {
                    (0, 0) => (255, 0, 0),   // top-left: red
                    (1, 0) => (0, 255, 0),   // top-right: green
                    (0, 1) => (0, 0, 255),   // bottom-left: blue
                    (1, 1) => (255, 255, 0), // bottom-right: yellow
                    _ => (0, 0, 0),
                };
                pixels[idx] = r;
                pixels[idx + 1] = g;
                pixels[idx + 2] = b;
                pixels[idx + 3] = 255;
            }
        }

        let img = image::RgbaImage::from_raw(4, 4, pixels).unwrap();
        let tileset = Tileset::from_image(image::DynamicImage::ImageRgba8(img), 2, 2).unwrap();

        assert_eq!(tileset.len(), 4);
        assert_eq!(tileset.columns, 2);
        assert_eq!(tileset.rows, 2);

        // Check tile 0 (top-left) is red
        let tile0 = tileset.get_tile(0).unwrap();
        assert_eq!(tile0.get_pixel(0, 0), Some([255, 0, 0, 255]));

        // Check tile 1 (top-right) is green
        let tile1 = tileset.get_tile(1).unwrap();
        assert_eq!(tile1.get_pixel(0, 0), Some([0, 255, 0, 255]));

        // Check tile 2 (bottom-left) is blue
        let tile2 = tileset.get_tile(2).unwrap();
        assert_eq!(tile2.get_pixel(0, 0), Some([0, 0, 255, 255]));

        // Check tile 3 (bottom-right) is yellow
        let tile3 = tileset.get_tile(3).unwrap();
        assert_eq!(tile3.get_pixel(0, 0), Some([255, 255, 0, 255]));
    }

    #[test]
    fn test_get_tile_at() {
        let pixels = vec![0u8; 4 * 4 * 4];
        let img = image::RgbaImage::from_raw(4, 4, pixels).unwrap();
        let tileset = Tileset::from_image(image::DynamicImage::ImageRgba8(img), 2, 2).unwrap();

        // get_tile_at(col, row) should match get_tile(row * columns + col)
        assert_eq!(
            tileset.get_tile_at(0, 0).map(|t| t.index),
            tileset.get_tile(0).map(|t| t.index)
        );
        assert_eq!(
            tileset.get_tile_at(1, 0).map(|t| t.index),
            tileset.get_tile(1).map(|t| t.index)
        );
        assert_eq!(
            tileset.get_tile_at(0, 1).map(|t| t.index),
            tileset.get_tile(2).map(|t| t.index)
        );
        assert_eq!(
            tileset.get_tile_at(1, 1).map(|t| t.index),
            tileset.get_tile(3).map(|t| t.index)
        );

        // Out of bounds
        assert!(tileset.get_tile_at(2, 0).is_none());
        assert!(tileset.get_tile_at(0, 2).is_none());
    }

    #[test]
    fn test_uv_coordinates() {
        let pixels = vec![0u8; 4 * 4 * 4];
        let img = image::RgbaImage::from_raw(4, 4, pixels).unwrap();
        let tileset = Tileset::from_image(image::DynamicImage::ImageRgba8(img), 2, 2).unwrap();

        // Tile 0: should be (0.0, 0.0) to (0.5, 0.5)
        let uv0 = tileset.get_tile_uv(0).unwrap();
        assert_eq!(uv0, (0.0, 0.0, 0.5, 0.5));

        // Tile 1: should be (0.5, 0.0) to (1.0, 0.5)
        let uv1 = tileset.get_tile_uv(1).unwrap();
        assert_eq!(uv1, (0.5, 0.0, 1.0, 0.5));

        // Tile 3: should be (0.5, 0.5) to (1.0, 1.0)
        let uv3 = tileset.get_tile_uv(3).unwrap();
        assert_eq!(uv3, (0.5, 0.5, 1.0, 1.0));
    }

    #[test]
    fn test_invalid_dimensions() {
        let pixels = vec![0u8; 4 * 4 * 4];
        let img = image::RgbaImage::from_raw(4, 4, pixels).unwrap();

        // Zero tile size should fail
        assert!(Tileset::from_image(image::DynamicImage::ImageRgba8(img.clone()), 0, 2).is_err());
        assert!(Tileset::from_image(image::DynamicImage::ImageRgba8(img.clone()), 2, 0).is_err());

        // Tile larger than image should fail
        assert!(Tileset::from_image(image::DynamicImage::ImageRgba8(img), 8, 8).is_err());
    }
}
