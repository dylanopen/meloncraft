//! Module for messages [`BlockBroken`] and [`PlayerBrokeBlock`].

use bevy::ecs::message::Message;
use bevy::math::IVec3;
use bevy::prelude::Entity;

#[derive(Message, Debug, Clone)]
pub struct BlockBroken {
    pub block_location: IVec3,
}

#[derive(Message, Debug, Clone)]
pub struct PlayerBrokeBlock {
    pub block_broken: BlockBroken,
    pub player: Entity,
    pub sequence: i32,
}
