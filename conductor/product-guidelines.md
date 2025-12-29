# Runeforge Product Guidelines

## Overview

This document defines the standards for documentation, API design, and code quality that govern the Runeforge project. All contributors should follow these guidelines to maintain consistency across the codebase.

---

## Documentation Style

### Tone and Voice

Runeforge documentation uses a **technical and precise** tone:

- **Clarity over personality** — Focus on accuracy and completeness
- **Direct language** — Avoid unnecessary words, hedging, or filler
- **Assume competence** — Target experienced Rust developers familiar with roguelike concepts
- **Be specific** — Use exact terminology; avoid vague descriptions
- **Show, don't tell** — Prefer code examples over lengthy explanations when appropriate

### Documentation Standards

Follow **standard rustdoc conventions**:

- Every public item (function, struct, trait, enum) must have a description
- Document all parameters and return values
- Include code examples for:
  - Complex or non-obvious APIs
  - Functions with multiple usage patterns
  - Core abstractions (Console, FOV, Pathfinding)
- Simple, self-explanatory APIs do not require examples
- Use `# Examples`, `# Panics`, `# Errors` sections as appropriate

### Example Documentation

```rust
/// Computes the field of view from a given origin point.
///
/// Uses the recursive shadowcasting algorithm to determine which tiles
/// are visible from the origin within the specified radius.
///
/// # Arguments
///
/// * `origin` - The center point for FOV calculation
/// * `radius` - Maximum visibility distance
/// * `transparent` - Callback returning `true` if a tile allows light through
///
/// # Returns
///
/// A `FovMap` containing visibility data for all tiles within radius.
///
/// # Example
///
/// ```
/// use runeforge_fov::{FovMap, compute_fov};
/// use runeforge_geometry::Point;
///
/// let fov = compute_fov(
///     Point::new(5, 5),
///     8,
///     |p| map.is_transparent(p),
/// );
/// ```
pub fn compute_fov<F>(origin: Point, radius: i32, transparent: F) -> FovMap
where
    F: Fn(Point) -> bool,
```

---

## API Design

### Naming Conventions

Runeforge follows **Rust-idiomatic naming**:

- Use `snake_case` for functions and variables
- Use `PascalCase` for types and traits
- Use `SCREAMING_SNAKE_CASE` for constants
- Prefer method syntax over free functions (e.g., `console.put_char()` not `console_put_char()`)
- Follow Rust API guidelines: https://rust-lang.github.io/api-guidelines/

### Migration Support

To support developers migrating from libtcod:

- Maintain a **migration guide** mapping libtcod function names to Runeforge equivalents
- Document behavioral differences where APIs diverge
- Provide migration examples for common patterns

### Error Handling

Runeforge uses **Result-based error handling with typed errors**:

- All fallible operations return `Result<T, E>`
- Each crate defines its own error types using `thiserror`
- Error types should be:
  - Specific and actionable
  - Implement `std::error::Error`
  - Include relevant context (e.g., coordinates, dimensions)
- **Panics** are reserved for programmer errors only:
  - Index out of bounds (when using indexing syntax)
  - Invariant violations that indicate bugs
  - Document all panic conditions with `# Panics` section

### Example Error Type

```rust
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConsoleError {
    #[error("position ({x}, {y}) is outside console bounds ({width}x{height})")]
    OutOfBounds { x: i32, y: i32, width: u32, height: u32 },
    
    #[error("invalid console dimensions: {width}x{height}")]
    InvalidDimensions { width: u32, height: u32 },
    
    #[error("font file not found: {path}")]
    FontNotFound { path: String },
}
```

---

## Code Quality

### Dependency Management

Runeforge follows a **minimal dependencies** philosophy:

- Prefer standard library solutions where reasonable
- Each crate should minimize external dependencies
- Use **feature flags** for optional heavy dependencies
- Justify new dependencies in PR descriptions
- Regularly audit dependencies with `cargo deny`

### Dependency Guidelines

| Category | Guideline |
|----------|-----------|
| Required | `thiserror` for error types |
| Allowed | Well-maintained, focused crates (e.g., `rand`, `ab_glyph`) |
| Feature-gated | Heavy dependencies like `wgpu`, `pixels` |
| Avoid | Large frameworks, unmaintained crates |

### Versioning and Compatibility

Runeforge follows **strict semantic versioning**:

- **MAJOR** (x.0.0) — Breaking API changes
- **MINOR** (0.x.0) — New features, backward compatible
- **PATCH** (0.0.x) — Bug fixes, backward compatible

### Compatibility Rules

- No breaking changes in minor or patch releases
- Deprecate APIs for at least one minor release before removal
- Use `#[deprecated]` attribute with migration guidance
- Document breaking changes in CHANGELOG.md
- Consider `#[doc(hidden)]` for internal APIs that may change

### Example Deprecation

```rust
#[deprecated(
    since = "0.3.0",
    note = "Use `compute_fov` instead. This function will be removed in 0.5.0."
)]
pub fn calculate_fov(/* ... */) { /* ... */ }
```

---

## Summary

| Aspect | Standard |
|--------|----------|
| **Tone** | Technical and precise |
| **Naming** | Rust-idiomatic with migration guide |
| **Documentation** | Standard rustdoc with examples for complex APIs |
| **Errors** | Result-based with typed errors per crate |
| **Dependencies** | Minimal, feature-gated for heavy deps |
| **Versioning** | Strict semver, deprecate before removing |
