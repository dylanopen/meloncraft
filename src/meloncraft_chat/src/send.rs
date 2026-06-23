//! Module for message structs [`SendChatMessage`] and [`SendTitleMessage`].

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

/// Message indicating that the server has sent a title message to a list of clients.
/// See the fields for more details.
///
/// Title messages show up in the centre of the client's screen and shortly disappear.
/// Sending one while a message is still visible will overwrite the old title message.
#[derive(Message, Debug, Clone)]
pub struct SendTitleMessage {
    /// A list of player entities that should receive this message.
    /// Listening systems should send a set title text  packet to all these clients.
    /// This may contain every entity on the server.
    pub receivers: Vec<Entity>,

    /// The actual message itself. This is formatted text, in the form of an `NbtCompound` or
    /// `NbtString`, see [`NbtText`] for details.
    pub message: NbtText,
}

/// Message indicating that the server has sent a title message to a list of clients, and it
/// should be displayed above the hotbar.
/// See the fields for more details.
///
/// Actionbar titles display smaller, above the hotbar. In every other way, they are identical
/// to a main title message.
#[derive(Message, Debug, Clone)]
pub struct SendActionbarMessage {
    /// A list of player entities that should receive this message.
    /// Listening systems should send a set actionbar text packet to all these clients.
    /// This may contain every entity on the server.
    pub receivers: Vec<Entity>,

    /// The actual message itself. This is formatted text, in the form of an `NbtCompound` or
    /// `NbtString`, see [`NbtText`] for details.
    pub message: NbtText,
}
