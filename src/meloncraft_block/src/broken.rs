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

    /// The submessage for the [`BlockBroken`] message, which contains the block's position.
    /// This message will be a clone of the [`BlockBroken`] message, also sent after the event
    /// triggering this message. A [`BlockBroken`] message will always be sent whenever a
    /// [`PlayerBrokeBlock`] message is sent, but not necessarily the other way around.
    ///
    /// See the [`BlockBroken`] documentation for more information on the fields of a
    /// [`BlockBroken`] message.
    pub block_broken: BlockBroken,

    /// The Bevy [`Entity`] of the client which broke the block.
    /// You can use this entity in a query to get other information about the player, for example,
    /// their username or IP address from their `ClientConnection` component, or their position from
    /// an `EntityPosition` component, etc.
    pub player: Entity,

    /// The sequence number of the block breaking action, as an `i32`.
    ///
    /// The sequence number is used in the `ClientboundAcknowledgeBlockChange` packet, which the
    /// server sends to the client when that client broke a block.
    /// You can, for example, listen to this message and respond by acknowledging the block change
    /// with the same sequence number, to essentially complete the block breaking action for the
    /// client's view. This doesn't seem to be necessary though, you can also just send a
    /// `ClientboundBlockUpdate` packet instead, it seems.
    ///
    /// The sequence number is incremented by the client every time they break a block. If you send
    /// a `ClientboundAcknowledgeBlockChange` packet with the same sequence number, the client will
    /// know you have acknowledged the block change, and will show the block as broken.
    ///
    /// Read more: <https://minecraft.wiki/w/Java_Edition_protocol/Packets#Acknowledge_Block_Change>.
    pub sequence: i32,
}
