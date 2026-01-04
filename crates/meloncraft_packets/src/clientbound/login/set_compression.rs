use crate::clientbound_packet::ClientboundPacket;
use bevy::ecs::message::Message;
use bevy::prelude::Entity;
use meloncraft_client::connection_state::ConnectionState;
use meloncraft_network::packet::ClientboundNetworkPacket;
use meloncraft_protocol_types::ProtocolType;

#[derive(Message, Debug, Clone)]
pub struct SetCompression {
    pub entity: Entity,
    pub threshold: i32,
}

impl ClientboundPacket for SetCompression {
    fn id() -> i32 {
        0x03
    }

    fn state() -> ConnectionState {
        ConnectionState::Login
    }

    fn serialize(&self) -> Option<ClientboundNetworkPacket> {
        Some(ClientboundNetworkPacket {
            client: self.entity,
            id: Self::id(),
            data: self.threshold.net_serialize(),
        })
    }
}
