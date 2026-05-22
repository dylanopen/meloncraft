//! Bevy messages for world update events, e.g. chunk generation messages.

use bevy::ecs::message::Message;
use bevy::prelude::Entity;
use meloncraft_chunk::Chunk;

#[derive(Message, Debug, Clone, Eq, PartialEq)]
pub struct ChunkRequest {
    pub client: Entity,
    pub chunk_x: i32,
    pub chunk_z: i32,
}

#[derive(Message, Debug, Clone, Eq, PartialEq)]
pub struct ChunkGenerated {
    pub requested_by: Option<Entity>,
    pub chunk_x: i32,
    pub chunk_z: i32,
    pub chunk: Chunk,
}

#[derive(Message, Debug, Clone)]
pub struct SendChunk {
    pub client: Entity,
    pub chunk_x: i32,
    pub chunk_z: i32,
    pub chunk: Chunk,
}

#[derive(Message, Debug, Clone)]
pub struct GenerateChunk {
    pub requested_by: Entity,
    pub chunk_x: i32,
    pub chunk_z: i32,
}
