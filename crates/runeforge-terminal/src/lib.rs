//! Terminal rendering backend for roguelike games.
//!
//! This crate provides a simple ANSI terminal renderer for quick prototyping
//! and debugging of roguelike algorithms.
//!
//! # Example
//!
//! ```no_run
//! use runeforge_terminal::{Terminal, Cell};
//! use runeforge_color::Color;
//! use runeforge_geometry::Point;
//!
//! let mut term = Terminal::new(80, 24);
//! term.clear();
//! term.put_char(Point::new(10, 10), '@', Color::YELLOW, Color::BLACK);
//! term.present();
//! ```

#![deny(missing_docs)]

use runeforge_color::Color;
use runeforge_geometry::Point;
use std::io::{self, Write};

/// Represents a single cell in the terminal.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell {
    /// The character to display
    pub ch: char,
    /// Foreground color
    pub fg: Color,
    /// Background color
    pub bg: Color,
}

impl Cell {
    /// Creates a new cell with the given character and colors.
    pub fn new(ch: char, fg: Color, bg: Color) -> Self {
        Self { ch, fg, bg }
    }

    /// Creates an empty cell (space with black background).
    pub fn empty() -> Self {
        Self {
            ch: ' ',
            fg: Color::WHITE,
            bg: Color::BLACK,
        }
    }
}

impl Default for Cell {
    fn default() -> Self {
        Self::empty()
    }
}

/// A terminal renderer using ANSI escape codes.
pub struct Terminal {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
}

impl Terminal {
    /// Creates a new terminal with the given dimensions.
    ///
    /// # Example
    ///
    /// ```
    /// use runeforge_terminal::Terminal;
    ///
    /// let term = Terminal::new(80, 24);
    /// ```
    pub fn new(width: u32, height: u32) -> Self {
        let cells = vec![Cell::empty(); (width * height) as usize];
        Self {
            width,
            height,
            cells,
        }
    }

    /// Returns the width of the terminal.
    pub fn width(&self) -> u32 {
        self.width
    }

    /// Returns the height of the terminal.
    pub fn height(&self) -> u32 {
        self.height
    }

    /// Clears the terminal to empty cells.
    pub fn clear(&mut self) {
        for cell in &mut self.cells {
            *cell = Cell::empty();
        }
    }

    /// Clears the terminal with a specific background color.
    pub fn clear_with_color(&mut self, bg: Color) {
        for cell in &mut self.cells {
            *cell = Cell::new(' ', Color::WHITE, bg);
        }
    }

    /// Puts a character at the given position.
    ///
    /// Returns `false` if the position is out of bounds.
    pub fn put_char(&mut self, pos: Point, ch: char, fg: Color, bg: Color) -> bool {
        if let Some(cell) = self.get_cell_mut(pos) {
            *cell = Cell::new(ch, fg, bg);
            true
        } else {
            false
        }
    }

    /// Puts a string at the given position, advancing horizontally.
    ///
    /// Returns the number of characters actually written.
    pub fn put_string(&mut self, pos: Point, text: &str, fg: Color, bg: Color) -> usize {
        let mut count = 0;
        for (i, ch) in text.chars().enumerate() {
            let x = pos.x + i as i32;
            if x >= self.width as i32 {
                break;
            }
            if self.put_char(Point::new(x, pos.y), ch, fg, bg) {
                count += 1;
            }
        }
        count
    }

    /// Gets a mutable reference to the cell at the given position.
    fn get_cell_mut(&mut self, pos: Point) -> Option<&mut Cell> {
        if pos.x < 0 || pos.y < 0 || pos.x >= self.width as i32 || pos.y >= self.height as i32 {
            return None;
        }
        let index = (pos.y as u32 * self.width + pos.x as u32) as usize;
        self.cells.get_mut(index)
    }

    /// Gets a reference to the cell at the given position.
    pub fn get_cell(&self, pos: Point) -> Option<&Cell> {
        if pos.x < 0 || pos.y < 0 || pos.x >= self.width as i32 || pos.y >= self.height as i32 {
            return None;
        }
        let index = (pos.y as u32 * self.width + pos.x as u32) as usize;
        self.cells.get(index)
    }

    /// Presents the terminal to stdout using ANSI escape codes.
    ///
    /// This clears the screen and redraws all cells.
    pub fn present(&self) -> io::Result<()> {
        let mut stdout = io::stdout();

        // Clear screen and move cursor to top-left
        write!(stdout, "\x1b[2J\x1b[H")?;

        let mut last_fg = Color::WHITE;
        let mut last_bg = Color::BLACK;

        for y in 0..self.height {
            for x in 0..self.width {
                let pos = Point::new(x as i32, y as i32);
                let cell = self.get_cell(pos).unwrap();

                // Only change colors if they're different from the last cell
                if cell.fg != last_fg || cell.bg != last_bg {
                    write!(
                        stdout,
                        "\x1b[38;2;{};{};{}m\x1b[48;2;{};{};{}m",
                        cell.fg.r, cell.fg.g, cell.fg.b, cell.bg.r, cell.bg.g, cell.bg.b
                    )?;
                    last_fg = cell.fg;
                    last_bg = cell.bg;
                }

                write!(stdout, "{}", cell.ch)?;
            }
            writeln!(stdout)?;
        }

        // Reset colors
        write!(stdout, "\x1b[0m")?;
        stdout.flush()?;

        Ok(())
    }

    /// Hides the cursor.
    pub fn hide_cursor() -> io::Result<()> {
        print!("\x1b[?25l");
        io::stdout().flush()
    }

    /// Shows the cursor.
    pub fn show_cursor() -> io::Result<()> {
        print!("\x1b[?25h");
        io::stdout().flush()
    }

    /// Enables alternative screen buffer.
    ///
    /// This saves the current terminal state and switches to a new screen.
    pub fn enter_alt_screen() -> io::Result<()> {
        print!("\x1b[?1049h");
        io::stdout().flush()
    }

    /// Disables alternative screen buffer.
    ///
    /// This restores the previous terminal state.
    pub fn exit_alt_screen() -> io::Result<()> {
        print!("\x1b[?1049l");
        io::stdout().flush()
    }
}

impl Drop for Terminal {
    fn drop(&mut self) {
        // Ensure cursor is visible when terminal is dropped
        let _ = Self::show_cursor();
        let _ = Self::exit_alt_screen();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_terminal_creation() {
        let term = Terminal::new(80, 24);
        assert_eq!(term.width(), 80);
        assert_eq!(term.height(), 24);
    }

    #[test]
    fn test_put_char() {
        let mut term = Terminal::new(10, 10);
        assert!(term.put_char(Point::new(5, 5), '@', Color::YELLOW, Color::BLACK));
        let cell = term.get_cell(Point::new(5, 5)).unwrap();
        assert_eq!(cell.ch, '@');
        assert_eq!(cell.fg, Color::YELLOW);
    }

    #[test]
    fn test_put_string() {
        let mut term = Terminal::new(20, 10);
        let count = term.put_string(Point::new(0, 0), "Hello", Color::GREEN, Color::BLACK);
        assert_eq!(count, 5);

        assert_eq!(term.get_cell(Point::new(0, 0)).unwrap().ch, 'H');
        assert_eq!(term.get_cell(Point::new(4, 0)).unwrap().ch, 'o');
    }

    #[test]
    fn test_out_of_bounds() {
        let mut term = Terminal::new(10, 10);
        assert!(!term.put_char(Point::new(100, 100), '@', Color::WHITE, Color::BLACK));
        assert!(term.get_cell(Point::new(100, 100)).is_none());
    }

    #[test]
    fn test_clear() {
        let mut term = Terminal::new(10, 10);
        term.put_char(Point::new(5, 5), '@', Color::YELLOW, Color::BLACK);
        term.clear();
        let cell = term.get_cell(Point::new(5, 5)).unwrap();
        assert_eq!(cell.ch, ' ');
    }
}
