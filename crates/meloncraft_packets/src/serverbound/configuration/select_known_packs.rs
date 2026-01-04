use crate::ServerboundPacket;
use bevy::ecs::message::Message;
use bevy::prelude::Entity;
use meloncraft_client::connection_state::ConnectionState;
use meloncraft_core::datapack::DatapackMetadata;
use meloncraft_network::packet::ServerboundNetworkPacket;
use meloncraft_protocol_types::{PrefixedArray, ProtocolBuffer};

#[derive(Message, Debug, Clone)]
pub struct SelectKnownPacks {
    pub client: Entity,
    pub known_packs: Vec<DatapackMetadata>,
}

impl ServerboundPacket for SelectKnownPacks {
    fn id() -> i32 {
        0x07
    }

    fn state() -> ConnectionState {
        ConnectionState::Configuration
    }

    fn deserialize(packet: &ServerboundNetworkPacket) -> Option<Self> {
        let mut data = packet.data.clone();
        let client = packet.client;
        let known_packs: PrefixedArray<DatapackMetadata> = data.net_deserialize().unwrap();
        let known_packs = known_packs.0;
        Some(Self {
            client,
            known_packs,
        })
    }
}
