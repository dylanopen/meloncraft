use bevy::ecs::message::Message;
use bevy::prelude::Entity;
use meloncraft_chunk::Chunk;

#[derive(Message, Debug, Clone, Eq, PartialEq)]
pub struct ChunkRequest {
    client: Entity,
    chunk_x: i32,
    chunk_z: i32,
}

#[derive(Message, Debug, Clone, Eq, PartialEq)]
pub struct ChunkGenerated {
    requested_by: Entity,
    chunk_x: i32,
    chunk_z: i32,
    chunk: Chunk,
}