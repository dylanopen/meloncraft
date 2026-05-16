use crate::clientbound_packet::ClientboundPacket;
use bevy::ecs::message::Message;
use bevy::prelude::Entity;
use meloncraft_client::connection_state::ConnectionState;
use meloncraft_network::packet::ClientboundNetworkPacket;

#[derive(Message, Debug, Clone)]
pub struct ClientboundClearDialog {
    pub client: Entity,
}

impl ClientboundPacket for ClientboundClearDialog {
    fn id() -> i32 {
        return 0x11
    }

    fn state() -> ConnectionState {
        return ConnectionState::Configuration
    }

    fn serialize(&self) -> Option<ClientboundNetworkPacket> {
        let data = Vec::new();
        return Some(ClientboundNetworkPacket {
            client: self.client,
            id: Self::id(),
            data,
        })
    }
}
