use bevy::ecs::message::Message;
use bevy::prelude::Entity;

#[derive(Message, Debug, Clone, Eq, PartialEq)]
pub struct ChunkRequest {
    client: Entity,
    chunk_x: i32,
    chunk_z: i32,
}