use crate::ServerboundPacket;
use bevy::prelude::{Entity, Message};
use meloncraft_client::connection_state::ConnectionState;
use meloncraft_network::packet::ServerboundNetworkPacket;
use meloncraft_protocol_types::ProtocolType as _;

#[derive(Message, Debug, Clone)]
pub struct ServerboundStatusPing {
    pub client: Entity,
    pub timestamp: i64,
}

impl ServerboundPacket for ServerboundStatusPing {
    fn id() -> i32 {
        return 0x01
    }
    fn state() -> ConnectionState {
        return ConnectionState::Status
    }
    fn deserialize(packet: &ServerboundNetworkPacket) -> Option<Self> {
        let mut incoming = packet.clone();
        let client = incoming.client;
        let timestamp = i64::net_deserialize(&mut incoming.data).unwrap();
        return Some(ServerboundStatusPing { client, timestamp })
    }
}
