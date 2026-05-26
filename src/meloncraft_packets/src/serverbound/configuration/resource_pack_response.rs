use crate::ServerboundPacket;
use bevy::prelude::{Entity, Message};
use meloncraft_client::connection_state::ConnectionState;
use meloncraft_core::ResourcePackLoadResult;
use crate::network_messages::ServerboundNetworkPacket;
use meloncraft_player::Uuid;
use meloncraft_protocol_types::{ProtocolBuffer as _, VarInt};

#[derive(Message, Debug, Clone)]
pub struct ServerboundResourcePackResponse {
    pub client: Entity,
    pub uuid: Uuid,
    pub load_result: ResourcePackLoadResult,
}

impl ServerboundPacket for ServerboundResourcePackResponse {
    fn id() -> i32 {
        return 0x06
    }
    fn state() -> ConnectionState {
        return ConnectionState::Configuration
    }

    fn deserialize(packet: ServerboundNetworkPacket) -> Option<Self> {
        let mut data = packet.data.clone();
        let client = packet.client;
        let uuid = data.net_deserialize().unwrap();
        let load_result: VarInt = data.net_deserialize().unwrap();
        let load_result = ResourcePackLoadResult::try_from(load_result.0).unwrap();

        return Some(Self {
            client,
            uuid,
            load_result,
        })
    }
}
