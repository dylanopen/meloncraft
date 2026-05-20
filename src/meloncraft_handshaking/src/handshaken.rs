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
    pub player: Entity,
}

#[derive(Message, Clone, Debug)]
pub struct LoginHandshaken {
    pub player: Entity,
}

#[derive(Message, Clone, Debug)]
pub struct TransferHandshaken {
    pub player: Entity,
}
