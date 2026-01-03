use crate::IncomingPacket;
use bevy::ecs::message::Message;
use bevy::prelude::Entity;
use meloncraft_client::connection_state::ConnectionState;
use meloncraft_network::packet::IncomingNetworkPacket;

#[derive(Message, Debug, Clone)]
pub struct LoginAcknowledged {
    pub client: Entity,
}

impl IncomingPacket for LoginAcknowledged {
    fn id() -> i32 {
        0x03
    }

    fn state() -> ConnectionState {
        ConnectionState::Login
    }

    fn parse(incoming: &IncomingNetworkPacket) -> Option<Self> {
        Some(Self {
            client: incoming.client,
        })
    }
}
