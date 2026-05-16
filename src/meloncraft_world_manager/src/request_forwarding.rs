use bevy::prelude::{MessageReader, MessageWriter, Res};
use meloncraft_world::messages::{ChunkRequest, GenerateChunk, SendChunk};
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