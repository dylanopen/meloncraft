use crate::ServerboundPacket;
use bevy::prelude::{Entity, Message};
use meloncraft_client::connection_state::ConnectionState;
use meloncraft_core::ResourcePackLoadResult;
use meloncraft_network::packet::ServerboundNetworkPacket;
use meloncraft_player::Uuid;
use meloncraft_protocol_types::{ProtocolBuffer, VarInt};

#[derive(Message, Debug, Clone)]
pub struct ResourcePackResponse {
    pub client: Entity,
    pub uuid: Uuid,
    pub load_result: ResourcePackLoadResult,
}

impl ServerboundPacket for ResourcePackResponse {
    fn id() -> i32 {
        0x06
    }
    fn state() -> ConnectionState {
        ConnectionState::Configuration
    }

    fn deserialize(packet: &ServerboundNetworkPacket) -> Option<Self> {
        let mut data = packet.data.clone();
        let client = packet.client;
        let uuid = data.net_deserialize().unwrap();
        let load_result: VarInt = data.net_deserialize().unwrap();
        let load_result = ResourcePackLoadResult::try_from(load_result.0).unwrap();

        Some(Self {
            client,
            uuid,
            load_result,
        })
    }
}
