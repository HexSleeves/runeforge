//! Basic Terminal Usage Demo
//!
//! This example demonstrates the core features of the runeforge-terminal crate:
//! - Creating a terminal buffer
//! - Drawing characters and strings
//! - Using colors
//! - Presenting to stdout

use runeforge_color::Color;
use runeforge_terminal::prelude::{IVec2, Terminal};
use std::io;

fn main() -> io::Result<()> {
    // Create a terminal buffer (80x24 is classic terminal size)
    let mut term = Terminal::new(60, 20);

    // Set up the alternate screen (preserves user's terminal)
    Terminal::enter_alt_screen()?;
    Terminal::hide_cursor()?;

    // Clear and draw a border
    term.clear();
    draw_border(&mut term);

    // Draw title
    term.put_string(
        IVec2::new(20, 1),
        "RUNEFORGE TERMINAL DEMO",
        Color::YELLOW,
        Color::BLACK,
    );

    // Draw various colored text
    term.put_string(IVec2::new(5, 4), "Colors:", Color::WHITE, Color::BLACK);
    term.put_string(IVec2::new(15, 4), "Red", Color::RED, Color::BLACK);
    term.put_string(IVec2::new(20, 4), "Green", Color::GREEN, Color::BLACK);
    term.put_string(IVec2::new(27, 4), "Blue", Color::BLUE, Color::BLACK);
    term.put_string(IVec2::new(33, 4), "Yellow", Color::YELLOW, Color::BLACK);
    term.put_string(IVec2::new(41, 4), "Cyan", Color::CYAN, Color::BLACK);
    term.put_string(IVec2::new(47, 4), "Magenta", Color::MAGENTA, Color::BLACK);

    // Draw characters individually
    term.put_string(IVec2::new(5, 6), "Characters:", Color::WHITE, Color::BLACK);
    term.put_char(IVec2::new(18, 6), '@', Color::YELLOW, Color::BLACK);
    term.put_char(
        IVec2::new(20, 6),
        '#',
        Color::rgb(139, 90, 43),
        Color::BLACK,
    );
    term.put_char(IVec2::new(22, 6), '%', Color::GREEN, Color::BLACK);
    term.put_char(IVec2::new(24, 6), '!', Color::RED, Color::BLACK);
    term.put_char(IVec2::new(26, 6), '?', Color::CYAN, Color::BLACK);
    term.put_char(
        IVec2::new(28, 6),
        '$',
        Color::rgb(255, 215, 0),
        Color::BLACK,
    );

    // Draw with background colors
    term.put_string(IVec2::new(5, 8), "Backgrounds:", Color::WHITE, Color::BLACK);
    term.put_string(
        IVec2::new(18, 8),
        " Dark ",
        Color::WHITE,
        Color::rgb(50, 50, 50),
    );
    term.put_string(
        IVec2::new(25, 8),
        " Blue ",
        Color::WHITE,
        Color::rgb(0, 0, 100),
    );
    term.put_string(
        IVec2::new(32, 8),
        " Green ",
        Color::WHITE,
        Color::rgb(0, 80, 0),
    );
    term.put_string(
        IVec2::new(40, 8),
        " Red ",
        Color::WHITE,
        Color::rgb(100, 0, 0),
    );

    // Draw a simple dungeon mockup
    term.put_string(
        IVec2::new(5, 10),
        "Mini dungeon:",
        Color::WHITE,
        Color::BLACK,
    );

    let dungeon = ["########", "#......#", "#.@..g.#", "#......#", "###..###"];

    for (y, line) in dungeon.iter().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            let color = match ch {
                '#' => Color::rgb(139, 90, 43),
                '@' => Color::YELLOW,
                'g' => Color::GREEN,
                '.' => Color::rgb(100, 100, 100),
                _ => Color::WHITE,
            };
            term.put_char(
                IVec2::new(18 + x as i32, 10 + y as i32),
                ch,
                color,
                Color::BLACK,
            );
        }
    }

    // Instructions
    term.put_string(
        IVec2::new(5, 17),
        "Press Enter to exit...",
        Color::rgb(128, 128, 128),
        Color::BLACK,
    );

    // Present to screen
    term.present()?;

    // Wait for input
    let mut buffer = [0u8; 1];
    std::io::Read::read_exact(&mut std::io::stdin(), &mut buffer)?;

    // Cleanup
    Terminal::show_cursor()?;
    Terminal::exit_alt_screen()?;

    println!("Terminal demo complete!");
    Ok(())
}

fn draw_border(term: &mut Terminal) {
    let w = term.width() as i32;
    let h = term.height() as i32;
    let border_color = Color::rgb(100, 100, 100);

    // Top and bottom
    for x in 0..w {
        term.put_char(IVec2::new(x, 0), '─', border_color, Color::BLACK);
        term.put_char(IVec2::new(x, h - 1), '─', border_color, Color::BLACK);
    }

    // Left and right
    for y in 0..h {
        term.put_char(IVec2::new(0, y), '│', border_color, Color::BLACK);
        term.put_char(IVec2::new(w - 1, y), '│', border_color, Color::BLACK);
    }

    // Corners
    term.put_char(IVec2::new(0, 0), '┌', border_color, Color::BLACK);
    term.put_char(IVec2::new(w - 1, 0), '┐', border_color, Color::BLACK);
    term.put_char(IVec2::new(0, h - 1), '└', border_color, Color::BLACK);
    term.put_char(IVec2::new(w - 1, h - 1), '┘', border_color, Color::BLACK);
}
