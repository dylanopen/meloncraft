use crate::clientbound_packet::ClientboundPacket;
use bevy::ecs::message::Message;
use bevy::prelude::Entity;
use meloncraft_client::connection_state::ConnectionState;
use meloncraft_network::packet::ClientboundNetworkPacket;

#[derive(Message, Clone, Debug)]
pub struct ClientboundFinishConfiguration {
    pub client: Entity,
}

impl ClientboundPacket for ClientboundFinishConfiguration {
    fn id() -> i32 {
        return 0x03
    }

    fn state() -> ConnectionState {
        return ConnectionState::Configuration
    }

    fn serialize(&self) -> Option<ClientboundNetworkPacket> {
        return Some(ClientboundNetworkPacket {
            client: self.client,
            id: Self::id(),
            data: Vec::new(),
        })
    }
}
