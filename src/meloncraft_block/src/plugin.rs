//! Module for [`MeloncraftBlockPlugin`].

use crate::broken::{BlockBroken, PlayerBrokeBlock};
use crate::set::SetBlock;
use bevy::app::{App, Plugin};

/// Plugin to register Bevy messages for the `block` crate.
///
/// ## Registered messages
/// - **[`SetBlock`]**: Sent when a block is set by any cause. Contains the position of the block
///   that was set, and the new block type to set the block to. You should listen for this if you
///   want to do something when *any* block is set, regardless of the cause.
/// - **[`BlockBroken`]**: Sent when a block is broken by any cause. Contains *just* the block broken's
///   position; you should listen for this if you want to do something when *any* block is broken,
///   regardless of the cause.
/// - **[`PlayerBrokeBlock`]**: Sent when a block is broken by a player. Contains the submessage for the
///   [`BlockBroken`] message, as well as the `Entity` of the player who broke the block, and the
///   [sequence number](`PlayerBrokeBlock::sequence`) of the block breaking action. You should
///   listen to *this* if you want to respond only when a player breaks a block, and ignore other
///   causes.
pub struct MeloncraftBlockPlugin;

impl Plugin for MeloncraftBlockPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<SetBlock>();
        app.add_message::<BlockBroken>();
        app.add_message::<PlayerBrokeBlock>();
    }
}
