use crate::clientbound_packet::ClientboundPacket;
use bevy::ecs::message::Message;
use bevy::prelude::Entity;
use meloncraft_client::connection_state::ConnectionState;
use meloncraft_core::Identifier;
use meloncraft_network::packet::ClientboundNetworkPacket;
use meloncraft_protocol_types::{PrefixedArray, ProtocolType};
use meloncraft_registry::RegistryEntry;

#[derive(Message, Debug, Clone)]
pub struct RegistryData {
    pub client: Entity,
    pub registry_id: Identifier,
    pub registry_entries: Vec<RegistryEntry>,
}

impl ClientboundPacket for RegistryData {
    fn id() -> i32 {
        0x07
    }

    fn state() -> ConnectionState {
        ConnectionState::Configuration
    }

    fn serialize(&self) -> Option<ClientboundNetworkPacket> {
        let mut data = self.registry_id.net_serialize();
        data.extend(PrefixedArray(self.registry_entries.clone()).net_serialize());
        Some(ClientboundNetworkPacket {
            client: self.client,
            id: Self::id(),
            data,
        })
    }
}
