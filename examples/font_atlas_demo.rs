//! Font atlas demonstration.
//!
//! This example shows how to:
//! - Load fonts (BDF bitmap fonts)
//! - Create glyph atlases for efficient rendering
//! - Access glyph metrics and UV coordinates
//! - Render characters to buffers
//!
//! Run with: cargo run --example font_atlas_demo

use runeforge_color::Color;
use runeforge_tileset::{Font, GlyphAtlas};

/// Minimal BDF font for demonstration (no external file needed)
const DEMO_BDF: &[u8] = b"STARTFONT 2.1
FONT -Demo-Font-Medium-R-Normal--8-80-75-75-C-80-ISO10646-1
SIZE 8 75 75
FONTBOUNDINGBOX 8 8 0 0
STARTPROPERTIES 2
FONT_ASCENT 8
FONT_DESCENT 0
ENDPROPERTIES
CHARS 16
STARTCHAR space
ENCODING 32
SWIDTH 500 0
DWIDTH 8 0
BBX 8 8 0 0
BITMAP
00
00
00
00
00
00
00
00
ENDCHAR
STARTCHAR hash
ENCODING 35
SWIDTH 500 0
DWIDTH 8 0
BBX 8 8 0 0
BITMAP
24
24
7E
24
7E
24
24
00
ENDCHAR
STARTCHAR dot
ENCODING 46
SWIDTH 500 0
DWIDTH 8 0
BBX 8 8 0 0
BITMAP
00
00
00
00
00
18
18
00
ENDCHAR
STARTCHAR zero
ENCODING 48
SWIDTH 500 0
DWIDTH 8 0
BBX 8 8 0 0
BITMAP
3C
42
46
5A
62
42
3C
00
ENDCHAR
STARTCHAR one
ENCODING 49
SWIDTH 500 0
DWIDTH 8 0
BBX 8 8 0 0
BITMAP
08
18
08
08
08
08
1C
00
ENDCHAR
STARTCHAR two
ENCODING 50
SWIDTH 500 0
DWIDTH 8 0
BBX 8 8 0 0
BITMAP
3C
42
02
0C
30
40
7E
00
ENDCHAR
STARTCHAR at
ENCODING 64
SWIDTH 500 0
DWIDTH 8 0
BBX 8 8 0 0
BITMAP
3C
42
5A
5A
5C
40
3E
00
ENDCHAR
STARTCHAR A
ENCODING 65
SWIDTH 500 0
DWIDTH 8 0
BBX 8 8 0 0
BITMAP
18
24
42
42
7E
42
42
00
ENDCHAR
STARTCHAR B
ENCODING 66
SWIDTH 500 0
DWIDTH 8 0
BBX 8 8 0 0
BITMAP
7C
42
7C
42
42
42
7C
00
ENDCHAR
STARTCHAR C
ENCODING 67
SWIDTH 500 0
DWIDTH 8 0
BBX 8 8 0 0
BITMAP
3C
42
40
40
40
42
3C
00
ENDCHAR
STARTCHAR D
ENCODING 68
SWIDTH 500 0
DWIDTH 8 0
BBX 8 8 0 0
BITMAP
78
44
42
42
42
44
78
00
ENDCHAR
STARTCHAR E
ENCODING 69
SWIDTH 500 0
DWIDTH 8 0
BBX 8 8 0 0
BITMAP
7E
40
7C
40
40
40
7E
00
ENDCHAR
STARTCHAR F
ENCODING 70
SWIDTH 500 0
DWIDTH 8 0
BBX 8 8 0 0
BITMAP
7E
40
7C
40
40
40
40
00
ENDCHAR
STARTCHAR G
ENCODING 71
SWIDTH 500 0
DWIDTH 8 0
BBX 8 8 0 0
BITMAP
3C
42
40
4E
42
42
3C
00
ENDCHAR
STARTCHAR H
ENCODING 72
SWIDTH 500 0
DWIDTH 8 0
BBX 8 8 0 0
BITMAP
42
42
7E
42
42
42
42
00
ENDCHAR
STARTCHAR I
ENCODING 73
SWIDTH 500 0
DWIDTH 8 0
BBX 8 8 0 0
BITMAP
3E
08
08
08
08
08
3E
00
ENDCHAR
ENDFONT
";

fn main() {
    println!("=== Runeforge Font Atlas Demo ===\n");

    // Load the embedded BDF font
    let font =
        runeforge_tileset::BitmapFont::from_bytes(DEMO_BDF).expect("Failed to load demo font");

    println!("Font loaded:");
    println!("  - Name: {}", font.name());
    println!(
        "  - Cell size: {}x{}",
        font.cell_width(),
        font.cell_height()
    );
    println!("  - Line height: {}", font.line_height());
    println!();

    // Check which glyphs are available
    println!("Available glyphs:");
    let test_chars = [
        '@', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', '#', '.', '0', '1', '2', ' ', 'Z',
    ];
    for c in test_chars {
        let status = if font.has_glyph(c) { "✓" } else { "✗" };
        println!("  {} '{}'", status, c);
    }
    println!();

    // Render individual glyphs
    println!("Rendering glyphs:");
    for c in ['@', 'A', '#'] {
        if let Some(glyph) = font.render_glyph(c) {
            println!("  '{}': {}x{} pixels", c, glyph.width, glyph.height);
            println!(
                "        bearing: ({}, {})",
                glyph.bearing_x, glyph.bearing_y
            );
            println!("        bitmap size: {} bytes", glyph.bitmap.len());

            // Show ASCII art preview of the glyph
            println!("        preview:");
            for y in 0..glyph.height {
                print!("          ");
                for x in 0..glyph.width {
                    let idx = (y * glyph.width + x) as usize;
                    let pixel = glyph.bitmap.get(idx).copied().unwrap_or(0);
                    print!(
                        "{}",
                        if pixel > 128 {
                            "██"
                        } else if pixel > 0 {
                            "░░"
                        } else {
                            "  "
                        }
                    );
                }
                println!();
            }
        }
    }
    println!();

    // Create a glyph atlas
    println!("Creating glyph atlas...");
    let atlas = GlyphAtlas::from_font(&font, 8, 8);
    println!("Atlas created:");
    println!(
        "  - Texture size: {}x{}",
        atlas.texture_width, atlas.texture_height
    );
    println!("  - Cell size: {}x{}", atlas.cell_width, atlas.cell_height);
    println!("  - Grid: {}x{}", atlas.columns, atlas.rows);
    println!("  - Glyphs: {}", atlas.len());
    println!();

    // Access glyph info from atlas
    println!("Glyph info from atlas:");
    for c in ['@', 'A', '#', '.'] {
        if let Some(glyph) = atlas.get_glyph(c) {
            println!(
                "  '{}': atlas pos ({}, {}), size {}x{}",
                c, glyph.atlas_x, glyph.atlas_y, glyph.width, glyph.height
            );
        }
    }
    println!();

    // Get UV coordinates for GPU rendering
    println!("UV coordinates for GPU rendering:");
    for c in ['@', 'A', '#', '.'] {
        if let Some((u_min, v_min, u_max, v_max)) = atlas.get_uv(c) {
            println!(
                "  '{}': ({:.3}, {:.3}) to ({:.3}, {:.3})",
                c, u_min, v_min, u_max, v_max
            );
        }
    }
    println!();

    // Demonstrate rendering to a buffer
    println!("Rendering text to buffer...");
    let text = "@ABCDEFGHI";
    let buffer_width = (text.len() as u32) * 8;
    let buffer_height = 8u32;
    let mut buffer = vec![0u8; (buffer_width * buffer_height * 4) as usize];

    // Render text in green on dark background
    let fg = Color::rgb(0, 255, 0); // Green foreground
    let bg = Some(Color::rgb(32, 32, 32)); // Dark gray background

    for (i, c) in text.chars().enumerate() {
        let x = (i as u32) * 8;
        atlas.render_char(&mut buffer, buffer_width, x, 0, c, fg, bg);
    }

    // Show rendered text as ASCII art
    println!("  Text: \"{}\"", text);
    println!("  Buffer: {}x{} pixels", buffer_width, buffer_height);
    println!("  Rendered output:");
    for y in 0..buffer_height {
        print!("    ");
        for x in 0..buffer_width {
            let idx = ((y * buffer_width + x) * 4) as usize;
            let g = buffer[idx + 1]; // Green channel
            print!(
                "{}",
                if g > 200 {
                    "█"
                } else if g > 100 {
                    "▓"
                } else if g > 50 {
                    "░"
                } else {
                    " "
                }
            );
        }
        println!();
    }
    println!();

    // Show default charset info
    println!("Default roguelike charset:");
    let charset = GlyphAtlas::default_charset();
    println!("  - Total characters: {}", charset.len());

    // Group by category
    let ascii_count = charset
        .iter()
        .filter(|&&c| c as u32 >= 32 && c as u32 <= 126)
        .count();
    let box_chars: Vec<_> = charset
        .iter()
        .filter(|&&c| "─│┌┐└┘├┤┬┴┼═║╔╗╚╝╠╣╦╩╬".contains(c))
        .collect();
    let block_chars: Vec<_> = charset
        .iter()
        .filter(|&&c| "░▒▓█▄▀▌▐".contains(c))
        .collect();

    println!("  - ASCII printable: {} chars", ascii_count);
    println!(
        "  - Box drawing: {} chars ({})",
        box_chars.len(),
        box_chars.iter().map(|c| c.to_string()).collect::<String>()
    );
    println!(
        "  - Block elements: {} chars ({})",
        block_chars.len(),
        block_chars
            .iter()
            .map(|c| c.to_string())
            .collect::<String>()
    );

    println!("\n=== Demo Complete ===");
}
