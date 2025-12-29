//! Windowed roguelike demo using winit 0.30 and pixels for GPU-accelerated rendering.
//!
//! This demonstrates the complete integration of PixelsRenderer with winit and pixels
//! to create a real windowed roguelike application with keyboard input.
//!
//! Controls:
//! - Arrow keys: Move player
//! - ESC: Close window
//!
//! Configuration:
//! - Adjust SCALE_FACTOR constant (line 26) to change window size
//! - Default is 3x for crisp, pixel-perfect scaling
//!
//! Run with: cargo run --example windowed_roguelike --package runeforge-pixels

use pixels::{Pixels, SurfaceTexture};
use runeforge_color::Color;
use runeforge_console::Console;
use runeforge_geometry::Point;
use runeforge_pixels::PixelsRenderer;
use runeforge_tileset::TrueTypeFont;
use winit::application::ApplicationHandler;
use winit::event::{ElementState, KeyEvent, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::window::{Window, WindowId};

const CONSOLE_WIDTH: u32 = 80;
const CONSOLE_HEIGHT: u32 = 25;
const SCALE_FACTOR: u32 = 3; // 3x scaling for larger, crisp pixels

struct Game {
    window: Option<&'static Window>,
    pixels: Option<Pixels<'static>>,
    renderer: PixelsRenderer,
    player_pos: Point,
}

impl Game {
    fn new(font: &dyn runeforge_tileset::Font) -> Self {
        let renderer = PixelsRenderer::new(CONSOLE_WIDTH, CONSOLE_HEIGHT, font);
        Self {
            window: None,
            pixels: None,
            renderer,
            player_pos: Point::new(40, 12),
        }
    }

    fn render_scene(&mut self) {
        // Clear console
        self.renderer.clear();

        // Draw border
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
            Point::new(30, 0),
            " RUNEFORGE DEMO ",
            Color::YELLOW,
            Color::BLACK,
        );

        // Draw floor tiles
        for y in 2..23 {
            for x in 2..78 {
                self.renderer
                    .draw_char(Point::new(x, y), '.', Color::DARK_GRAY, Color::BLACK);
            }
        }

        // Draw walls
        for y in 5..18 {
            self.renderer
                .draw_char(Point::new(20, y), '#', Color::WHITE, Color::BLACK);
            self.renderer
                .draw_char(Point::new(60, y), '#', Color::WHITE, Color::BLACK);
        }
        for x in 20..=60 {
            self.renderer
                .draw_char(Point::new(x, 5), '#', Color::WHITE, Color::BLACK);
            self.renderer
                .draw_char(Point::new(x, 17), '#', Color::WHITE, Color::BLACK);
        }

        // Draw doors
        self.renderer
            .draw_char(Point::new(40, 5), '+', Color::BROWN, Color::BLACK);
        self.renderer
            .draw_char(Point::new(40, 17), '+', Color::BROWN, Color::BLACK);

        // Draw some monsters
        self.renderer
            .draw_char(Point::new(30, 10), 'g', Color::GREEN, Color::BLACK);
        self.renderer
            .draw_char(Point::new(50, 12), 'o', Color::RED, Color::BLACK);

        // Draw items
        self.renderer
            .draw_char(Point::new(35, 8), '!', Color::MAGENTA, Color::BLACK);
        self.renderer.draw_char(
            Point::new(45, 14),
            '/',
            Color::rgb(139, 69, 19),
            Color::BLACK,
        );

        // Draw player
        self.renderer
            .draw_char(self.player_pos, '@', Color::CYAN, Color::BLACK);

        // Draw status bar
        self.renderer
            .draw_hline(23, 1, 78, '─', Color::GRAY, Color::BLACK);

        self.renderer.draw_string(
            Point::new(2, 24),
            &format!("Position: ({}, {})", self.player_pos.x, self.player_pos.y),
            Color::GREEN,
            Color::BLACK,
        );

        self.renderer.draw_string(
            Point::new(30, 24),
            "Use arrow keys to move",
            Color::YELLOW,
            Color::BLACK,
        );

        // Render to pixel buffer
        self.renderer.present();
    }

    fn move_player(&mut self, dx: i32, dy: i32) {
        let new_pos = Point::new(
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
            .with_title("Runeforge - Windowed Roguelike Demo")
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
                    KeyEvent {
                        physical_key: PhysicalKey::Code(key_code),
                        state: ElementState::Pressed,
                        ..
                    },
                ..
            } => match key_code {
                KeyCode::Escape => {
                    println!("ESC pressed. Exiting...");
                    event_loop.exit();
                }
                KeyCode::ArrowUp => {
                    self.move_player(0, -1);
                    self.render_scene();
                    self.update_pixels();
                }
                KeyCode::ArrowDown => {
                    self.move_player(0, 1);
                    self.render_scene();
                    self.update_pixels();
                }
                KeyCode::ArrowLeft => {
                    self.move_player(-1, 0);
                    self.render_scene();
                    self.update_pixels();
                }
                KeyCode::ArrowRight => {
                    self.move_player(1, 0);
                    self.render_scene();
                    self.update_pixels();
                }
                _ => {}
            },
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
    println!("Windowed Roguelike Demo");
    println!("======================");
    println!();
    println!("Note: Adjust SCALE_FACTOR constant to change window size");
    println!(
        "      Current scale: {}x (change line 26 in source)",
        SCALE_FACTOR
    );
    println!();
    println!("Controls:");
    println!("  Arrow keys: Move player");
    println!("  ESC: Exit");
    println!();

    // Load font
    let font_path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../runeforge-software/assets/font.ttf"
    );

    let font_data = match std::fs::read(font_path) {
        Ok(data) => data,
        Err(_) => {
            eprintln!("Error: Could not find font file at '{}'", font_path);
            eprintln!();
            eprintln!("Please ensure you have a font.ttf file in the runeforge-software/assets directory.");
            eprintln!("You can download a free monospace font like:");
            eprintln!("  - https://github.com/dejavu-fonts/dejavu-fonts/releases");
            eprintln!("  - https://github.com/microsoft/cascadia-code/releases");
            std::process::exit(1);
        }
    };

    let font = TrueTypeFont::from_bytes(&font_data, 16.0).expect("Failed to load font");

    // Create event loop and game
    let event_loop = EventLoop::new().expect("Failed to create event loop");
    event_loop.set_control_flow(ControlFlow::Wait);

    let mut game = Game::new(&font);

    println!("Starting window...");
    event_loop.run_app(&mut game).expect("Event loop failed");
}
