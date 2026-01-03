use crate::outgoing_packet::OutgoingPacket;
use bevy::ecs::message::Message;
use bevy::prelude::Entity;
use meloncraft_client::connection_state::ConnectionState;
use meloncraft_network::packet::OutgoingNetworkPacket;

#[derive(Message, Debug, Clone)]
pub struct KeepAlive {
    pub client: Entity,
}

impl OutgoingPacket for KeepAlive {
    fn id() -> i32 {
        0x04
    }

    fn state() -> ConnectionState {
        ConnectionState::Configuration
    }

    fn serialize(&self) -> Option<OutgoingNetworkPacket> {
        Some(OutgoingNetworkPacket {
            client: self.client,
            id: Self::id(),
            data: Vec::new(),
        })
    }
}
