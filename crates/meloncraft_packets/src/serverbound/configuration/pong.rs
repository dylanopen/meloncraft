use crate::ServerboundPacket;
use bevy::prelude::{Entity, Message};
use meloncraft_client::connection_state::ConnectionState;
use meloncraft_network::packet::ServerboundNetworkPacket;
use meloncraft_protocol_types::ProtocolBuffer;

#[derive(Message, Debug, Clone)]
pub struct Pong {
    pub client: Entity,
    pub id: i32,
}

impl ServerboundPacket for Pong {
    fn id() -> i32 {
        0x05
    }
    fn state() -> ConnectionState {
        ConnectionState::Configuration
    }

    fn deserialize(packet: &ServerboundNetworkPacket) -> Option<Self> {
        let mut data = packet.data.clone();
        let client = packet.client;
        let id = data.net_deserialize().unwrap();

        Some(Self { client, id })
    }
}
