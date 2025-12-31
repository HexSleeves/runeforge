//! Simple roguelike rendering demo using the software renderer.
//!
//! This demo creates a basic roguelike scene with a player, walls, and floor.
//! The output is saved to a PNG file.
//!
//! Note: This demo requires a TrueType font file. Download a free font like:
//! - DejaVuSansMono.ttf
//! - CascadiaCode.ttf
//! - FiraCode.ttf
//!
//! Run with: cargo run --example roguelike_demo

use runeforge_color::Color;
use runeforge_geometry::prelude::IVec2;
use runeforge_terminal::prelude::{Console, SoftwareRenderer};
use runeforge_tileset::prelude::TrueTypeFont;

fn main() {
    println!("Roguelike Software Renderer Demo");
    println!("================================");

    // Try to load a font (you'll need to have one available)
    let font_path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/crates/runeforge-terminal/assets/font.ttf"
    );

    let font_data = match std::fs::read(font_path) {
        Ok(data) => data,
        Err(_) => {
            eprintln!("Error: Could not find font file at '{}'", font_path);
            eprintln!();
            eprintln!("Please download a TrueType font and save it as 'font.ttf'");
            eprintln!("Free fonts available:");
            eprintln!("  - https://github.com/dejavu-fonts/dejavu-fonts/releases");
            eprintln!("  - https://github.com/microsoft/cascadia-code/releases");
            std::process::exit(1);
        }
    };

    let font = TrueTypeFont::from_bytes(&font_data, 16.0).expect("Failed to load font");

    // Create a 40x25 console (typical small roguelike size)
    let mut renderer = SoftwareRenderer::new(40, 25, &font);

    // Clear to black
    renderer.clear();

    // Draw a border box
    renderer.draw_box(0, 0, 40, 25, Color::GRAY, Color::BLACK, false);

    // Draw a title
    renderer.draw_string(IVec2::new(14, 0), " DUNGEON ", Color::YELLOW, Color::BLACK);

    // Draw some floor tiles (.)
    for y in 2..23 {
        for x in 2..38 {
            renderer.draw_char(IVec2::new(x, y), '.', Color::DARK_GRAY, Color::BLACK);
        }
    }

    // Draw some walls (#)
    // Vertical wall
    for y in 5..15 {
        renderer.draw_char(IVec2::new(15, y), '#', Color::WHITE, Color::BLACK);
    }
    // Horizontal wall
    for x in 10..25 {
        renderer.draw_char(IVec2::new(x, 10), '#', Color::WHITE, Color::BLACK);
    }

    // Draw a door (+)
    renderer.draw_char(IVec2::new(15, 10), '+', Color::BROWN, Color::BLACK);

    // Draw some monsters
    renderer.draw_char(IVec2::new(8, 7), 'g', Color::GREEN, Color::BLACK); // Goblin
    renderer.draw_char(IVec2::new(25, 15), 'o', Color::RED, Color::BLACK); // Orc
    renderer.draw_char(
        IVec2::new(30, 18),
        's',
        Color::rgb(100, 200, 100),
        Color::BLACK,
    );
    // Snake

    // Draw some items
    renderer.draw_char(IVec2::new(12, 8), '!', Color::MAGENTA, Color::BLACK); // Potion
    renderer.draw_char(IVec2::new(20, 12), '/', Color::BROWN, Color::BLACK); // Staff
    renderer.draw_char(IVec2::new(5, 20), '$', Color::YELLOW, Color::BLACK); // Gold

    // Draw the player (@)
    renderer.draw_char(IVec2::new(20, 20), '@', Color::CYAN, Color::BLACK);

    // Draw a status bar
    renderer.draw_hline(23, 1, 38, '─', Color::GRAY, Color::BLACK);

    renderer.draw_string(IVec2::new(2, 24), "HP:100/100", Color::GREEN, Color::BLACK);
    renderer.draw_string(IVec2::new(15, 24), "Lvl:1", Color::YELLOW, Color::BLACK);
    renderer.draw_string(IVec2::new(24, 24), "Gold:50", Color::YELLOW, Color::BLACK);

    // Render and save to PNG
    renderer
        .save_png("roguelike_demo.png")
        .expect("Failed to save PNG");

    println!("Demo complete!");
    println!("Output saved to: roguelike_demo.png");
    println!();
    println!("The demo shows:");
    println!("  - Border box using box-drawing characters");
    println!("  - Floor tiles (.)");
    println!("  - Walls (#) forming a cross pattern");
    println!("  - A door (+) at the intersection");
    println!("  - Monsters (g, o, s) with different colors");
    println!("  - Items (!, /, $)");
    println!("  - Player character (@) in cyan");
    println!("  - Status bar with HP, level, and gold");
}
