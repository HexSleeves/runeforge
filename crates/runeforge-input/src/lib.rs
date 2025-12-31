//! Input handling and abstraction for roguelike games.
//!
//! # Overview
//!
//! `runeforge-input` provides an action-based input system that decouples physical input (keys, clicks)
//! from logical game actions (Move North, Open Inventory). It supports:
//!
//! *   **Action Mapping:** Bind keys to `VirtualKey`s (e.g., 'i' -> `Inventory`).
//! *   **Input State:** Track pressed keys and mouse positions.
//! *   **Presets:** Built-in support for common roguelike schemes (Vi-keys, WASD, Numpad).
//! *   **Rebinding:** Easily change bindings at runtime.
//!
//! # Usage
//!
//! Add this to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! runeforge-input = "0.1"
//! ```
//!
//! ## Basic Example
//!
//! ```rust
//! use runeforge_input::{InputMap, VirtualKey, Direction, screen_delta};
//! use winit::keyboard::KeyCode;
//!
//! fn main() {
//!     // 1. Create a default input map (WASD, Vi-keys, etc.)
//!     let input_map = InputMap::roguelike_default();
//!
//!     // 2. Simulate a key press (usually from winit event loop)
//!     let pressed_key = KeyCode::KeyH;
//!
//!     // 3. Translate to game action
//!     if let Some(actions) = input_map.get(pressed_key) {
//!         for action in actions {
//!             match action {
//!                 VirtualKey::Move(dir) => {
//!                     // Use screen_delta for screen coordinates (Y increases downward)
//!                     let (dx, dy) = screen_delta(*dir);
//!                     println!("Moving by ({}, {})", dx, dy);
//!                 }
//!                 _ => {}
//!             }
//!         }
//!     }
//! }
//! ```
//!
//! # Coordinate Conventions
//!
//! This crate uses `runeforge_direction::Direction` which uses **mathematical coordinates**
//! where North = +Y. For screen coordinates (origin top-left, Y increases downward),
//! use the [`screen_delta`] helper function.

#![deny(missing_docs)]

use std::collections::{HashMap, HashSet};
use winit::event::{KeyEvent, MouseButton as WinitMouseButton};
use winit::keyboard::{KeyCode, PhysicalKey};

// Re-export Direction from runeforge-direction
pub use runeforge_direction::prelude::Direction;

/// Convert a Direction to screen coordinates (Y increases downward).
///
/// This is useful for games where the origin is at the top-left corner.
///
/// # Example
///
/// ```
/// use runeforge_input::{Direction, screen_delta};
///
/// // In screen coordinates, North means moving UP (negative Y)
/// let (dx, dy) = screen_delta(Direction::NORTH);
/// assert_eq!((dx, dy), (0, -1));
///
/// // South means moving DOWN (positive Y)
/// let (dx, dy) = screen_delta(Direction::SOUTH);
/// assert_eq!((dx, dy), (0, 1));
/// ```
#[inline]
pub fn screen_delta(dir: Direction) -> (i32, i32) {
    let coord = dir.coord();
    // Flip Y axis for screen coordinates
    (coord.x, -coord.y)
}

/// A virtual key representing a logical game action.
///
/// Multiple physical keys can map to the same `VirtualKey`, allowing
/// support for different control schemes (vi-keys, numpad, arrows, etc.).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VirtualKey {
    /// Directional movement (8 directions)
    Move(Direction),

    /// Confirm/Accept action (Enter, Space, 'y')
    Confirm,

    /// Cancel action (Esc, 'n')
    Cancel,

    /// Pick up item from ground (',', 'g')
    Pickup,

    /// Drop item ('d')
    Drop,

    /// Open inventory ('i')
    Inventory,

    /// Use/Apply item ('u', 'a')
    Use,

    /// Look/Examine ('l', 'x')
    Look,

    /// Wait/Rest ('.', '5', numpad 5)
    Wait,

    /// Eat food ('e')
    Eat,

    /// Drink potion ('q')
    Quaff,

    /// Read scroll/book ('r')
    Read,

    /// Zap wand ('z')
    Zap,

    /// Open door ('o')
    Open,

    /// Close door ('c')
    Close,

    /// Show help ('?')
    Help,

    /// Quit game (Ctrl+Q, Esc in menus)
    Quit,
}

/// Mouse button identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MouseButton {
    /// Left mouse button
    Left,
    /// Right mouse button
    Right,
    /// Middle mouse button
    Middle,
    /// Other mouse button with ID
    Other(u16),
}

impl From<WinitMouseButton> for MouseButton {
    fn from(button: WinitMouseButton) -> Self {
        match button {
            WinitMouseButton::Left => MouseButton::Left,
            WinitMouseButton::Right => MouseButton::Right,
            WinitMouseButton::Middle => MouseButton::Middle,
            WinitMouseButton::Back => MouseButton::Other(3),
            WinitMouseButton::Forward => MouseButton::Other(4),
            WinitMouseButton::Other(n) => MouseButton::Other(n),
        }
    }
}

/// An input event representing a high-level game input.
#[derive(Debug, Clone, PartialEq)]
pub enum InputEvent {
    /// A virtual key was pressed
    KeyPress(VirtualKey),

    /// A virtual key was released
    KeyRelease(VirtualKey),

    /// Mouse cursor moved to (x, y) in pixels
    MouseMove {
        /// X coordinate in pixels
        x: f64,
        /// Y coordinate in pixels
        y: f64,
    },

    /// Mouse button clicked at (x, y) in pixels
    MouseClick {
        /// X coordinate in pixels
        x: f64,
        /// Y coordinate in pixels
        y: f64,
        /// Which button was clicked
        button: MouseButton,
    },

    /// Mouse button released at (x, y) in pixels
    MouseRelease {
        /// X coordinate in pixels
        x: f64,
        /// Y coordinate in pixels
        y: f64,
        /// Which button was released
        button: MouseButton,
    },

    /// Window close requested
    Quit,
}

/// Maps physical keyboard keys to virtual game actions.
///
/// Supports multiple control schemes and allows custom key rebinding.
pub struct InputMap {
    mappings: HashMap<KeyCode, Vec<VirtualKey>>,
}

impl InputMap {
    /// Create a new empty input map.
    pub fn new() -> Self {
        Self {
            mappings: HashMap::new(),
        }
    }

    /// Create an input map with default roguelike bindings.
    ///
    /// This includes:
    /// - Vi-keys (hjklyubn) for 8-directional movement
    /// - Numpad (1-9) for 8-directional movement
    /// - Arrow keys for 4-directional movement
    /// - WASD for 4-directional movement
    /// - Standard roguelike action keys (i, d, u, etc.)
    ///
    /// # Example
    ///
    /// ```
    /// use runeforge_input::InputMap;
    ///
    /// let input_map = InputMap::roguelike_default();
    /// ```
    pub fn roguelike_default() -> Self {
        let mut map = Self::new();

        // Vi-keys movement (hjklyubn)
        map.bind(KeyCode::KeyH, VirtualKey::Move(Direction::WEST));
        map.bind(KeyCode::KeyJ, VirtualKey::Move(Direction::SOUTH));
        map.bind(KeyCode::KeyK, VirtualKey::Move(Direction::NORTH));
        map.bind(KeyCode::KeyL, VirtualKey::Move(Direction::EAST));
        map.bind(KeyCode::KeyY, VirtualKey::Move(Direction::NORTH_WEST));
        map.bind(KeyCode::KeyU, VirtualKey::Move(Direction::NORTH_EAST));
        map.bind(KeyCode::KeyB, VirtualKey::Move(Direction::SOUTH_WEST));
        map.bind(KeyCode::KeyN, VirtualKey::Move(Direction::SOUTH_EAST));

        // Arrow keys (4-directional)
        map.bind(KeyCode::ArrowLeft, VirtualKey::Move(Direction::WEST));
        map.bind(KeyCode::ArrowRight, VirtualKey::Move(Direction::EAST));
        map.bind(KeyCode::ArrowUp, VirtualKey::Move(Direction::NORTH));
        map.bind(KeyCode::ArrowDown, VirtualKey::Move(Direction::SOUTH));

        // WASD (4-directional)
        map.bind(KeyCode::KeyA, VirtualKey::Move(Direction::WEST));
        map.bind(KeyCode::KeyD, VirtualKey::Move(Direction::EAST));
        map.bind(KeyCode::KeyW, VirtualKey::Move(Direction::NORTH));
        map.bind(KeyCode::KeyS, VirtualKey::Move(Direction::SOUTH));

        // Numpad movement (8-directional)
        map.bind(KeyCode::Numpad1, VirtualKey::Move(Direction::SOUTH_WEST));
        map.bind(KeyCode::Numpad2, VirtualKey::Move(Direction::SOUTH));
        map.bind(KeyCode::Numpad3, VirtualKey::Move(Direction::SOUTH_EAST));
        map.bind(KeyCode::Numpad4, VirtualKey::Move(Direction::WEST));
        map.bind(KeyCode::Numpad5, VirtualKey::Wait);
        map.bind(KeyCode::Numpad6, VirtualKey::Move(Direction::EAST));
        map.bind(KeyCode::Numpad7, VirtualKey::Move(Direction::NORTH_WEST));
        map.bind(KeyCode::Numpad8, VirtualKey::Move(Direction::NORTH));
        map.bind(KeyCode::Numpad9, VirtualKey::Move(Direction::NORTH_EAST));

        // Wait/Rest
        map.bind(KeyCode::Period, VirtualKey::Wait);
        map.bind(KeyCode::Space, VirtualKey::Wait);

        // Actions
        map.bind(KeyCode::Comma, VirtualKey::Pickup);
        map.bind(KeyCode::KeyG, VirtualKey::Pickup);
        map.bind(KeyCode::KeyI, VirtualKey::Inventory);
        map.bind(KeyCode::Enter, VirtualKey::Confirm);
        map.bind(KeyCode::Escape, VirtualKey::Cancel);
        map.bind(KeyCode::Slash, VirtualKey::Help);

        // Extended actions
        map.bind(KeyCode::KeyC, VirtualKey::Close);
        map.bind(KeyCode::KeyO, VirtualKey::Open);
        map.bind(KeyCode::KeyE, VirtualKey::Eat);
        map.bind(KeyCode::KeyQ, VirtualKey::Quaff);
        map.bind(KeyCode::KeyR, VirtualKey::Read);
        map.bind(KeyCode::KeyZ, VirtualKey::Zap);
        map.bind(KeyCode::KeyX, VirtualKey::Look);

        map
    }

    /// Bind a physical key to a virtual key.
    ///
    /// Multiple virtual keys can be bound to the same physical key.
    ///
    /// # Example
    ///
    /// ```
    /// use runeforge_input::{InputMap, VirtualKey, Direction};
    /// use winit::keyboard::KeyCode;
    ///
    /// let mut map = InputMap::new();
    /// map.bind(KeyCode::ArrowUp, VirtualKey::Move(Direction::NORTH));
    /// ```
    pub fn bind(&mut self, key: KeyCode, virtual_key: VirtualKey) {
        self.mappings.entry(key).or_default().push(virtual_key);
    }

    /// Remove all bindings for a physical key.
    pub fn unbind(&mut self, key: KeyCode) {
        self.mappings.remove(&key);
    }

    /// Map a winit `KeyEvent` to virtual keys.
    ///
    /// Returns `None` if the key is not mapped.
    ///
    /// This method is typically called inside your winit event handler to
    /// translate physical key presses into logical game actions.
    pub fn map_key_event(&self, event: &KeyEvent) -> Option<&[VirtualKey]> {
        if let PhysicalKey::Code(keycode) = event.physical_key {
            self.mappings.get(&keycode).map(|v| v.as_slice())
        } else {
            None
        }
    }

    /// Get all virtual keys bound to a physical key.
    pub fn get(&self, key: KeyCode) -> Option<&[VirtualKey]> {
        self.mappings.get(&key).map(|v| v.as_slice())
    }
}

impl Default for InputMap {
    fn default() -> Self {
        Self::roguelike_default()
    }
}

/// Tracks the current state of input (pressed keys, mouse position, etc.).
pub struct InputState {
    pressed_keys: HashSet<VirtualKey>,
    mouse_pos: (f64, f64),
    mouse_buttons: HashSet<MouseButton>,
}

impl InputState {
    /// Create a new input state with nothing pressed.
    pub fn new() -> Self {
        Self {
            pressed_keys: HashSet::new(),
            mouse_pos: (0.0, 0.0),
            mouse_buttons: HashSet::new(),
        }
    }

    /// Update state from an input event.
    ///
    /// # Example
    ///
    /// ```
    /// use runeforge_input::{InputState, InputEvent, VirtualKey};
    ///
    /// let mut state = InputState::new();
    /// state.update(&InputEvent::KeyPress(VirtualKey::Confirm));
    ///
    /// assert!(state.is_pressed(VirtualKey::Confirm));
    /// ```
    pub fn update(&mut self, event: &InputEvent) {
        match event {
            InputEvent::KeyPress(vkey) => {
                self.pressed_keys.insert(*vkey);
            }
            InputEvent::KeyRelease(vkey) => {
                self.pressed_keys.remove(vkey);
            }
            InputEvent::MouseMove { x, y } => {
                self.mouse_pos = (*x, *y);
            }
            InputEvent::MouseClick { button, .. } => {
                self.mouse_buttons.insert(*button);
            }
            InputEvent::MouseRelease { button, .. } => {
                self.mouse_buttons.remove(button);
            }
            InputEvent::Quit => {}
        }
    }

    /// Check if a virtual key is currently pressed.
    pub fn is_pressed(&self, key: VirtualKey) -> bool {
        self.pressed_keys.contains(&key)
    }

    /// Check if a mouse button is currently pressed.
    pub fn is_mouse_pressed(&self, button: MouseButton) -> bool {
        self.mouse_buttons.contains(&button)
    }

    /// Get the current mouse position in pixels.
    pub fn mouse_position(&self) -> (f64, f64) {
        self.mouse_pos
    }

    /// Clear all pressed keys and buttons.
    pub fn clear(&mut self) {
        self.pressed_keys.clear();
        self.mouse_buttons.clear();
    }
}

impl Default for InputState {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_screen_delta() {
        // North should be (0, -1) in screen coordinates
        assert_eq!(screen_delta(Direction::NORTH), (0, -1));
        assert_eq!(screen_delta(Direction::SOUTH), (0, 1));
        assert_eq!(screen_delta(Direction::EAST), (1, 0));
        assert_eq!(screen_delta(Direction::WEST), (-1, 0));
        assert_eq!(screen_delta(Direction::NORTH_EAST), (1, -1));
        assert_eq!(screen_delta(Direction::SOUTH_WEST), (-1, 1));
    }

    #[test]
    fn test_input_map_vi_keys() {
        let map = InputMap::roguelike_default();

        assert!(map
            .get(KeyCode::KeyH)
            .unwrap()
            .contains(&VirtualKey::Move(Direction::WEST)));
        assert!(map
            .get(KeyCode::KeyJ)
            .unwrap()
            .contains(&VirtualKey::Move(Direction::SOUTH)));
        assert!(map
            .get(KeyCode::KeyK)
            .unwrap()
            .contains(&VirtualKey::Move(Direction::NORTH)));
        assert!(map
            .get(KeyCode::KeyL)
            .unwrap()
            .contains(&VirtualKey::Move(Direction::EAST)));
    }

    #[test]
    fn test_input_map_numpad() {
        let map = InputMap::roguelike_default();

        assert!(map
            .get(KeyCode::Numpad7)
            .unwrap()
            .contains(&VirtualKey::Move(Direction::NORTH_WEST)));
        assert!(map
            .get(KeyCode::Numpad8)
            .unwrap()
            .contains(&VirtualKey::Move(Direction::NORTH)));
        assert!(map
            .get(KeyCode::Numpad9)
            .unwrap()
            .contains(&VirtualKey::Move(Direction::NORTH_EAST)));
        assert!(map
            .get(KeyCode::Numpad5)
            .unwrap()
            .contains(&VirtualKey::Wait));
    }

    #[test]
    fn test_input_map_arrows() {
        let map = InputMap::roguelike_default();

        assert!(map
            .get(KeyCode::ArrowUp)
            .unwrap()
            .contains(&VirtualKey::Move(Direction::NORTH)));
        assert!(map
            .get(KeyCode::ArrowDown)
            .unwrap()
            .contains(&VirtualKey::Move(Direction::SOUTH)));
        assert!(map
            .get(KeyCode::ArrowLeft)
            .unwrap()
            .contains(&VirtualKey::Move(Direction::WEST)));
        assert!(map
            .get(KeyCode::ArrowRight)
            .unwrap()
            .contains(&VirtualKey::Move(Direction::EAST)));
    }

    #[test]
    fn test_input_state_key_press() {
        let mut state = InputState::new();

        assert!(!state.is_pressed(VirtualKey::Confirm));

        state.update(&InputEvent::KeyPress(VirtualKey::Confirm));
        assert!(state.is_pressed(VirtualKey::Confirm));

        state.update(&InputEvent::KeyRelease(VirtualKey::Confirm));
        assert!(!state.is_pressed(VirtualKey::Confirm));
    }

    #[test]
    fn test_input_state_mouse() {
        let mut state = InputState::new();

        state.update(&InputEvent::MouseMove { x: 100.0, y: 200.0 });
        assert_eq!(state.mouse_position(), (100.0, 200.0));

        state.update(&InputEvent::MouseClick {
            x: 100.0,
            y: 200.0,
            button: MouseButton::Left,
        });
        assert!(state.is_mouse_pressed(MouseButton::Left));

        state.update(&InputEvent::MouseRelease {
            x: 100.0,
            y: 200.0,
            button: MouseButton::Left,
        });
        assert!(!state.is_mouse_pressed(MouseButton::Left));
    }

    #[test]
    fn test_custom_binding() {
        let mut map = InputMap::new();
        map.bind(KeyCode::Space, VirtualKey::Confirm);

        assert!(map
            .get(KeyCode::Space)
            .unwrap()
            .contains(&VirtualKey::Confirm));

        map.unbind(KeyCode::Space);
        assert!(map.get(KeyCode::Space).is_none());
    }
}
