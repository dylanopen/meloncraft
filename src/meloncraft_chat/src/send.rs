//! Module for message struct [`SendChatMessage`].

use bevy::ecs::entity::Entity;
use bevy::ecs::message::Message;
use meloncraft_text::NbtText;

/// Message indicating that the server has sent a chat message to a list of clients.
/// See the fields for more details.
#[derive(Message, Debug, Clone)]
pub struct SendChatMessage {
    /// A list of player entities that should receive this message.
    /// Listening systems should send a chat packet to all these clients.
    /// This may contain every entity on the server.
    pub receivers: Vec<Entity>,

    /// The actual message itself. This is formatted text, in the form of an `NbtCompound` or
    /// `NbtString`, see [`NbtText`] for details.
    pub message: NbtText,
}
