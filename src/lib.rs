//! Runeforge - A modern roguelike library for Rust.
//!
//! This crate is the main entry point for the Runeforge workspace. It re-exports
//! the `runeforge-core` facade crate and provides workspace-level examples.

#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)]
#![warn(clippy::dbg_macro, clippy::todo, clippy::unimplemented)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::multiple_crate_versions)] // Transitive deps from wgpu/pixels

pub use runeforge_core::*;
