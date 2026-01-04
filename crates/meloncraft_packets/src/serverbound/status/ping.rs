use crate::ServerboundPacket;
use bevy::prelude::{Entity, Message};
use meloncraft_client::connection_state::ConnectionState;
use meloncraft_network::packet::ServerboundNetworkPacket;
use meloncraft_protocol_types::ProtocolType;

#[derive(Message, Debug, Clone)]
pub struct Ping {
    pub client: Entity,
    pub timestamp: i64,
}

impl ServerboundPacket for Ping {
    fn id() -> i32 {
        0x01
    }
    fn state() -> ConnectionState {
        ConnectionState::Status
    }
    fn parse(incoming: &ServerboundNetworkPacket) -> Option<Self> {
        let mut incoming = incoming.clone();
        let client = incoming.client;
        let timestamp = i64::net_deserialize(&mut incoming.data).unwrap();
        Some(Ping { client, timestamp })
    }
}
