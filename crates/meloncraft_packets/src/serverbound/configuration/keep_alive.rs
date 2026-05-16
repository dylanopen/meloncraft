use crate::ServerboundPacket;
use bevy::prelude::{Entity, Message};
use meloncraft_client::connection_state::ConnectionState;
use meloncraft_network::packet::ServerboundNetworkPacket;
use meloncraft_protocol_types::ProtocolBuffer as _;

#[derive(Message, Debug, Clone)]
pub struct ServerboundConfigurationKeepAlive {
    pub client: Entity,
    pub id: i64,
}

impl ServerboundPacket for ServerboundConfigurationKeepAlive {
    fn id() -> i32 {
        return 0x04
    }
    fn state() -> ConnectionState {
        return ConnectionState::Configuration
    }

    fn deserialize(packet: &ServerboundNetworkPacket) -> Option<Self> {
        let mut data = packet.data.clone();
        let client = packet.client;
        let id = data.net_deserialize().unwrap();

        return Some(Self { client, id })
    }
}
