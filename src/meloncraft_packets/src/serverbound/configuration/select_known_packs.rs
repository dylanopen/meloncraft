use crate::ServerboundPacket;
use bevy::ecs::message::Message;
use bevy::prelude::Entity;
use meloncraft_client::connection_state::ConnectionState;
use meloncraft_core::datapack::DatapackMetadata;
use crate::network_messages::ServerboundNetworkPacket;
use meloncraft_protocol_types::{PrefixedArray, ProtocolBuffer as _};

#[derive(Message, Debug, Clone)]
pub struct ServerboundSelectKnownPacks {
    pub client: Entity,
    pub known_packs: Vec<DatapackMetadata>,
}

impl ServerboundPacket for ServerboundSelectKnownPacks {
    fn id() -> i32 {
        return 0x07
    }

    fn state() -> ConnectionState {
        return ConnectionState::Configuration
    }

    fn deserialize(packet: &ServerboundNetworkPacket) -> Option<Self> {
        let mut data = packet.data.clone();
        let client = packet.client;
        let known_packs: PrefixedArray<DatapackMetadata> = data.net_deserialize().unwrap();
        let known_packs = known_packs.0;
        return Some(Self {
            client,
            known_packs,
        })
    }
}
