use crate::clientbound_packet::ClientboundPacket;
use bevy::prelude::{Entity, Message};
use meloncraft_client::connection_state::ConnectionState;
use meloncraft_protocol_types::ProtocolType as _;

#[derive(Message, Debug, Clone)]
pub struct ClientboundPong {
    pub client: Entity,
    pub timestamp: i64,
}

impl ClientboundPacket for ClientboundPong {
    fn id() -> i32 {
        return 0x01
    }
    fn state() -> ConnectionState {
        return ConnectionState::Status
    }

    fn client(&self) -> Entity {
        return self.client;
    }

    fn data(&self, data: &mut Vec<u8>) {
        data.extend(self.timestamp.net_serialize());
    }
}
