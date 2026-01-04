use crate::clientbound_packet::ClientboundPacket;
use bevy::ecs::message::Message;
use bevy::prelude::Entity;
use meloncraft_client::connection_state::ConnectionState;
use meloncraft_core::Identifier;
use meloncraft_network::packet::ClientboundNetworkPacket;
use meloncraft_protocol_types::{Byte, PrefixedArray, ProtocolType};

#[derive(Message, Debug, Clone)]
pub struct StoreCookie {
    pub client: Entity,
    pub key: Identifier,
    pub value: Vec<Byte>,
}

impl ClientboundPacket for StoreCookie {
    fn id() -> i32 {
        0x0A
    }

    fn state() -> ConnectionState {
        ConnectionState::Configuration
    }

    fn serialize(&self) -> Option<ClientboundNetworkPacket> {
        let mut data = self.key.net_serialize();
        data.extend(PrefixedArray(self.value.clone()).net_serialize());
        Some(ClientboundNetworkPacket {
            client: self.client,
            id: Self::id(),
            data,
        })
    }
}
