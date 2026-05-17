use crate::ServerboundPacket;
use bevy::prelude::{Entity, Message};
use meloncraft_client::connection_state::ConnectionState;
use crate::network_messages::ServerboundNetworkPacket;
use meloncraft_protocol_types::ProtocolBuffer as _;

#[derive(Message, Debug, Clone)]
pub struct ServerboundConfigurationPong {
    pub client: Entity,
    pub id: i32,
}

impl ServerboundPacket for ServerboundConfigurationPong {
    fn id() -> i32 {
        return 0x05
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
