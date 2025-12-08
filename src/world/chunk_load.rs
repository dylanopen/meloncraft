use tracing::error;
use valence::{ChunkLayer, anvil::{AnvilLevel, ChunkLoadEvent, ChunkLoadStatus}, app::{Plugin, Update}, message::SendMessage, prelude::{EventReader, Query, UnloadedChunk, With}, text::{Color, IntoText}};

pub struct WorldLoadPlugin;

impl Plugin for WorldLoadPlugin {
    fn build(&self, app: &mut valence::app::App) {
        app.add_systems(Update, handle_chunk_load);
    }
}

fn handle_chunk_load(
    mut events: EventReader<ChunkLoadEvent>,
    mut layers: Query<&mut ChunkLayer, With<AnvilLevel>>,
) {
    let mut layer = layers.single_mut();

    for event in events.read() {
        match &event.status {
            ChunkLoadStatus::Success { .. } => {},
            ChunkLoadStatus::Empty => {
                // TODO: chunk generation
                layer.insert_chunk(event.pos, UnloadedChunk::new());
            }
            ChunkLoadStatus::Failed(e) => {
                let errmsg = format!(
                    "Failed to load chunk at ({}, {}): {e:#}",
                    event.pos.x, event.pos.z
                );

                error!("{}", errmsg);
                layer.send_chat_message(errmsg.color(Color::RED));

                layer.insert_chunk(event.pos, UnloadedChunk::new());
            }
        }
    }
}

