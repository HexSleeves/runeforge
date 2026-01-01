# Hello World

Here is a minimal example to verify that Runeforge is installed and working correctly.

## The Code

Create a file named `main.rs` in your project's `src` directory:

```rust
use runeforge_rl::prelude::*;

fn main() {
    // Initialize the random number generator
    let mut rng = Rng::new();

    // Create a 2D integer vector (point)
    let point = IVec2::new(10, 20);

    // Define a color
    let color = Color::RED;

    println!("Runeforge Hello World!");
    println!("----------------------");
    println!("Random d6 roll: {}", rng.roll_dice(1, 6));
    println!("Location: ({}, {})", point.x, point.y);
    println!("Color: {}", color);
}
```

## Running It

Run the program using cargo:

```bash
cargo run
```

You should see output similar to:

```text
Runeforge Hello World!
----------------------
Random d6 roll: 4
Location: (10, 20)
Color: #FF0000
```
