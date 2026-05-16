use crate::clientbound_packet::ClientboundPacket;
use bevy::ecs::message::Message;
use bevy::prelude::Entity;
use meloncraft_client::connection_state::ConnectionState;
use meloncraft_network::packet::ClientboundNetworkPacket;
use meloncraft_protocol_types::{PrefixedArray, ProtocolType as _};
use meloncraft_registry::RegistryTags;

#[derive(Message, Debug, Clone)]
pub struct ClientboundUpdateTags {
    pub client: Entity,
    pub registries: Vec<RegistryTags>,
}

impl ClientboundPacket for ClientboundUpdateTags {
    fn id() -> i32 {
        0x0D
    }

    fn state() -> ConnectionState {
        ConnectionState::Configuration
    }

    fn serialize(&self) -> Option<ClientboundNetworkPacket> {
        let data = PrefixedArray(self.registries.clone()).net_serialize();
        Some(ClientboundNetworkPacket {
            client: self.client,
            id: Self::id(),
            data,
        })
    }
}
