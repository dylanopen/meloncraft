use bevy::ecs::query::With;
use bevy::ecs::system::Query;
use bevy::prelude::{MessageReader, MessageWriter};
use meloncraft_world::messages::{ChunkGenerated, ChunkRequest, GenerateChunk, SendChunk};
use meloncraft_world::world::World;
use crate::marker::Overworld;

#[expect(clippy::expect_used, reason = "Will soon be replaced with a logger")]
pub fn send_requested_chunks(
    mut chunk_request_mr: MessageReader<ChunkRequest>,
    world_q: Query<&World, With<Overworld>>,
    mut generate_chunk_mw: MessageWriter<GenerateChunk>,
    mut send_chunk_mw: MessageWriter<SendChunk>,
) {
    let world = world_q.single()
        .expect("Expected exactly one world with the Overworld component");

    for request in chunk_request_mr.read() {
        let chunk = world.get_chunk(&request.chunk_pos);
        if let Some(chunk) = chunk {
            send_chunk_mw.write(SendChunk {
                client: request.client,
                chunk_pos: request.chunk_pos,
                chunk: chunk.clone(),
            });
        } else {
            generate_chunk_mw.write(GenerateChunk {
                requested_by: request.client,
                chunk_pos: request.chunk_pos,
            });
        }
    }
}

pub fn send_generated_chunks(
    mut generated_chunk_mr: MessageReader<ChunkGenerated>,
    mut send_chunk_mw: MessageWriter<SendChunk>,
) {
    for generated_chunk in generated_chunk_mr.read() {
        if let Some(requested_by) = generated_chunk.requested_by {
            send_chunk_mw.write(SendChunk {
                client: requested_by,
                chunk_pos: generated_chunk.chunk_pos,
                chunk: generated_chunk.chunk.clone(),
            });
        }
    }
}
