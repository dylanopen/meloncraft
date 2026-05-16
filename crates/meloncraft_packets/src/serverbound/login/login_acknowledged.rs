use crate::ServerboundPacket;
use bevy::ecs::message::Message;
use bevy::prelude::Entity;
use meloncraft_client::connection_state::ConnectionState;
use meloncraft_network::packet::ServerboundNetworkPacket;

#[derive(Message, Debug, Clone)]
pub struct ServerboundLoginAcknowledged {
    pub client: Entity,
}

impl ServerboundPacket for ServerboundLoginAcknowledged {
    fn id() -> i32 {
        0x03
    }

    fn state() -> ConnectionState {
        ConnectionState::Login
    }

    fn deserialize(packet: &ServerboundNetworkPacket) -> Option<Self> {
        Some(Self {
            client: packet.client,
        })
    }
}
