use crate::clientbound_packet::ClientboundPacket;
use bevy::ecs::message::Message;
use bevy::prelude::Entity;
use meloncraft_client::connection_state::ConnectionState;
use meloncraft_network::packet::ClientboundNetworkPacket;
use meloncraft_protocol_types::{PrefixedArray, ProtocolType};
use meloncraft_registry::RegistryTags;

#[derive(Message, Debug, Clone)]
pub struct UpdateTags {
    pub client: Entity,
    registries: Vec<RegistryTags>,
}

impl ClientboundPacket for UpdateTags {
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
