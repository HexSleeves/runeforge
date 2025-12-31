//! Windowed roguelike demo using PNG tilesets (CP437 sprite sheets).
//!
//! This demonstrates using classic roguelike tilesets instead of TrueType fonts.
//! The tileset uses CP437 character encoding for mapping characters to tiles.
//!
//! Controls:
//! - Arrow keys: Move player
//! - ESC: Close window
//!
//! Configuration:
//! - Adjust SCALE_FACTOR constant (line 29) to change window size
//! - Default is 3x for crisp, pixel-perfect scaling
//!
//! Run with: cargo run --example windowed_tileset_roguelike

use runeforge_color::Color;
use runeforge_geometry::prelude::IVec2;
use runeforge_input::{InputMap, VirtualKey};
use runeforge_terminal::prelude::{Console, PixelsRenderer};
use runeforge_tileset::prelude::Tileset;

use pixels::{Pixels, SurfaceTexture};
use winit::application::ApplicationHandler;
use winit::event::{ElementState, KeyEvent, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowId};

const CONSOLE_WIDTH: u32 = 80;
const CONSOLE_HEIGHT: u32 = 25;
const SCALE_FACTOR: u32 = 3; // 3x scaling for larger, crisp pixels

struct Game {
    window: Option<&'static Window>,
    pixels: Option<Pixels<'static>>,
    renderer: PixelsRenderer,
    player_pos: IVec2,
    input_map: InputMap,
}

impl Game {
    fn new(tileset: &dyn runeforge_tileset::prelude::Font) -> Self {
        let renderer = PixelsRenderer::new(CONSOLE_WIDTH, CONSOLE_HEIGHT, tileset);
        Self {
            window: None,
            pixels: None,
            renderer,
            player_pos: IVec2::new(40, 12),
            input_map: InputMap::roguelike_default(),
        }
    }

    fn render_scene(&mut self) {
        // Clear console
        self.renderer.clear();

        // Draw border with box-drawing characters (from CP437)
        self.renderer.draw_box(
            0,
            0,
            CONSOLE_WIDTH,
            CONSOLE_HEIGHT,
            Color::GRAY,
            Color::BLACK,
            false,
        );

        // Draw title
        self.renderer.draw_string(
            IVec2::new(27, 0),
            " TILESET ROGUELIKE ",
            Color::YELLOW,
            Color::BLACK,
        );

        // Draw floor tiles
        for y in 2..23 {
            for x in 2..78 {
                self.renderer
                    .draw_char(IVec2::new(x, y), '.', Color::DARK_GRAY, Color::BLACK);
            }
        }

        // Draw walls with # character
        for y in 5..18 {
            self.renderer
                .draw_char(IVec2::new(20, y), '#', Color::WHITE, Color::BLACK);
            self.renderer
                .draw_char(IVec2::new(60, y), '#', Color::WHITE, Color::BLACK);
        }
        for x in 20..=60 {
            self.renderer
                .draw_char(IVec2::new(x, 5), '#', Color::WHITE, Color::BLACK);
            self.renderer
                .draw_char(IVec2::new(x, 17), '#', Color::WHITE, Color::BLACK);
        }

        // Draw doors
        self.renderer
            .draw_char(IVec2::new(40, 5), '+', Color::BROWN, Color::BLACK);
        self.renderer
            .draw_char(IVec2::new(40, 17), '+', Color::BROWN, Color::BLACK);

        // Draw some monsters
        self.renderer
            .draw_char(IVec2::new(30, 10), 'g', Color::GREEN, Color::BLACK);
        self.renderer
            .draw_char(IVec2::new(50, 12), 'o', Color::RED, Color::BLACK);

        // Draw items
        self.renderer
            .draw_char(IVec2::new(35, 8), '!', Color::MAGENTA, Color::BLACK);
        self.renderer.draw_char(
            IVec2::new(45, 14),
            '/',
            Color::rgb(139, 69, 19),
            Color::BLACK,
        );

        // Draw player
        self.renderer
            .draw_char(self.player_pos, '@', Color::CYAN, Color::BLACK);

        // Draw status bar with box-drawing characters
        self.renderer
            .draw_hline(23, 1, 78, '─', Color::GRAY, Color::BLACK);

        self.renderer.draw_string(
            IVec2::new(2, 24),
            &format!(
                "Position: ({:2}, {:2})",
                self.player_pos.x, self.player_pos.y
            ),
            Color::GREEN,
            Color::BLACK,
        );

        self.renderer.draw_string(
            IVec2::new(25, 24),
            "CP437 Tileset",
            Color::YELLOW,
            Color::BLACK,
        );

        self.renderer.draw_string(
            IVec2::new(42, 24),
            "Arrow/Vi-keys/Numpad",
            Color::rgb(100, 150, 255),
            Color::BLACK,
        );

        // Render to pixel buffer
        self.renderer.present();
    }

    fn move_player(&mut self, dx: i32, dy: i32) {
        let new_pos = IVec2::new(
            (self.player_pos.x + dx).clamp(2, 77),
            (self.player_pos.y + dy).clamp(2, 22),
        );

        // Simple collision: don't walk into walls
        if let Some(cell) = self.renderer.get(new_pos) {
            if cell.ch != '#' {
                self.player_pos = new_pos;
            }
        }
    }
}

impl ApplicationHandler for Game {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_some() {
            return;
        }

        let window_attributes = Window::default_attributes()
            .with_title("Runeforge - Tileset Roguelike Demo")
            .with_inner_size(winit::dpi::PhysicalSize::new(
                self.renderer.pixel_width() * SCALE_FACTOR,
                self.renderer.pixel_height() * SCALE_FACTOR,
            ))
            .with_resizable(false);

        let window = event_loop
            .create_window(window_attributes)
            .expect("Failed to create window");

        let window_size = window.inner_size();

        // Convert to static lifetime using Box::leak
        // This is intentional for GUI apps that live until program exit
        let window_ref: &'static Window = Box::leak(Box::new(window));

        let surface_texture =
            SurfaceTexture::new(window_size.width, window_size.height, window_ref);

        let pixels = Pixels::new(
            self.renderer.pixel_width(),
            self.renderer.pixel_height(),
            surface_texture,
        )
        .expect("Failed to create Pixels");

        self.window = Some(window_ref);
        self.pixels = Some(pixels);

        // Initial render
        self.render_scene();
        self.update_pixels();
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                println!("Window close requested. Exiting...");
                event_loop.exit();
            }
            WindowEvent::KeyboardInput {
                event:
                    key_event @ KeyEvent {
                        state: ElementState::Pressed,
                        ..
                    },
                ..
            } => {
                // Map physical key to virtual keys using input system
                // Collect keys first to avoid borrowing issues
                let virtual_keys = self
                    .input_map
                    .map_key_event(&key_event)
                    .map(|keys| keys.to_vec());

                if let Some(keys) = virtual_keys {
                    for vkey in keys {
                        match vkey {
                            VirtualKey::Move(direction) => {
                                let (dx, dy) = runeforge_rl::input::screen_delta(direction);
                                self.move_player(dx, dy);
                                self.render_scene();
                                self.update_pixels();
                            }
                            VirtualKey::Cancel => {
                                println!("ESC pressed. Exiting...");
                                event_loop.exit();
                            }
                            _ => {}
                        }
                    }
                }
            }
            WindowEvent::RedrawRequested => {
                self.update_pixels();
            }
            _ => {}
        }
    }
}

impl Game {
    fn update_pixels(&mut self) {
        if let (Some(window), Some(pixels)) = (&self.window, &mut self.pixels) {
            // Copy renderer's pixel buffer to pixels frame
            let frame = pixels.frame_mut();
            frame.copy_from_slice(self.renderer.pixel_buffer());

            // Render to window
            if let Err(e) = pixels.render() {
                eprintln!("pixels.render() failed: {}", e);
            }

            window.request_redraw();
        }
    }
}

fn main() {
    println!("Tileset Roguelike Demo");
    println!("======================");
    println!();
    println!("Using CP437 tileset (classic roguelike sprite sheet)");
    println!("Note: Adjust SCALE_FACTOR constant to change window size");
    println!(
        "      Current scale: {}x (change line 29 in source)",
        SCALE_FACTOR
    );
    println!();
    println!("Controls:");
    println!("  Arrow keys: Move player");
    println!("  ESC: Exit");
    println!();

    // Load tileset
    let tileset_path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/crates/runeforge-tileset/assets/terminal8x8_gs_ro.png"
    );

    let tileset = match Tileset::from_file(tileset_path, 8, 8) {
        Ok(t) => {
            println!("✓ Successfully loaded tileset from {}", tileset_path);
            println!(
                "  Tileset: {}x{} tiles, {}x{} pixels per tile",
                t.columns, t.rows, t.tile_width, t.tile_height
            );
            t
        }
        Err(e) => {
            eprintln!("✗ Failed to load tileset: {}", e);
            eprintln!();
            eprintln!("Tileset path: {}", tileset_path);
            eprintln!("Make sure you're running from the `runeforge-rl/` directory:");
            eprintln!("  cargo run --example windowed_tileset_roguelike");
            std::process::exit(1);
        }
    };

    // Create event loop and game
    let event_loop = EventLoop::new().expect("Failed to create event loop");
    event_loop.set_control_flow(ControlFlow::Wait);

    let mut game = Game::new(&tileset);

    println!();
    println!("Starting window with {}x scaling...", SCALE_FACTOR);
    event_loop.run_app(&mut game).expect("Event loop failed");
}
