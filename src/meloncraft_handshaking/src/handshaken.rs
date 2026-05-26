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

/// Message representing that a client has sent a handshake to the server, with their next
/// *intention* being `Login`.
///
/// You should probably respond to this by updating the client's `ConnectionState` to `Login`, if
/// they are allowed to log in to the server.
///
/// This is sent when the client is joining the server from the server list page (typing in the IP
/// themselves). If they were transferred to this server from a different one, a
/// [`TransferHandshaken`] message should be sent instead, not a [`LoginHandshaken`], because you
/// may want to process these differently (e.g. if you don't accept transfers).
#[derive(Message, Clone, Debug)]
pub struct LoginHandshaken {
    /// The entity that represents the client / player who sent the handshake.
    /// You can use this in other queries to add or get other components of this player.
    /// This can be useful if you, for example, want to check their `ClientConnection`'s IP address,
    /// to check whether they are IP-banned or not.
    pub player: Entity,
}

/// Message representing that a client has sent a handshake to the server, with their next
/// *state intention* being `Login`, but they were *transferred* from a different server to this
/// one.
///
/// You should probably respond to this by updating the client's `ConnectionState` to `Login`, if
/// they are allowed to log in to the server and you want to enable transfers **to** your server.
/// You may then want to start the login process, e.g. with a `ClientboundLoginStart` packet.
///
/// This is sent when the client is joining the server because another server transferred them here.
/// If they were not transferred to this server from a different one but instead joined the server
/// from the Multiplayer server list, a [`LoginHandshaken`] message should be sent instead, not a
/// [`TransferHandshaken`], because you may want to process these differently.
#[derive(Message, Clone, Debug)]
pub struct TransferHandshaken {
    /// The entity that represents the client / player who sent the handshake.
    /// You can use this in other queries to add or get other components of this player.
    /// This can be useful if you, for example, want to check their `ClientConnection`'s IP address,
    /// to check whether they are IP-banned or not.
    pub player: Entity,
}
