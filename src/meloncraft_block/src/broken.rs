//! Module for messages [`BlockBroken`] and [`PlayerBrokeBlock`].

use bevy::ecs::message::Message;
use bevy::math::IVec3;
use bevy::prelude::Entity;

/// Message sent whenever a block is broken by any cause.
///
/// You should listen to this event if you want to do something when *any* block is broken,
/// regardless of the cause.
///
/// No information about the block is sent with this message, except for the block's position. You
/// may want to use other components and messages to, say, get the block type from a `Chunk` /
/// `World`.
/// 
/// ## Alternatives
/// - If you want to respond only when a player breaks a block, and ignore other causes, listen to
///   the [`PlayerBrokeBlock`] message instead, which wraps this message as a submessage, as well as
///   other info.
#[derive(Message, Debug, Clone)]
pub struct BlockBroken {

    /// The location of the block that was broken, as an [`IVec3`].
    /// The coordinates are in block coordinates, not chunk coordinates. These will be the same as
    /// are visible in the client's F3 menu.
    ///
    /// As blocks are discrete, their positions are integers, so stored as an [`IVec3`].
    pub block_location: IVec3,
}

/// Message sent whenever a block is broken by a player.
///
/// You should listen to this event if you want to respond only when a player breaks a block, and
/// ignore other causes. 
///
/// This message contains the submessage for the [`BlockBroken`] message, as well as the `Entity` of
/// the player who broke the block, and the [sequence number](`PlayerBrokeBlock::sequence`) of the
/// block breaking action.
///
/// ## Alternatives
/// - If you want to respond to *any* block breaking, regardless of the cause, listen to the
///   [`BlockBroken`] message instead, which is sent whenever a block is broken by any cause, and does
///   not contain the player or sequence info.
#[derive(Message, Debug, Clone)]
pub struct PlayerBrokeBlock {

    pub block_broken: BlockBroken,

    pub player: Entity,

    pub sequence: i32,
}
