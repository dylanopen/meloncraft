//! Crate containing data types for representing a world, its chunks, etc. and messages related
//! to world and chunk events, for the [Meloncraft](https://github.com/dylanopen/meloncraft)
//! server implementation.
//!
//! See the documentation for the individual modules, structs and enums for more information on the
//! data types contained in this crate, and their usage in packets and systems throughout the
//! codebase.
//!
//! ## Pure `type` crate
//! `meloncraft_world` is a *type* crate. This means it doesn't hold any systems or stateful
//! functions. It just holds datatypes and message declarations & initialization
//! which *themselves* are *used* by systems and functions in other crates.
//! Impure functions are not allowed in this crate, and so should any functions not directly related
//! to operating on the datatype itself only (utility functions for conversions, instantiation,
//! logic, etc. are fine but only if they operate on the datatype itself, and not on any external
//! state or external mutable parameters).

use bevy::app::{App, Plugin};

use self::messages::{ChunkGenerated, ChunkRequest, GenerateChunk, SendChunk};

pub mod messages;
pub mod world;

/// Plugin for registering messages related to a Minecraft world, e.g. chunk sending and generation.
///
/// ## Registered messages:
/// - [`ChunkRequest`]
/// - [`ChunkGenerated`]
/// - [`SendChunk`]
/// - [`GenerateChunk`]
pub struct MeloncraftWorldPlugin;

impl Plugin for MeloncraftWorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<ChunkRequest>();
        app.add_message::<ChunkGenerated>();
        app.add_message::<SendChunk>();
        app.add_message::<GenerateChunk>();
    }
}
