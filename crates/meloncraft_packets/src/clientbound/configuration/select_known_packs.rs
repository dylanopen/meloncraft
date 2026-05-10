use crate::clientbound_packet::ClientboundPacket;
use bevy::ecs::message::Message;
use bevy::prelude::Entity;
use meloncraft_client::connection_state::ConnectionState;
use meloncraft_core::datapack::DatapackMetadata;
use meloncraft_network::packet::ClientboundNetworkPacket;
use meloncraft_protocol_types::{PrefixedArray, ProtocolType};

#[derive(Message, Debug, Clone)]
pub struct SelectKnownPacks {
    pub client: Entity,
    pub known_packs: Vec<DatapackMetadata>,
}

impl ClientboundPacket for SelectKnownPacks {
    fn id() -> i32 {
        0x0E
    }

    fn state() -> ConnectionState {
        ConnectionState::Configuration
    }

    fn serialize(&self) -> Option<ClientboundNetworkPacket> {
        let data = PrefixedArray(self.known_packs.clone()).net_serialize();
        Some(ClientboundNetworkPacket {
            client: self.client,
            id: Self::id(),
            data,
        })
    }
}
