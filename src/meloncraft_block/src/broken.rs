//! Module for messages [`BlockBroken`] and [`PlayerBrokeBlock`].

use bevy::ecs::message::Message;
use bevy::math::IVec3;
use bevy::prelude::Entity;

/// Message sent whenever a block is broken by any cause.
///
/// You should listen to this event if you want to do something when *any* block is broken,
/// regardless of the cause.
/// 
/// ## Alternatives
/// - If you want to respond only when a player breaks a block, and ignore other causes, listen to
///   the [`PlayerBrokeBlock`] message instead, which wraps this message as a submessage, as well as
///   other info.
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
