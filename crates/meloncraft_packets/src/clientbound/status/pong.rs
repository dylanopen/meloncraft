use crate::clientbound_packet::ClientboundPacket;
use bevy::prelude::{Entity, Message};
use meloncraft_client::connection_state::ConnectionState;
use meloncraft_network::packet::ClientboundNetworkPacket;
use meloncraft_protocol_types::ProtocolType;

#[derive(Message, Debug, Clone)]
pub struct Pong {
    pub client: Entity,
    pub timestamp: i64,
}

impl ClientboundPacket for Pong {
    fn id() -> i32 {
        0x01
    }
    fn state() -> ConnectionState {
        ConnectionState::Status
    }
    fn serialize(&self) -> Option<ClientboundNetworkPacket> {
        Some(ClientboundNetworkPacket {
            client: self.client,
            id: Self::id(),
            data: self.timestamp.net_serialize(),
        })
    }
}
