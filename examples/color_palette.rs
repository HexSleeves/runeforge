//! Color Palette Demo
//!
//! This example showcases the full RGB color capabilities of the terminal
//! by displaying gradient palettes and color mixing.

use runeforge_color::Color;
use runeforge_terminal::prelude::{IVec2, Terminal};
use std::io;
use std::thread;
use std::time::Duration;

fn main() -> io::Result<()> {
    println!("\n=== Runeforge Color Palette Demo ===\n");
    println!("Displaying RGB color capabilities of the ANSI terminal renderer.\n");

    let mut term = Terminal::new(64, 22);

    Terminal::enter_alt_screen()?;
    Terminal::hide_cursor()?;

    // Scene 1: Basic color palette
    term.clear();
    draw_title(&mut term, "Standard Colors");

    let colors = [
        ("BLACK", Color::BLACK),
        ("WHITE", Color::WHITE),
        ("RED", Color::RED),
        ("GREEN", Color::GREEN),
        ("BLUE", Color::BLUE),
        ("YELLOW", Color::YELLOW),
        ("CYAN", Color::CYAN),
        ("MAGENTA", Color::MAGENTA),
    ];

    for (i, (name, color)) in colors.iter().enumerate() {
        let y = 4 + i as i32;
        term.put_string(IVec2::new(4, y), name, *color, Color::BLACK);
        for x in 0..20 {
            term.put_char(IVec2::new(15 + x, y), '█', *color, Color::BLACK);
        }
    }

    term.present()?;
    thread::sleep(Duration::from_secs(2));

    // Scene 2: RGB gradients
    term.clear();
    draw_title(&mut term, "RGB Gradients");

    // Red gradient
    term.put_string(IVec2::new(4, 4), "Red:", Color::WHITE, Color::BLACK);
    for x in 0..50 {
        let intensity = (x * 255 / 50) as u8;
        term.put_char(
            IVec2::new(10 + x, 4),
            '█',
            Color::rgb(intensity, 0, 0),
            Color::BLACK,
        );
    }

    // Green gradient
    term.put_string(IVec2::new(4, 6), "Green:", Color::WHITE, Color::BLACK);
    for x in 0..50 {
        let intensity = (x * 255 / 50) as u8;
        term.put_char(
            IVec2::new(10 + x, 6),
            '█',
            Color::rgb(0, intensity, 0),
            Color::BLACK,
        );
    }

    // Blue gradient
    term.put_string(IVec2::new(4, 8), "Blue:", Color::WHITE, Color::BLACK);
    for x in 0..50 {
        let intensity = (x * 255 / 50) as u8;
        term.put_char(
            IVec2::new(10 + x, 8),
            '█',
            Color::rgb(0, 0, intensity),
            Color::BLACK,
        );
    }

    // Gray gradient
    term.put_string(IVec2::new(4, 10), "Gray:", Color::WHITE, Color::BLACK);
    for x in 0..50 {
        let intensity = (x * 255 / 50) as u8;
        term.put_char(
            IVec2::new(10 + x, 10),
            '█',
            Color::rgb(intensity, intensity, intensity),
            Color::BLACK,
        );
    }

    term.present()?;
    thread::sleep(Duration::from_secs(2));

    // Scene 3: HSV-like rainbow
    term.clear();
    draw_title(&mut term, "Rainbow Spectrum");

    for x in 0..60 {
        // Simple rainbow using RGB interpolation
        let phase = (x as f32 / 60.0) * 6.0;
        let (r, g, b) = match phase as i32 {
            0 => (255, (phase.fract() * 255.0) as u8, 0),
            1 => ((255.0 * (1.0 - phase.fract())) as u8, 255, 0),
            2 => (0, 255, (phase.fract() * 255.0) as u8),
            3 => (0, (255.0 * (1.0 - phase.fract())) as u8, 255),
            4 => ((phase.fract() * 255.0) as u8, 0, 255),
            _ => (
                (255.0 * (1.0 - phase.fract())) as u8,
                0,
                (255.0 * (1.0 - phase.fract())) as u8,
            ),
        };

        for y in 4..12 {
            term.put_char(IVec2::new(2 + x, y), '█', Color::rgb(r, g, b), Color::BLACK);
        }
    }

    term.put_string(
        IVec2::new(4, 14),
        "Full RGB support with 16 million colors!",
        Color::WHITE,
        Color::BLACK,
    );

    term.present()?;
    thread::sleep(Duration::from_secs(2));

    // Scene 4: Dungeon palette example
    term.clear();
    draw_title(&mut term, "Roguelike Color Palette");

    let dungeon_colors = [
        ("Player (@)", Color::YELLOW, '@'),
        ("Wall (#)", Color::rgb(139, 90, 43), '#'),
        ("Floor (.)", Color::rgb(80, 80, 80), '.'),
        ("Water (~)", Color::rgb(64, 164, 223), '~'),
        ("Lava (~)", Color::rgb(255, 100, 0), '~'),
        ("Grass (\")", Color::rgb(34, 139, 34), '"'),
        ("Gold ($)", Color::rgb(255, 215, 0), '$'),
        ("Goblin (g)", Color::rgb(0, 200, 0), 'g'),
        ("Dragon (D)", Color::rgb(200, 0, 0), 'D'),
        ("Ghost (G)", Color::rgb(200, 200, 255), 'G'),
    ];

    for (i, (name, color, ch)) in dungeon_colors.iter().enumerate() {
        let y = 4 + i as i32;
        term.put_char(IVec2::new(4, y), *ch, *color, Color::BLACK);
        term.put_string(IVec2::new(6, y), name, Color::WHITE, Color::BLACK);
    }

    // Mini scene on the right
    let scene = [
        "##########",
        "#........#",
        "#.@..g...#",
        "#...~~~..#",
        "#..D$~~~~#",
        "#...\"\"...#",
        "#....G...#",
        "##########",
    ];

    for (y, line) in scene.iter().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            let color = match ch {
                '#' => Color::rgb(139, 90, 43),
                '@' => Color::YELLOW,
                'g' => Color::rgb(0, 200, 0),
                'D' => Color::rgb(200, 0, 0),
                'G' => Color::rgb(200, 200, 255),
                '~' => Color::rgb(64, 164, 223),
                '"' => Color::rgb(34, 139, 34),
                '$' => Color::rgb(255, 215, 0),
                '.' => Color::rgb(80, 80, 80),
                _ => Color::WHITE,
            };
            term.put_char(
                IVec2::new(35 + x as i32, 4 + y as i32),
                ch,
                color,
                Color::BLACK,
            );
        }
    }

    term.put_string(
        IVec2::new(4, 18),
        "Press Enter to exit...",
        Color::rgb(100, 100, 100),
        Color::BLACK,
    );

    term.present()?;

    // Wait for input
    let mut buffer = [0u8; 1];
    std::io::Read::read_exact(&mut std::io::stdin(), &mut buffer)?;

    // Cleanup
    Terminal::show_cursor()?;
    Terminal::exit_alt_screen()?;

    println!("Color palette demo complete!");
    Ok(())
}

fn draw_title(term: &mut Terminal, title: &str) {
    let x = (term.width() as i32 - title.len() as i32) / 2;
    term.put_string(IVec2::new(x, 1), title, Color::CYAN, Color::BLACK);

    // Underline
    for i in 0..title.len() {
        term.put_char(
            IVec2::new(x + i as i32, 2),
            '─',
            Color::rgb(80, 80, 80),
            Color::BLACK,
        );
    }
}
