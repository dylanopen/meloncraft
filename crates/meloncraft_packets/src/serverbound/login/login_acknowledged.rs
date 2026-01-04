use crate::ServerboundPacket;
use bevy::ecs::message::Message;
use bevy::prelude::Entity;
use meloncraft_client::connection_state::ConnectionState;
use meloncraft_network::packet::ServerboundNetworkPacket;

#[derive(Message, Debug, Clone)]
pub struct LoginAcknowledged {
    pub client: Entity,
}

impl ServerboundPacket for LoginAcknowledged {
    fn id() -> i32 {
        0x03
    }

    fn state() -> ConnectionState {
        ConnectionState::Login
    }

    fn deserialize(incoming: &ServerboundNetworkPacket) -> Option<Self> {
        Some(Self {
            client: incoming.client,
        })
    }
}
