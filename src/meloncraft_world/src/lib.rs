use bevy::app::{App, Plugin};

pub mod world;
pub mod messages;

pub struct MeloncraftWorldPlugin;

impl Plugin for MeloncraftWorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<messages::ChunkRequest>();
        app.add_message::<messages::ChunkGenerated>();
        app.add_message::<messages::SendChunk>();
        app.add_message::<messages::GenerateChunk>();
    }
}
