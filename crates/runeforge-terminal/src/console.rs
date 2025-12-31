//! Console abstraction for roguelike rendering.
//!
//! This crate provides a backend-agnostic console interface that can be
//! implemented by different rendering backends (terminal, software, GPU, etc.).
//!
//! Game code can depend on this crate and work with any backend that implements
//! the `Console` trait.
//!
//! # Example
//!
//! ```
//! use runeforge_terminal::prelude::*;
//! use runeforge_color::Color;
//!
//! fn draw_to_console<C: Console>(console: &mut C) {
//!     console.clear();
//!     console.set(IVec2::new(10, 10), Cell::new('@', Color::YELLOW, Color::BLACK));
//!     console.present();
//! }
//! ```

use crate::prelude::*;
use runeforge_color::Color;

/// Trait for console rendering backends.
///
/// All rendering backends (terminal, software, GPU) implement this trait
/// to provide a consistent interface for roguelike games.
///
/// # Example Implementation
///
/// ```
/// use runeforge_terminal::prelude::*;
/// use runeforge_color::Color;
///
/// struct MyConsole {
///     width: u32,
///     height: u32,
///     cells: Vec<Cell>,
/// }
///
/// impl Console for MyConsole {
///     fn width(&self) -> u32 {
///         self.width
///     }
///
///     fn height(&self) -> u32 {
///         self.height
///     }
///
///     fn set(&mut self, pos: IVec2, cell: Cell) -> bool {
///         if pos.x < 0 || pos.y < 0 || pos.x >= self.width as i32 || pos.y >= self.height as i32 {
///             return false;
///         }
///         let idx = (pos.y as u32 * self.width + pos.x as u32) as usize;
///         if idx < self.cells.len() {
///             self.cells[idx] = cell;
///             true
///         } else {
///             false
///         }
///     }
///
///     fn get(&self, pos: IVec2) -> Option<Cell> {
///         if pos.x < 0 || pos.y < 0 || pos.x >= self.width as i32 || pos.y >= self.height as i32 {
///             return None;
///         }
///         let idx = (pos.y as u32 * self.width + pos.x as u32) as usize;
///         self.cells.get(idx).copied()
///     }
///
///     fn clear(&mut self) {
///         for cell in &mut self.cells {
///             *cell = Cell::empty();
///         }
///     }
///
///     fn present(&mut self) {
///         // Backend-specific rendering
///     }
/// }
/// ```
pub trait Console {
    /// Returns the width of the console in cells.
    fn width(&self) -> u32;

    /// Returns the height of the console in cells.
    fn height(&self) -> u32;

    /// Sets a cell at the given position.
    ///
    /// Returns `true` if the cell was set, `false` if out of bounds.
    fn set(&mut self, pos: IVec2, cell: Cell) -> bool;

    /// Gets the cell at the given position.
    ///
    /// Returns `None` if out of bounds.
    fn get(&self, pos: IVec2) -> Option<Cell>;

    /// Clears the console to empty cells.
    fn clear(&mut self);

    /// Presents the console to the screen/output.
    ///
    /// This may flush to terminal, swap buffers for GPU, or save to file
    /// for software rendering.
    fn present(&mut self);

    // --- Convenience methods with default implementations ---

    /// Returns the console dimensions as (width, height).
    fn size(&self) -> (u32, u32) {
        (self.width(), self.height())
    }

    /// Checks if a position is within bounds.
    fn in_bounds(&self, pos: IVec2) -> bool {
        pos.x >= 0 && pos.y >= 0 && pos.x < self.width() as i32 && pos.y < self.height() as i32
    }

    /// Clears the console with a specific background color.
    fn clear_with_color(&mut self, bg: Color) {
        for y in 0..self.height() {
            for x in 0..self.width() {
                let pos = IVec2::new(x as i32, y as i32);
                self.set(pos, Cell::new(' ', Color::WHITE, bg));
            }
        }
    }

    /// Sets only the character at a position, preserving colors.
    fn set_char(&mut self, pos: IVec2, ch: char) -> bool {
        if let Some(cell) = self.get(pos) {
            self.set(pos, cell.with_char(ch))
        } else {
            false
        }
    }

    /// Sets only the foreground color at a position.
    fn set_fg(&mut self, pos: IVec2, fg: Color) -> bool {
        if let Some(cell) = self.get(pos) {
            self.set(pos, cell.with_fg(fg))
        } else {
            false
        }
    }

    /// Sets only the background color at a position.
    fn set_bg(&mut self, pos: IVec2, bg: Color) -> bool {
        if let Some(cell) = self.get(pos) {
            self.set(pos, cell.with_bg(bg))
        } else {
            false
        }
    }

    /// Draws a character with foreground and background colors.
    fn draw_char(&mut self, pos: IVec2, ch: char, fg: Color, bg: Color) -> bool {
        self.set(pos, Cell::new(ch, fg, bg))
    }

    /// Draws a string horizontally starting at the given position.
    ///
    /// Returns the number of characters actually drawn.
    fn draw_string(&mut self, pos: IVec2, text: &str, fg: Color, bg: Color) -> usize {
        let mut count = 0;
        for (i, ch) in text.chars().enumerate() {
            let x = pos.x + i as i32;
            if x >= self.width() as i32 {
                break;
            }
            if self.draw_char(IVec2::new(x, pos.y), ch, fg, bg) {
                count += 1;
            }
        }
        count
    }

    /// Draws a vertical line from start to end with the given character and colors.
    fn draw_vline(
        &mut self,
        x: i32,
        y_start: i32,
        y_end: i32,
        ch: char,
        fg: Color,
        bg: Color,
    ) -> usize {
        let mut count = 0;
        let (min_y, max_y) = if y_start <= y_end {
            (y_start, y_end)
        } else {
            (y_end, y_start)
        };

        for y in min_y..=max_y {
            if self.draw_char(IVec2::new(x, y), ch, fg, bg) {
                count += 1;
            }
        }
        count
    }

    /// Draws a horizontal line from start to end with the given character and colors.
    fn draw_hline(
        &mut self,
        y: i32,
        x_start: i32,
        x_end: i32,
        ch: char,
        fg: Color,
        bg: Color,
    ) -> usize {
        let mut count = 0;
        let (min_x, max_x) = if x_start <= x_end {
            (x_start, x_end)
        } else {
            (x_end, x_start)
        };

        for x in min_x..=max_x {
            if self.draw_char(IVec2::new(x, y), ch, fg, bg) {
                count += 1;
            }
        }
        count
    }

    /// Fills a rectangular region with a cell.
    fn fill_rect(&mut self, x: i32, y: i32, width: u32, height: u32, cell: Cell) -> usize {
        let mut count = 0;
        for dy in 0..height as i32 {
            for dx in 0..width as i32 {
                if self.set(IVec2::new(x + dx, y + dy), cell) {
                    count += 1;
                }
            }
        }
        count
    }

    /// Draws a box with the given corners and edges.
    ///
    /// # Arguments
    ///
    /// * `x`, `y` - Top-left corner position
    /// * `width`, `height` - Box dimensions
    /// * `fg`, `bg` - Colors
    /// * `double` - Use double-line box drawing characters if true
    #[allow(clippy::too_many_arguments)]
    fn draw_box(
        &mut self,
        x: i32,
        y: i32,
        width: u32,
        height: u32,
        fg: Color,
        bg: Color,
        double: bool,
    ) {
        if width < 2 || height < 2 {
            return;
        }

        let (tl, tr, bl, br, h, v) = if double {
            ('╔', '╗', '╚', '╝', '═', '║')
        } else {
            ('┌', '┐', '└', '┘', '─', '│')
        };

        // Corners
        self.draw_char(IVec2::new(x, y), tl, fg, bg);
        self.draw_char(IVec2::new(x + width as i32 - 1, y), tr, fg, bg);
        self.draw_char(IVec2::new(x, y + height as i32 - 1), bl, fg, bg);
        self.draw_char(
            IVec2::new(x + width as i32 - 1, y + height as i32 - 1),
            br,
            fg,
            bg,
        );

        // Horizontal lines
        self.draw_hline(y, x + 1, x + width as i32 - 2, h, fg, bg);
        self.draw_hline(
            y + height as i32 - 1,
            x + 1,
            x + width as i32 - 2,
            h,
            fg,
            bg,
        );

        // Vertical lines
        self.draw_vline(x, y + 1, y + height as i32 - 2, v, fg, bg);
        self.draw_vline(
            x + width as i32 - 1,
            y + 1,
            y + height as i32 - 2,
            v,
            fg,
            bg,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Simple test console implementation
    struct TestConsole {
        width: u32,
        height: u32,
        cells: Vec<Cell>,
    }

    impl TestConsole {
        fn new(width: u32, height: u32) -> Self {
            Self {
                width,
                height,
                cells: vec![Cell::empty(); (width * height) as usize],
            }
        }
    }

    impl Console for TestConsole {
        fn width(&self) -> u32 {
            self.width
        }

        fn height(&self) -> u32 {
            self.height
        }

        fn set(&mut self, pos: IVec2, cell: Cell) -> bool {
            if !self.in_bounds(pos) {
                return false;
            }
            let idx = (pos.y as u32 * self.width + pos.x as u32) as usize;
            self.cells[idx] = cell;
            true
        }

        fn get(&self, pos: IVec2) -> Option<Cell> {
            if !self.in_bounds(pos) {
                return None;
            }
            let idx = (pos.y as u32 * self.width + pos.x as u32) as usize;
            self.cells.get(idx).copied()
        }

        fn clear(&mut self) {
            for cell in &mut self.cells {
                *cell = Cell::empty();
            }
        }

        fn present(&mut self) {
            // No-op for testing
        }
    }

    #[test]
    fn test_cell_creation() {
        let cell = Cell::new('@', Color::RED, Color::BLUE);
        assert_eq!(cell.ch, '@');
        assert_eq!(cell.fg, Color::RED);
        assert_eq!(cell.bg, Color::BLUE);
    }

    #[test]
    fn test_console_basic_ops() {
        let mut console = TestConsole::new(10, 10);
        assert_eq!(console.width(), 10);
        assert_eq!(console.height(), 10);

        let pos = IVec2::new(5, 5);
        let cell = Cell::new('@', Color::YELLOW, Color::BLACK);

        assert!(console.set(pos, cell));
        assert_eq!(console.get(pos), Some(cell));
    }

    #[test]
    fn test_console_bounds_checking() {
        let console = TestConsole::new(10, 10);

        // In bounds
        assert!(console.in_bounds(IVec2::new(0, 0)));
        assert!(console.in_bounds(IVec2::new(9, 9)));

        // Out of bounds
        assert!(!console.in_bounds(IVec2::new(-1, 0)));
        assert!(!console.in_bounds(IVec2::new(0, -1)));
        assert!(!console.in_bounds(IVec2::new(10, 0)));
        assert!(!console.in_bounds(IVec2::new(0, 10)));
    }

    #[test]
    fn test_console_draw_string() {
        let mut console = TestConsole::new(20, 10);
        let count = console.draw_string(IVec2::new(5, 5), "Hello", Color::WHITE, Color::BLACK);

        assert_eq!(count, 5);
        assert_eq!(console.get(IVec2::new(5, 5)).unwrap().ch, 'H');
        assert_eq!(console.get(IVec2::new(9, 5)).unwrap().ch, 'o');
    }

    #[test]
    fn test_console_draw_box() {
        let mut console = TestConsole::new(20, 10);
        console.draw_box(5, 5, 10, 5, Color::WHITE, Color::BLACK, false);

        // Check corners
        assert_eq!(console.get(IVec2::new(5, 5)).unwrap().ch, '┌');
        assert_eq!(console.get(IVec2::new(14, 5)).unwrap().ch, '┐');
        assert_eq!(console.get(IVec2::new(5, 9)).unwrap().ch, '└');
        assert_eq!(console.get(IVec2::new(14, 9)).unwrap().ch, '┘');

        // Check edges
        assert_eq!(console.get(IVec2::new(6, 5)).unwrap().ch, '─');
        assert_eq!(console.get(IVec2::new(5, 6)).unwrap().ch, '│');
    }

    #[test]
    fn test_console_clear() {
        let mut console = TestConsole::new(10, 10);
        console.draw_string(IVec2::new(5, 5), "Test", Color::WHITE, Color::BLACK);
        console.clear();

        assert_eq!(console.get(IVec2::new(5, 5)).unwrap(), Cell::empty());
    }
}
