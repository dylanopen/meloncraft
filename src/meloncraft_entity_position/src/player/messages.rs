use bevy::ecs::message::Message;
use bevy::prelude::Entity;
use crate::EntityPosition;

#[derive(Debug, Clone, Message)]
pub struct PlayerMoved {
    pub entity: Entity,
    pub old_position: EntityPosition,
    pub new_position: EntityPosition,
}