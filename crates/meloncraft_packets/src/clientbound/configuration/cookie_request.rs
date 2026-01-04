use crate::clientbound_packet::ClientboundPacket;
use bevy::ecs::message::Message;
use bevy::prelude::Entity;
use meloncraft_client::connection_state::ConnectionState;
use meloncraft_core::Identifier;
use meloncraft_network::packet::ClientboundNetworkPacket;
use meloncraft_protocol_types::ProtocolType;

#[derive(Message, Debug, Clone)]
pub struct CookieRequest {
    pub client: Entity,
    pub key: Identifier,
}

impl ClientboundPacket for CookieRequest {
    fn id() -> i32 {
        0x00
    }

    fn state() -> ConnectionState {
        ConnectionState::Configuration
    }

    fn serialize(&self) -> Option<ClientboundNetworkPacket> {
        let mut data = Vec::new();
        data.append(&mut self.key.net_serialize());
        Some(ClientboundNetworkPacket {
            client: self.client,
            id: Self::id(),
            data,
        })
    }
}
