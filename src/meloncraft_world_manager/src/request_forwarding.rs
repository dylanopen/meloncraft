use bevy::prelude::{MessageReader, MessageWriter, Res};
use meloncraft_world::messages::{ChunkGenerated, ChunkRequest, GenerateChunk, SendChunk};
use crate::marker::Overworld;

pub fn send_requested_chunks(
    mut chunk_request_mr: MessageReader<ChunkRequest>,
    world: Res<Overworld>,
    mut generate_chunk_mw: MessageWriter<GenerateChunk>,
    mut send_chunk_mw: MessageWriter<SendChunk>,
) {
    for request in chunk_request_mr.read() {
        let chunk = world.0.get_chunk(request.chunk_x, request.chunk_z);
        if let Some(chunk) = chunk {
            send_chunk_mw.write(SendChunk {
                client: request.client,
                chunk_x: request.chunk_x,
                chunk_z: request.chunk_z,
                chunk: chunk.clone(),
            });
        } else {
            generate_chunk_mw.write(GenerateChunk {
                requested_by: request.client,
                chunk_x: request.chunk_x,
                chunk_z: request.chunk_z,
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
                chunk_x: generated_chunk.chunk_x,
                chunk_z: generated_chunk.chunk_z,
                chunk: generated_chunk.chunk.clone(),
            });
        }
    }
}
