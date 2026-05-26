use crate::ServerboundPacket;
use bevy::prelude::{Entity, Message};
use meloncraft_client::connection_state::ConnectionState;
use crate::network_messages::ServerboundNetworkPacket;
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
    fn deserialize(mut packet: ServerboundNetworkPacket) -> Option<Self> {
        let client = packet.client;
        let timestamp = i64::net_deserialize(&mut packet.data).unwrap();
        return Some(ServerboundStatusPing { client, timestamp })
    }
}
