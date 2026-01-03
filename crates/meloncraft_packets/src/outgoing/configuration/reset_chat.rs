use crate::outgoing_packet::OutgoingPacket;
use bevy::ecs::message::Message;
use bevy::prelude::Entity;
use meloncraft_client::connection_state::ConnectionState;
use meloncraft_network::packet::OutgoingNetworkPacket;

#[derive(Message, Debug, Clone)]
pub struct ResetChat {
    pub client: Entity,
}

impl OutgoingPacket for ResetChat {
    fn id() -> i32 {
        0x06
    }

    fn state() -> ConnectionState {
        ConnectionState::Configuration
    }

    fn serialize(&self) -> Option<OutgoingNetworkPacket> {
        let data = Vec::new();
        Some(OutgoingNetworkPacket {
            client: self.client,
            id: Self::id(),
            data,
        })
    }
}
