use bevy::ecs::entity::Entity;
use bevy::ecs::message::Message;
use meloncraft_protocol_types::{GameEventType, ProtocolType};

use crate::clientbound_packet::ClientboundPacket;

#[derive(Message, Debug, Clone)]
pub struct ClientboundGameEvent {
    pub client: Entity,
    pub event: GameEventType,
}

impl ClientboundPacket for ClientboundGameEvent {
    fn id() -> i32 {
        0x26
    }

    fn state() -> meloncraft_client::connection_state::ConnectionState {
        meloncraft_client::connection_state::ConnectionState::Play
    }

    fn serialize(&self) -> Option<meloncraft_network::packet::ClientboundNetworkPacket> {
        let mut data = Vec::new();
        data.extend(self.event.net_serialize());

        Some(meloncraft_network::packet::ClientboundNetworkPacket {
            client: self.client,
            id: Self::id(),
            data,
        })
    }
}
