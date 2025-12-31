//! Demo showing how to load and use PNG-based tilesets (like classic roguelike sprite sheets).
//!
//! This demonstrates loading a CP437 tileset PNG (16x16 grid of glyphs) and
//! extracting individual tiles for use in rendering.
//!
//! Run with: cargo run --example png_tileset_demo

use runeforge_tileset::prelude::Tileset;

fn main() {
    println!("PNG Tileset Demo");
    println!("================\n");

    // Try to load the bundled terminal tileset
    let tileset_path = "crates/runeforge-tileset/assets/terminal8x8_gs_ro.png";

    println!("Loading tileset from: {}", tileset_path);

    let tileset = match Tileset::from_file(tileset_path, 8, 8) {
        Ok(t) => {
            println!("✓ Successfully loaded tileset!");
            t
        }
        Err(e) => {
            eprintln!("✗ Failed to load tileset: {}", e);
            eprintln!("\nMake sure you're running from the `runeforge-rl/` directory:");
            eprintln!("  cargo run --example png_tileset_demo");
            std::process::exit(1);
        }
    };

    println!("\nTileset information:");
    println!(
        "  Dimensions: {}x{} pixels per tile",
        tileset.tile_width, tileset.tile_height
    );
    println!(
        "  Grid: {}x{} tiles ({} total)",
        tileset.columns,
        tileset.rows,
        tileset.len()
    );
    println!(
        "  Texture: {}x{} pixels\n",
        tileset.texture_width, tileset.texture_height
    );

    // Demonstrate accessing specific tiles

    println!("CP437 Character Mapping Examples:");
    println!("==================================");

    let test_chars = [
        ('@', "Player character"),
        ('#', "Wall"),
        ('.', "Floor"),
        ('g', "Goblin"),
        ('D', "Dragon"),
        ('!', "Potion"),
        ('/', "Sword"),
        ('$', "Gold"),
        ('♥', "Heart (CP437 special)"),
        ('☺', "Smiley (CP437 special)"),
        ('─', "Horizontal line"),
        ('│', "Vertical line"),
        ('┌', "Top-left corner"),
        ('█', "Solid block"),
    ];

    for (ch, description) in test_chars {
        if let Some(tile) = tileset.get_cp437_tile(ch) {
            // Get the top-left pixel color to show it's loaded
            let pixel = tile.get_pixel(0, 0).unwrap_or([0, 0, 0, 0]);

            println!(
                "  '{}' - {} (index {}, pixel sample: R:{} G:{} B:{} A:{})",
                ch, description, tile.index, pixel[0], pixel[1], pixel[2], pixel[3]
            );
        } else {
            println!("  '{}' - {} (NOT FOUND in CP437)", ch, description);
        }
    }

    println!("\nDirect tile access:");
    println!("===================");

    // Access tile by index (row-major order)
    if let Some(tile) = tileset.get_tile(64) {
        println!("  Tile 64 ('@' in CP437):");
        println!(
            "    Position in sprite sheet: ({}, {})",
            tile.src_x, tile.src_y
        );
        println!("    Size: {}x{} pixels", tile.width, tile.height);
        println!("    Pixel data length: {} bytes", tile.pixels.len());
    }

    // Access tile by grid position
    if let Some(tile) = tileset.get_tile_at(0, 4) {
        // Row 4, column 0 = index 64 = '@'
        println!("\n  Tile at grid position (0, 4):");
        println!("    Index: {}", tile.index);
        println!("    Character: '@' (64 in CP437)");
    }

    // Get UV coordinates for GPU rendering
    println!("\nUV Coordinates for GPU rendering:");
    println!("==================================");

    for (index, ch) in [(0, "NULL"), (32, "SPACE"), (64, "@"), (35, "#")] {
        if let Some((u_min, v_min, u_max, v_max)) = tileset.get_tile_uv(index) {
            println!(
                "  '{}' (index {}): UV({:.3}, {:.3}) to ({:.3}, {:.3})",
                ch, index, u_min, v_min, u_max, v_max
            );
        }
    }

    println!("\nIntegration with rendering backends:");
    println!("====================================");
    println!("1. Software/CPU rendering:");
    println!("   - Use tile.pixels to get raw RGBA data");
    println!("   - Blit directly to framebuffer\n");

    println!("2. GPU rendering (OpenGL/wgpu):");
    println!("   - Upload tileset.texture as GPU texture");
    println!("   - Use get_tile_uv() for texture coordinates");
    println!("   - Render quads with appropriate UV mapping\n");

    println!("3. With GlyphAtlas (for text rendering):");
    println!("   - Create TrueType font for crisp text");
    println!("   - Use Tileset for decorative tiles");
    println!("   - Combine both for hybrid rendering\n");

    println!("Demo complete! The tileset system supports:");
    println!("  ✓ PNG sprite sheet loading");
    println!("  ✓ CP437 character mapping");
    println!("  ✓ Individual tile extraction");
    println!("  ✓ GPU-ready UV coordinates");
    println!("  ✓ Direct pixel access for CPU rendering");
}
