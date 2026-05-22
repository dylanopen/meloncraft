use bevy::app::{App, Plugin};

use self::messages::{ChunkGenerated, ChunkRequest, GenerateChunk, SendChunk};

pub mod world;
pub mod messages;

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
