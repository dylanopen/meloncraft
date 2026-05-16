use bevy::ecs::message::Message;
use bevy::prelude::Entity;
use meloncraft_chunk::Chunk;

#[derive(Message, Debug, Clone)]
pub struct SendChunk {
    pub client: Entity,
    pub chunk_x: i32,
    pub chunk_z: i32,
    pub chunk: Chunk,
}