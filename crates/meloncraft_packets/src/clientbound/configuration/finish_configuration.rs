use crate::clientbound_packet::ClientboundPacket;
use bevy::ecs::message::Message;
use bevy::prelude::Entity;
use meloncraft_client::connection_state::ConnectionState;
use meloncraft_network::packet::ClientboundNetworkPacket;

#[derive(Message, Clone, Debug)]
pub struct FinishConfiguration {
    pub client: Entity,
}

impl ClientboundPacket for FinishConfiguration {
    fn id() -> i32 {
        0x03
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
