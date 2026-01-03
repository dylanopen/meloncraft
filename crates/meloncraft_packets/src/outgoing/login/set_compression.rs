use bevy::ecs::message::Message;
use crate::outgoing_packet::OutgoingPacket;
use bevy::prelude::Entity;
use meloncraft_client::connection_state::ConnectionState;
use meloncraft_network::packet::OutgoingNetworkPacket;
use meloncraft_protocol_types::ProtocolType;

#[derive(Message, Debug, Clone)]
pub struct SetCompression {
    pub entity: Entity,
    pub threshold: i32,
}

impl OutgoingPacket for SetCompression {
    fn id() -> i32 {
        0x03
    }

    fn state() -> ConnectionState {
        ConnectionState::Login
    }

    fn serialize(&self) -> Option<OutgoingNetworkPacket> {
        Some(OutgoingNetworkPacket {
            client: self.entity,
            id: Self::id(),
            data: self.threshold.net_serialize(),
        })
    }
}
