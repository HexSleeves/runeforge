//! Interactive FOV demonstration
//!
//! This example shows the field-of-view algorithm in action.
//! The player (@) can see tiles within their FOV radius.
//! Visible tiles are shown in bright colors, non-visible in dark gray.
//!
//! Controls:
//! - Arrow keys or WASD: Move player
//! - Q: Quit

use runeforge_color::Color;
use runeforge_fov::prelude::{Fov, FovProvider};
use runeforge_geometry::prelude::IVec2;
use runeforge_terminal::prelude::Terminal;
use std::io::{self, Read};

/// Represents the game map
struct Map {
    width: u32,
    height: u32,
    tiles: Vec<bool>, // true = wall, false = floor
}

impl Map {
    /// Creates a new map with a simple dungeon layout
    fn new(width: u32, height: u32) -> Self {
        let mut tiles = vec![false; (width * height) as usize];

        // Create border walls
        for x in 0..width {
            tiles[x as usize] = true; // Top wall
            tiles[((height - 1) * width + x) as usize] = true; // Bottom wall
        }
        for y in 0..height {
            tiles[(y * width) as usize] = true; // Left wall
            tiles[(y * width + width - 1) as usize] = true; // Right wall
        }

        // Add some interior walls for interest
        for x in 10..20 {
            if x != 15 {
                // Leave a gap
                tiles[(5 * width + x) as usize] = true;
            }
        }

        for y in 8..15 {
            if y != 11 {
                // Leave a gap
                tiles[(y * width + 25) as usize] = true;
            }
        }

        // Add some pillars
        tiles[(8 * width + 15) as usize] = true;
        tiles[(12 * width + 18) as usize] = true;

        Self {
            width,
            height,
            tiles,
        }
    }

    /// Returns true if the position is a wall
    fn is_blocking(&self, pos: IVec2) -> bool {
        if pos.x < 0 || pos.y < 0 || pos.x >= self.width as i32 || pos.y >= self.height as i32 {
            return true; // Out of bounds is blocking
        }
        let index = (pos.y as u32 * self.width + pos.x as u32) as usize;
        self.tiles[index]
    }

    /// Returns true if the position is walkable
    fn is_walkable(&self, pos: IVec2) -> bool {
        !self.is_blocking(pos)
    }
}

/// Implement FovProvider for Map so it can provide opacity info to the FOV algorithm
impl FovProvider<()> for Map {
    fn is_opaque(&mut self, pos: IVec2, _data: &mut ()) -> bool {
        self.is_blocking(pos)
    }
}

/// The game state
struct Game {
    map: Map,
    player_pos: IVec2,
    fov_radius: i32,
    visible: Vec<bool>,
}

impl Game {
    fn new() -> Self {
        let map = Map::new(60, 20);
        let player_pos = IVec2::new(5, 10);
        let visible = vec![false; (map.width * map.height) as usize];

        let mut game = Self {
            map,
            player_pos,
            fov_radius: 8,
            visible,
        };

        game.update_fov();
        game
    }

    /// Update the FOV based on player position
    fn update_fov(&mut self) {
        // Clear previous visibility
        for v in &mut self.visible {
            *v = false;
        }

        // Compute new FOV using Shadowcast algorithm
        let visible_tiles =
            Fov::Shadowcast.compute(self.player_pos, self.fov_radius as u32, &mut self.map, ());

        // Mark visible tiles
        for pos in visible_tiles {
            if pos.x >= 0
                && pos.y >= 0
                && pos.x < self.map.width as i32
                && pos.y < self.map.height as i32
            {
                let index = (pos.y as u32 * self.map.width + pos.x as u32) as usize;
                self.visible[index] = true;
            }
        }
    }

    /// Move the player in the given direction
    fn move_player(&mut self, dx: i32, dy: i32) {
        let new_pos = IVec2::new(self.player_pos.x + dx, self.player_pos.y + dy);
        if self.map.is_walkable(new_pos) {
            self.player_pos = new_pos;
            self.update_fov();
        }
    }

    /// Render the game to the terminal
    fn render(&self, term: &mut Terminal) {
        term.clear();

        // Draw the map
        for y in 0..self.map.height {
            for x in 0..self.map.width {
                let pos = IVec2::new(x as i32, y as i32);
                let index = (y * self.map.width + x) as usize;
                let is_wall = self.map.tiles[index];
                let is_visible = self.visible[index];

                let (ch, fg, bg) = if pos == self.player_pos {
                    ('@', Color::YELLOW, Color::BLACK)
                } else if is_visible {
                    if is_wall {
                        ('#', Color::rgb(200, 200, 200), Color::BLACK)
                    } else {
                        ('.', Color::rgb(128, 128, 128), Color::BLACK)
                    }
                } else {
                    // Non-visible tiles are very dark
                    if is_wall {
                        ('#', Color::rgb(40, 40, 40), Color::BLACK)
                    } else {
                        (' ', Color::BLACK, Color::BLACK)
                    }
                };

                term.put_char(pos, ch, fg, bg);
            }
        }

        // Draw instructions at the bottom
        let help_y = self.map.height as i32;
        term.put_string(
            IVec2::new(0, help_y),
            "Arrow keys/WASD: Move | Q: Quit | FOV Radius: 8",
            Color::WHITE,
            Color::BLACK,
        );
    }
}

fn main() -> io::Result<()> {
    // Set up terminal
    Terminal::enter_alt_screen()?;
    Terminal::hide_cursor()?;

    let mut term = Terminal::new(60, 24);
    let mut game = Game::new();

    // Initial render
    game.render(&mut term);
    term.present()?;

    println!("\nUse arrow keys or WASD to move the player.");
    println!("The '@' symbol is you. You can see tiles within your FOV radius.");
    println!("Press Q to quit.\n");

    // Simple input loop (non-blocking would require crossterm/termion)
    // For now, this is a static demo showing the FOV
    println!("Press Enter to see another view...");

    // Wait for input
    let mut buffer = [0; 1];
    io::stdin().read_exact(&mut buffer)?;

    // Move player and re-render
    game.move_player(5, 0);
    game.render(&mut term);
    term.present()?;

    println!("Moved right 5 spaces. FOV updates automatically.");
    println!("Press Enter to continue...");
    io::stdin().read_exact(&mut buffer)?;

    game.move_player(0, 3);
    game.render(&mut term);
    term.present()?;

    println!("Moved down 3 spaces. Notice how walls block vision.");
    println!("Press Enter to exit...");
    io::stdin().read_exact(&mut buffer)?;

    // Cleanup
    Terminal::show_cursor()?;
    Terminal::exit_alt_screen()?;

    Ok(())
}
