# Installation

Runeforge is available on [crates.io](https://crates.io/crates/runeforge-rl).

## Adding to Cargo.toml

Add the following to your project's `Cargo.toml` file under `[dependencies]`:

```toml
[dependencies]
runeforge-rl = "0.1"
```

## Feature Flags

Runeforge is modular. By default, it includes the most common features. You can customize your build by enabling or disabling features:

```toml
[dependencies]
runeforge-rl = { version = "0.1", default-features = false, features = ["render-wgpu", "fov", "pathfinding"] }
```

### Available Features

- `render-wgpu`: Enable GPU-accelerated rendering (default).
- `render-software`: Enable software rendering backend.
- `render-terminal`: Enable ANSI terminal backend.
- `fov`: Field-of-view algorithms.
- `pathfinding`: Pathfinding algorithms.
- `noise`: Procedural noise generation.
- `serialization`: Serde support for core types.
