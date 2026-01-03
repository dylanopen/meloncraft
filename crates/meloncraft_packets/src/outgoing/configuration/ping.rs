use crate::outgoing_packet::OutgoingPacket;
use bevy::ecs::message::Message;
use bevy::prelude::Entity;
use meloncraft_client::connection_state::ConnectionState;
use meloncraft_network::packet::OutgoingNetworkPacket;
use meloncraft_protocol_types::ProtocolType;

#[derive(Message, Debug, Clone)]
pub struct Ping {
    pub client: Entity,
    pub id: i32,
}

impl OutgoingPacket for Ping {
    fn id() -> i32 {
        0x05
    }

    fn state() -> ConnectionState {
        ConnectionState::Configuration
    }

    fn serialize(&self) -> Option<OutgoingNetworkPacket> {
        let data = self.id.net_serialize();
        Some(OutgoingNetworkPacket {
            client: self.client,
            id: Self::id(),
            data,
        })
    }
}
