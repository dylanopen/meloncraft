use bevy::ecs::message::Message;
use bevy::prelude::Entity;

/// Message representing that a client has sent a handshake to the server, with their next
/// *intention* being `Status`.
///
/// You should probably respond to this by updating the client's `ConnectionState` to `Status`.
///
/// This is sent when the client is querying the server's information, e.g. on the server list page.
/// You'll want to later respond with a `ClientboundStatusResponse` packet to send data such as the
/// server's MOTD.
#[derive(Message, Clone, Debug)]
pub struct StatusHandshaken {
    
    /// The entity that represents the client / player who sent the handshake.
    /// You can use this in other queries to add or get other components of this player.
    /// This can be useful if you, for example, want to check their `ClientConnection`'s IP address,
    /// to check whether they are IP-banned or not.
    pub player: Entity,
}

#[derive(Message, Clone, Debug)]
pub struct LoginHandshaken {

    /// The entity that represents the client / player who sent the handshake.
    /// You can use this in other queries to add or get other components of this player.
    /// This can be useful if you, for example, want to check their `ClientConnection`'s IP address,
    /// to check whether they are IP-banned or not.
    pub player: Entity,
}

#[derive(Message, Clone, Debug)]
pub struct TransferHandshaken {

    /// The entity that represents the client / player who sent the handshake.
    /// You can use this in other queries to add or get other components of this player.
    /// This can be useful if you, for example, want to check their `ClientConnection`'s IP address,
    /// to check whether they are IP-banned or not.
    pub player: Entity,
}
