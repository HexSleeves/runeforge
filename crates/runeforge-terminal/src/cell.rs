//! Represents a single cell in the terminal.

use runeforge_color::Color;

/// Represents a single cell in a console.
///
/// A cell contains a character, foreground color, and background color.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell {
    /// The character to display.
    pub ch: char,
    /// Foreground (text) color.
    pub fg: Color,
    /// Background color.
    pub bg: Color,
}

impl Cell {
    /// Creates a new cell with the given character and colors.
    ///
    /// # Example
    ///
    /// ```
    /// use runeforge_terminal::prelude::*;
    /// use runeforge_color::Color;
    ///
    /// let cell = Cell::new('@', Color::YELLOW, Color::BLACK);
    /// assert_eq!(cell.ch, '@');
    /// ```
    pub fn new(ch: char, fg: Color, bg: Color) -> Self {
        Self { ch, fg, bg }
    }

    /// Creates an empty cell (space with default colors).
    ///
    /// Default: white foreground, black background.
    pub fn empty() -> Self {
        Self {
            ch: ' ',
            fg: Color::WHITE,
            bg: Color::BLACK,
        }
    }

    /// Creates a cell with only a character (default colors).
    pub fn from_char(ch: char) -> Self {
        Self {
            ch,
            fg: Color::WHITE,
            bg: Color::BLACK,
        }
    }

    /// Sets the character, returning a new cell.
    pub fn with_char(mut self, ch: char) -> Self {
        self.ch = ch;
        self
    }

    /// Sets the foreground color, returning a new cell.
    pub fn with_fg(mut self, fg: Color) -> Self {
        self.fg = fg;
        self
    }

    /// Sets the background color, returning a new cell.
    pub fn with_bg(mut self, bg: Color) -> Self {
        self.bg = bg;
        self
    }
}

impl Default for Cell {
    fn default() -> Self {
        Self::empty()
    }
}
