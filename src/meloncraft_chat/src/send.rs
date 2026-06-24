//! Module for message structs [`SendChatMessage`] and [`SendTitleMessage`].

use bevy::ecs::entity::Entity;
use bevy::ecs::message::Message;
use meloncraft_text::NbtText;

use crate::title::{TitlePosition, TitleTimings};

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
/// Title messages show up in different places (see [`TitlePosition`] and disappear shortly
/// after they are sent.
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

    /// The position of the title message. Can be any variant of [`TitlePosition`].
    pub position: TitlePosition,

    /// The animation timings that the title should be displayed with, or `None` to just
    /// use the last title's settings.
    pub times: Option<TitleTimings>,
}

/// Tell the client(s) that they should clear all titles from the screen.
/// It **also** clears things such as title fade durations.
#[derive(Message, Debug, Clone)]
pub struct ClearTitles {
    /// A list of player entities that should receive this message.
    /// Listening systems should send a clear titles packet to all these clients.
    /// This may contain every entity on the server.
    pub receivers: Vec<Entity>,
}
