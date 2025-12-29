//! Tileset loading demonstration.
//!
//! This example shows how to:
//! - Load a PNG tileset/spritesheet
//! - Access individual tiles by index or grid position
//! - Use CP437 character mapping for classic roguelike tilesets
//! - Get UV coordinates for GPU rendering
//!
//! Run with: cargo run --example tileset_demo

use runeforge_tileset::{cp437_to_char, Tileset};

fn main() {
    println!("=== Runeforge Tileset Demo ===\n");

    // Load the 8x8 terminal tileset
    let tileset_path = "crates/runeforge-tileset/assets/terminal8x8_gs_ro.png";
    let tileset = Tileset::from_file(tileset_path, 8, 8).expect("Failed to load tileset");

    println!("Tileset loaded: {}", tileset_path);
    println!(
        "  - Tile size: {}x{}",
        tileset.tile_width, tileset.tile_height
    );
    println!("  - Grid: {}x{} tiles", tileset.columns, tileset.rows);
    println!("  - Total tiles: {}", tileset.len());
    println!(
        "  - Texture size: {}x{}",
        tileset.texture_width, tileset.texture_height
    );
    println!();

    // Demonstrate tile access by index
    println!("Accessing tiles by index:");
    for idx in [0u32, 1, 32, 64, 127] {
        if let Some(tile) = tileset.get_tile(idx) {
            let pixel = tile.get_pixel(0, 0).unwrap_or([0, 0, 0, 0]);
            println!(
                "  Tile {}: pos ({}, {}), corner pixel: rgba({}, {}, {}, {})",
                idx, tile.src_x, tile.src_y, pixel[0], pixel[1], pixel[2], pixel[3]
            );
        }
    }
    println!();

    // Demonstrate tile access by grid position
    println!("Accessing tiles by grid position:");
    let max_col = tileset.columns.saturating_sub(1);
    let max_row = tileset.rows.saturating_sub(1);
    for (col, row) in [(0, 0), (max_col, 0), (0, max_row), (max_col, max_row)] {
        if let Some(tile) = tileset.get_tile_at(col, row) {
            println!(
                "  Grid ({}, {}): tile index {}, src pos ({}, {})",
                col, row, tile.index, tile.src_x, tile.src_y
            );
        }
    }
    println!();

    // Demonstrate UV coordinates for GPU rendering
    println!("UV coordinates for tiles:");
    for idx in [0u32, 1, 16, 64] {
        if let Some((u_min, v_min, u_max, v_max)) = tileset.get_tile_uv(idx) {
            println!(
                "  Tile {}: UV ({:.4}, {:.4}) to ({:.4}, {:.4})",
                idx, u_min, v_min, u_max, v_max
            );
        }
    }
    println!();

    // Demonstrate CP437 character mapping
    println!("CP437 character mapping (index -> unicode):");
    let indices: [u8; 10] = [0, 1, 2, 32, 35, 46, 64, 65, 176, 219];
    for idx in indices {
        let unicode = cp437_to_char(idx);
        let display = if unicode.is_control() || unicode == '\0' {
            "(control)".to_string()
        } else {
            format!("'{}'", unicode)
        };
        println!("  CP437 {} -> U+{:04X} {}", idx, unicode as u32, display);
    }
    println!();

    // Demonstrate getting tiles for roguelike characters
    println!("Getting tiles for roguelike characters:");
    let roguelike_chars = ['@', '#', '.', '+', '-', '|', ' '];
    for c in roguelike_chars {
        if let Some(tile) = tileset.get_cp437_tile(c) {
            // Count non-transparent pixels
            let visible_pixels = tile
                .pixels
                .chunks(4)
                .filter(|p| p[3] > 0 && (p[0] > 0 || p[1] > 0 || p[2] > 0))
                .count();
            println!(
                "  '{}' (tile {}): {} visible pixels",
                c, tile.index, visible_pixels
            );
        } else {
            println!("  '{}': not found in tileset", c);
        }
    }
    println!();

    // Demonstrate iterating over tiles
    println!("First 8 tiles (analyzing content):");
    for tile in tileset.iter().take(8) {
        // Calculate average brightness
        let total_brightness: u32 = tile
            .pixels
            .chunks(4)
            .map(|p| (p[0] as u32 + p[1] as u32 + p[2] as u32) / 3)
            .sum();
        let avg = total_brightness / (tile.width * tile.height);

        // Count non-zero alpha pixels
        let opaque = tile.pixels.chunks(4).filter(|p| p[3] > 0).count();

        println!(
            "  Tile {}: avg brightness {}, {} opaque pixels",
            tile.index, avg, opaque
        );
    }

    // Also demo the 16x16 tileset
    println!("\n--- Loading 16x16 tileset ---\n");

    let tileset16_path = "crates/runeforge-tileset/assets/dejavu16x16_gs_tc.png";

    if let Ok(tileset16) = Tileset::from_file(tileset16_path, 16, 16) {
        println!("Tileset loaded: dejavu16x16_gs_tc.png");
        println!(
            "  - Tile size: {}x{}",
            tileset16.tile_width, tileset16.tile_height
        );
        println!("  - Grid: {}x{} tiles", tileset16.columns, tileset16.rows);
        println!("  - Total tiles: {}", tileset16.len());

        // Compare '@' character in both tilesets
        if let (Some(t8), Some(t16)) = (tileset.get_cp437_tile('@'), tileset16.get_cp437_tile('@'))
        {
            let pixels8 = t8.pixels.chunks(4).filter(|p| p[3] > 128).count();
            let pixels16 = t16.pixels.chunks(4).filter(|p| p[3] > 128).count();
            println!("\n  '@' comparison:");
            println!("    8x8 tileset: {} visible pixels", pixels8);
            println!("    16x16 tileset: {} visible pixels", pixels16);
        }
    }

    println!("\n=== Demo Complete ===");
}
