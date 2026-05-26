//! Module for message [`SetBlock`].

use bevy::ecs::message::Message;
use bevy::math::IVec3;

use crate::block::Block;

/// Message sent whenever a block is set by any cause.
///
/// You should listen to this event if you want to do something when a block is set, for example,
/// update the block in chunk storage.
#[derive(Message, Debug, Clone)]
pub struct SetBlock {
    /// The location of the block that was broken, as an [`IVec3`].
    /// The coordinates are in block coordinates, not chunk coordinates. These will be the same as
    /// are visible in the client's F3 menu.
    ///
    /// As blocks are discrete, their positions are integers, so stored as an [`IVec3`].
    pub block_location: IVec3,

    /// The new block type to set the block to, as a [`Block`] struct.
    /// To remove a block, set this to air (`Block::new(0)`).
    pub new_block: Block,
}
