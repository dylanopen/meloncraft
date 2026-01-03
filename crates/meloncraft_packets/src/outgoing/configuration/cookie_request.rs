use crate::outgoing_packet::OutgoingPacket;
use bevy::ecs::message::Message;
use bevy::prelude::Entity;
use meloncraft_client::connection_state::ConnectionState;
use meloncraft_network::packet::OutgoingNetworkPacket;
use meloncraft_protocol_types::{Identifier, ProtocolType};

#[derive(Message, Debug, Clone)]
pub struct CookieRequest {
    pub client: Entity,
    pub key: Identifier,
}

impl OutgoingPacket for CookieRequest {
    fn id() -> i32 {
        0x00
    }

    fn state() -> ConnectionState {
        ConnectionState::Configuration
    }

    fn serialize(&self) -> Option<OutgoingNetworkPacket> {
        let mut data = Vec::new();
        data.append(&mut self.key.net_serialize());
        Some(OutgoingNetworkPacket {
            client: self.client,
            id: Self::id(),
            data,
        })
    }
}
