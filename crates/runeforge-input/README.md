# runeforge-input

Action-based input abstraction layer for roguelike games.

## Features

- **Multiple Control Schemes**: Built-in support for vi-keys, numpad, arrows, and WASD
- **Action-Based Mapping**: Physical keys map to logical game actions (VirtualKey)
- **Custom Key Binding**: Easy remapping and custom control schemes
- **Mouse Support**: Track mouse position and button states
- **Input State Tracking**: Know what's currently pressed
- **8-Directional Movement**: Full diagonal support for classic roguelikes
- **Winit Integration**: Seamless integration with winit 0.30 event handling

## Quick Start

```rust
use runeforge_input::{InputMap, VirtualKey, Direction};
use winit::keyboard::KeyCode;

// Create default roguelike input map
let input_map = InputMap::roguelike_default();

// Check what a key does
if let Some(actions) = input_map.get(KeyCode::KeyH) {
    // 'h' moves west in vi-keys mode
    assert!(actions.contains(&VirtualKey::Move(Direction::West)));
}
```

## Control Schemes

The default input map (`InputMap::roguelike_default()`) supports all these schemes simultaneously:

### Vi-Keys (8-directional)

```bash
y  k  u
 \ | /
h -+- l
 / | \
b  j  n
```

### Numpad (8-directional)

```bash
7  8  9
 \ | /
4 -5- 6
 / | \
1  2  3
```

### Arrow Keys (4-directional)

- ↑ ↓ ← → for cardinal movement

### WASD (4-directional)

- W A S D for cardinal movement

## Usage in Winit Applications

```rust
use runeforge_input::{InputMap, VirtualKey};
use winit::event::{WindowEvent, KeyEvent, ElementState};

struct Game {
    input_map: InputMap,
    // ... other game state
}

impl Game {
    fn new() -> Self {
        Self {
            input_map: InputMap::roguelike_default(),
        }
    }

    fn handle_event(&mut self, event: WindowEvent) {
        match event {
            WindowEvent::KeyboardInput {
                event: key_event @ KeyEvent {
                    state: ElementState::Pressed,
                    ..
                },
                ..
            } => {
                // Collect virtual keys to avoid borrow checker issues
                let virtual_keys: Vec<VirtualKey> = self
                    .input_map
                    .map_key_event(&key_event)
                    .map(|keys| keys.to_vec())
                    .unwrap_or_default();

                for vkey in virtual_keys {
                    match vkey {
                        VirtualKey::Move(direction) => {
                            let (dx, dy) = direction.to_delta();
                            self.move_player(dx, dy);
                        }
                        VirtualKey::Pickup => {
                            self.pickup_item();
                        }
                        VirtualKey::Inventory => {
                            self.open_inventory();
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }
}
```

## Available Actions

### Movement

- `VirtualKey::Move(Direction)` - 8-directional movement

### Common Actions

- `Confirm` - Enter, Space
- `Cancel` - Escape
- `Pickup` - Comma, 'g'
- `Drop` - 'd'
- `Inventory` - 'i'
- `Use` - 'u'
- `Look` - 'l', 'x'
- `Wait` - Period, Space, Numpad 5

### Roguelike Actions

- `Eat` - 'e'
- `Quaff` (drink) - 'q'
- `Read` - 'r'
- `Zap` - 'z'
- `Open` - 'o'
- `Close` - 'c'
- `Help` - '?'
- `Quit` - handled separately in most games

## Custom Bindings

```rust
use runeforge_input::{InputMap, VirtualKey};
use winit::keyboard::KeyCode;

// Start with empty map
let mut input_map = InputMap::new();

// Bind keys manually
input_map.bind(KeyCode::Space, VirtualKey::Confirm);
input_map.bind(KeyCode::Escape, VirtualKey::Cancel);
input_map.bind(KeyCode::KeyE, VirtualKey::Use);

// Multiple keys can map to the same action
input_map.bind(KeyCode::Enter, VirtualKey::Confirm);
input_map.bind(KeyCode::KeyY, VirtualKey::Confirm);

// Remove a binding
input_map.unbind(KeyCode::Space);
```

## Input State Tracking

Track which keys and mouse buttons are currently pressed:

```rust
use runeforge_input::{InputState, InputEvent, VirtualKey, MouseButton};

let mut state = InputState::new();

// Update from events
state.update(&InputEvent::KeyPress(VirtualKey::Confirm));
state.update(&InputEvent::MouseMove { x: 100.0, y: 200.0 });
state.update(&InputEvent::MouseClick {
    x: 100.0,
    y: 200.0,
    button: MouseButton::Left,
});

// Check state
assert!(state.is_pressed(VirtualKey::Confirm));
assert!(state.is_mouse_pressed(MouseButton::Left));
assert_eq!(state.mouse_position(), (100.0, 200.0));
```

## Direction Utilities

The `Direction` enum provides useful conversion methods:

```rust
use runeforge_input::Direction;

// Convert to delta coordinates
let (dx, dy) = Direction::NorthEast.to_delta();
assert_eq!((dx, dy), (1, -1));

// Create from delta
let dir = Direction::from_delta(1, 1);
assert_eq!(dir, Some(Direction::SouthEast));

// Invalid deltas return None
assert_eq!(Direction::from_delta(2, 0), None);
assert_eq!(Direction::from_delta(0, 0), None);
```

## Why Action-Based Input?

Traditional roguelikes need to support multiple control schemes because:

1. **Not all keyboards have numpads** - Laptops often omit them
2. **Different preferences** - Some prefer vi-keys, others prefer arrows
3. **Accessibility** - Users should be able to rebind keys to their needs
4. **International keyboards** - QWERTY assumptions don't work everywhere

Action-based input solves this by:

1. Mapping multiple physical keys to logical actions
2. Letting games work with `VirtualKey::Move(Direction)` instead of raw keycodes
3. Making key rebinding trivial
4. Keeping game logic independent of input hardware

## Research Sources

The input system design is based on roguelike community best practices:

- [Preferred Key Controls - RogueBasin](https://www.roguebasin.com/index.php/Preferred_Key_Controls)
- [User Interface Features - RogueBasin](https://www.roguebasin.com/index.php?title=User_interface_features)
- [leafwing-input-manager](https://crates.io/crates/leafwing-input-manager) - Action mapping pattern
- [input-actions](https://docs.rs/input-actions) - Virtual button abstraction

## Examples

See the `runeforge-terminal` crate for complete working examples:

```bash
# TrueType font roguelike with full input support
cargo run --example windowed_roguelike --package runeforge-terminal

# Tileset roguelike with full input support
cargo run --example windowed_tileset_roguelike --package runeforge-terminal
```

Both examples support all control schemes (vi-keys, numpad, arrows, WASD) out of the box!

## License

BSD-3-Clause (see LICENSE in repository root)
