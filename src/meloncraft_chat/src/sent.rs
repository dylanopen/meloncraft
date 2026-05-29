use bevy::ecs::entity::Entity;
use bevy::ecs::message::Message;

/// Message to send whenever a player sends a chat message to the server (regular chat, not a
/// command).
///
/// The `packet_forwarding` crate will write this packet for you, and the `chat_broadcasting` packet
/// will send [`SendChatMessage`] immediately after this packet is received.
#[derive(Message, Debug, Clone)]
pub struct PlayerSentChatMessage {
    /// The player's `Entity`, who sent the message.
    /// This stores all the components of the client that sent the message and can be used in a
    /// query to get more information about the client, e.g., their username.
    pub sender: Entity,

    /// The time that the client sent the message at.
    /// Sent as the number of milliseconds since the epoch.
    pub timestamp: u64,

    /// The actual message content itself, as a plain, unformatted string.
    pub message: String,
}
