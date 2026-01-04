use crate::clientbound_packet::ClientboundPacket;
use bevy::ecs::message::Message;
use bevy::prelude::Entity;
use meloncraft_client::connection_state::ConnectionState;
use meloncraft_network::packet::ClientboundNetworkPacket;
use meloncraft_protocol_types::{JsonText, ProtocolType};

#[derive(Message, Debug, Clone)]
pub struct Disconnect {
    pub client: Entity,
    pub reason: JsonText,
}

impl ClientboundPacket for Disconnect {
    fn id() -> i32 {
        0x00
    }

    fn state() -> ConnectionState {
        ConnectionState::Login
    }

    fn serialize(&self) -> Option<ClientboundNetworkPacket> {
        Some(ClientboundNetworkPacket {
            client: self.client,
            id: Self::id(),
            data: self.reason.net_serialize(),
        })
    }
}
