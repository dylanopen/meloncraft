//! Systems for the state and storage of world/chunk events.

use bevy::ecs::message::MessageReader;
use bevy::ecs::query::With;
use bevy::ecs::system::Query;
use meloncraft_world::messages::ChunkGenerated;
use meloncraft_world::world::World;

use crate::marker::Overworld;

#[expect(clippy::expect_used, reason = "Will be replaced by logging when logging crate is implemented")]
pub fn store_generated_chunks(
    mut chunk_generated_mr: MessageReader<ChunkGenerated>,
    mut overworld_q: Query<&mut World, With<Overworld>>,
) {
    let mut world = overworld_q.single_mut().expect("Expected exactly one world with the Overworld component");

    for chunk_generated in chunk_generated_mr.read() {
        world.insert_chunk(chunk_generated.chunk_pos, chunk_generated.chunk.clone());
    }
}

