//! A complete playable roguelike demo integrating all Runeforge systems.
//!
//! This demo showcases:
//! - Procedural map generation (BSP dungeons)
//! - Field of view calculation
//! - Player movement and exploration
//! - Pixel-based rendering with winit + pixels
//!
//! Controls:
//! - Arrow keys, WASD, or Vi-keys (hjkl) to move
//! - ESC to quit

use anyhow::Result;
use image::GenericImageView;
use pixels::{Pixels, SurfaceTexture};
use runeforge_algorithms::{BspConfig, DungeonGenerator};
use runeforge_fov::compute_fov;
use runeforge_geometry::Point;
use runeforge_input::{InputEvent, InputMap, InputState, VirtualKey};
use runeforge_random::Rng;
use std::collections::HashSet;
use winit::{
    application::ApplicationHandler,
    event::{ElementState, KeyEvent, WindowEvent},
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    keyboard::{KeyCode, PhysicalKey},
    window::Window,
};

const MAP_WIDTH: u32 = 80;
const MAP_HEIGHT: u32 = 40;
const TILE_SIZE: u32 = 8; // Each tile is 8x8 pixels (matching tileset)
const BUFFER_WIDTH: u32 = MAP_WIDTH * TILE_SIZE;
const BUFFER_HEIGHT: u32 = MAP_HEIGHT * TILE_SIZE;
const SCALE_FACTOR: u32 = 3; // Window scale factor for larger display
const FOV_RADIUS: i32 = 8;

/// Tileset for character rendering
struct Tileset {
    /// RGBA pixel data from the tileset image
    pixels: Vec<u8>,
    /// Width of each character in pixels
    char_width: u32,
    /// Height of each character in pixels
    char_height: u32,
    /// Total width of the tileset image
    width: u32,
}

impl Tileset {
    /// Load the tileset from a PNG file
    fn load() -> Result<Self> {
        let img = image::open(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/assets/terminal8x8_gs_ro.png"
        ))?;
        let rgba = img.to_rgba8();
        let (width, _height) = img.dimensions();

        Ok(Tileset {
            pixels: rgba.into_raw(),
            char_width: 8,
            char_height: 8,
            width,
        })
    }

    /// Get the pixel at (x, y) in the tileset
    fn get_pixel(&self, x: u32, y: u32) -> [u8; 4] {
        let idx = ((y * self.width + x) * 4) as usize;
        [
            self.pixels[idx],
            self.pixels[idx + 1],
            self.pixels[idx + 2],
            self.pixels[idx + 3],
        ]
    }
}

/// Represents a tile in the game world
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Wall,
    Floor,
}

/// Game state
struct Game {
    /// The dungeon map
    map: Vec<Vec<Tile>>,
    /// Player's current position
    player_pos: Point,
    /// Tiles currently visible to the player
    visible_tiles: HashSet<Point>,
    /// Tiles that have been explored (seen before)
    explored_tiles: HashSet<Point>,
    /// Map dimensions
    width: u32,
    height: u32,
    /// Random number generator
    #[allow(dead_code)]
    rng: Rng,
}

impl Game {
    /// Create a new game with a procedurally generated dungeon
    fn new() -> Self {
        let mut rng = Rng::new();

        // Generate a dungeon using BSP algorithm
        let config = BspConfig::default()
            .with_min_room_size(4, 4)
            .with_max_depth(5)
            .with_room_padding(1);

        let dungeon = DungeonGenerator::generate(MAP_WIDTH, MAP_HEIGHT, &config, &mut rng);

        // Convert dungeon to tile map
        let mut map = vec![vec![Tile::Wall; MAP_WIDTH as usize]; MAP_HEIGHT as usize];
        for y in 0..MAP_HEIGHT {
            for x in 0..MAP_WIDTH {
                if dungeon.is_floor(x as i32, y as i32) {
                    map[y as usize][x as usize] = Tile::Floor;
                }
            }
        }

        // Place player in the first room's center
        let first_room = &dungeon.rooms()[0];
        let player_pos = Point::new(
            first_room.x + (first_room.width / 2) as i32,
            first_room.y + (first_room.height / 2) as i32,
        );

        let mut game = Game {
            map,
            player_pos,
            visible_tiles: HashSet::new(),
            explored_tiles: HashSet::new(),
            width: MAP_WIDTH,
            height: MAP_HEIGHT,
            rng,
        };

        // Compute initial FOV
        game.update_fov();

        game
    }

    /// Check if a position is within map bounds
    fn in_bounds(&self, pos: Point) -> bool {
        pos.x >= 0 && pos.x < self.width as i32 && pos.y >= 0 && pos.y < self.height as i32
    }

    /// Check if a tile is walkable
    fn is_walkable(&self, pos: Point) -> bool {
        if !self.in_bounds(pos) {
            return false;
        }
        self.map[pos.y as usize][pos.x as usize] == Tile::Floor
    }

    /// Move the player by a delta
    fn move_player(&mut self, dx: i32, dy: i32) {
        let new_pos = Point::new(self.player_pos.x + dx, self.player_pos.y + dy);

        if self.is_walkable(new_pos) {
            self.player_pos = new_pos;
            self.update_fov();
        }
    }

    /// Update the field of view from the player's position
    fn update_fov(&mut self) {
        self.visible_tiles.clear();

        // Create a local copy of the map for the is_blocking closure
        let map = self.map.clone();
        let width = self.width;
        let height = self.height;

        let is_blocking_fn = |pos: Point| -> bool {
            if pos.x < 0 || pos.y < 0 || pos.x >= width as i32 || pos.y >= height as i32 {
                return true;
            }
            map[pos.y as usize][pos.x as usize] == Tile::Wall
        };

        let visible_tiles = &mut self.visible_tiles;
        let explored_tiles = &mut self.explored_tiles;

        compute_fov(self.player_pos, FOV_RADIUS, &is_blocking_fn, &mut |pos| {
            if pos.x >= 0 && pos.y >= 0 && pos.x < width as i32 && pos.y < height as i32 {
                visible_tiles.insert(pos);
                explored_tiles.insert(pos);
            }
        });
    }

    /// Handle a virtual key press
    fn handle_virtual_key(&mut self, vkey: VirtualKey) -> bool {
        match vkey {
            VirtualKey::Quit => return false,
            VirtualKey::Move(dir) => {
                let (dx, dy) = dir.to_delta();
                self.move_player(dx, dy);
            }
            _ => {}
        }
        true
    }

    /// Render the game to the pixel buffer
    fn render(&self, frame: &mut [u8], tileset: &Tileset) {
        // Clear to black
        frame.fill(0);

        // Render each tile
        for y in 0..self.height {
            for x in 0..self.width {
                let pos = Point::new(x as i32, y as i32);

                // Determine character and tint based on visibility
                let (char_code, tint) = if pos == self.player_pos {
                    // Player - '@' (0x40) with yellow tint
                    (0x40u8, [1.0, 1.0, 0.0])
                } else if self.visible_tiles.contains(&pos) {
                    // Visible tile - full brightness
                    let ch = match self.map[y as usize][x as usize] {
                        Tile::Wall => 0x23,  // '#'
                        Tile::Floor => 0x2E, // '.'
                    };
                    (ch, [0.8, 0.8, 0.8])
                } else if self.explored_tiles.contains(&pos) {
                    // Explored but not visible - dimmed
                    let ch = match self.map[y as usize][x as usize] {
                        Tile::Wall => 0x23,  // '#'
                        Tile::Floor => 0x2E, // '.'
                    };
                    (ch, [0.4, 0.4, 0.4])
                } else {
                    // Unexplored - don't draw anything
                    continue;
                };

                // Draw the glyph with tint
                self.draw_glyph(frame, tileset, x, y, char_code, tint);
            }
        }
    }

    /// Draw a single character glyph from the tileset to the pixel buffer
    fn draw_glyph(
        &self,
        frame: &mut [u8],
        tileset: &Tileset,
        tile_x: u32,
        tile_y: u32,
        char_code: u8,
        tint: [f32; 3],
    ) {
        // Calculate position in tileset (CP437 layout: 16x16 grid)
        let glyph_col = (char_code % 16) as u32;
        let glyph_row = (char_code / 16) as u32;
        let glyph_x = glyph_col * tileset.char_width;
        let glyph_y = glyph_row * tileset.char_height;

        // Calculate position in framebuffer
        let screen_x = tile_x * TILE_SIZE;
        let screen_y = tile_y * TILE_SIZE;

        // Copy pixels from tileset to framebuffer with tint
        for py in 0..tileset.char_height {
            for px in 0..tileset.char_width {
                let src_x = glyph_x + px;
                let src_y = glyph_y + py;
                let dst_x = screen_x + px;
                let dst_y = screen_y + py;

                if dst_x < BUFFER_WIDTH && dst_y < BUFFER_HEIGHT {
                    let pixel = tileset.get_pixel(src_x, src_y);
                    let idx = ((dst_y * BUFFER_WIDTH + dst_x) * 4) as usize;

                    if idx + 3 < frame.len() {
                        // Apply tint to greyscale tileset
                        let brightness = pixel[0] as f32 / 255.0;
                        frame[idx] = (brightness * tint[0] * 255.0) as u8; // R
                        frame[idx + 1] = (brightness * tint[1] * 255.0) as u8; // G
                        frame[idx + 2] = (brightness * tint[2] * 255.0) as u8; // B
                        frame[idx + 3] = pixel[3]; // A
                    }
                }
            }
        }
    }
}

/// Application state
struct App {
    window: Option<&'static Window>,
    pixels: Option<Pixels<'static>>,
    game: Game,
    tileset: Tileset,
    input_map: InputMap,
    input_state: InputState,
}

impl Default for App {
    fn default() -> Self {
        Self {
            window: None,
            pixels: None,
            game: Game::new(),
            tileset: Tileset::load().expect("Failed to load tileset"),
            input_map: InputMap::roguelike_default(),
            input_state: InputState::new(),
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = event_loop
            .create_window(
                Window::default_attributes()
                    .with_title("Runeforge Demo - Roguelike")
                    .with_inner_size(winit::dpi::PhysicalSize::new(
                        BUFFER_WIDTH * SCALE_FACTOR,
                        BUFFER_HEIGHT * SCALE_FACTOR,
                    )),
            )
            .unwrap();

        let size = window.inner_size();
        let window_ref: &'static Window = Box::leak(Box::new(window));
        let surface = SurfaceTexture::new(size.width, size.height, window_ref);
        let pixels = Pixels::new(BUFFER_WIDTH, BUFFER_HEIGHT, surface).unwrap();

        self.window = Some(window_ref);
        self.pixels = Some(pixels);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _: winit::window::WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),

            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        physical_key: PhysicalKey::Code(key_code),
                        state: key_state,
                        ..
                    },
                ..
            } => {
                // Handle ESC key directly
                if key_code == KeyCode::Escape && key_state == ElementState::Pressed {
                    event_loop.exit();
                    return;
                }

                // Translate physical key to virtual keys using input map
                if let Some(virtual_keys) = self.input_map.get(key_code) {
                    for &vkey in virtual_keys {
                        // Create input event and update state
                        let event = if key_state == ElementState::Pressed {
                            InputEvent::KeyPress(vkey)
                        } else {
                            InputEvent::KeyRelease(vkey)
                        };
                        self.input_state.update(&event);
                        // Handle the virtual key press
                        if key_state == ElementState::Pressed && !self.game.handle_virtual_key(vkey)
                        {
                            event_loop.exit();
                            return;
                        }
                    }
                }
            }

            WindowEvent::RedrawRequested => {
                if let Some(pixels) = &mut self.pixels {
                    let frame = pixels.frame_mut();
                    self.game.render(frame, &self.tileset);
                    if pixels.render().is_err() {
                        event_loop.exit();
                    }
                }
            }

            _ => {}
        }
    }

    fn about_to_wait(&mut self, _: &ActiveEventLoop) {
        if let Some(window) = self.window {
            window.request_redraw();
        }
    }
}

fn main() -> Result<()> {
    let event_loop = EventLoop::new()?;
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = App::default();
    event_loop.run_app(&mut app)?;

    Ok(())
}
