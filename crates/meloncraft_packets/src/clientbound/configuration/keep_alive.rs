use crate::clientbound_packet::ClientboundPacket;
use bevy::ecs::message::Message;
use bevy::prelude::Entity;
use meloncraft_client::connection_state::ConnectionState;
use meloncraft_network::packet::ClientboundNetworkPacket;

#[derive(Message, Debug, Clone)]
pub struct KeepAlive {
    pub client: Entity,
}

impl ClientboundPacket for KeepAlive {
    fn id() -> i32 {
        0x04
    }

    fn state() -> ConnectionState {
        ConnectionState::Configuration
    }

    fn serialize(&self) -> Option<ClientboundNetworkPacket> {
        Some(ClientboundNetworkPacket {
            client: self.client,
            id: Self::id(),
            data: Vec::new(),
        })
    }
}
