use crate::clientbound_packet::ClientboundPacket;
use bevy::ecs::message::Message;
use bevy::prelude::Entity;
use meloncraft_client::connection_state::ConnectionState;
use meloncraft_network::packet::ClientboundNetworkPacket;
use meloncraft_protocol_types::{ProtocolType, VarInt};

#[derive(Message, Debug, Clone)]
pub struct Transfer {
    pub client: Entity,
    pub hostname: String,
    pub port: u16,
}

impl ClientboundPacket for Transfer {
    fn id() -> i32 {
        0x0B
    }

    fn state() -> ConnectionState {
        ConnectionState::Configuration
    }

    fn serialize(&self) -> Option<ClientboundNetworkPacket> {
        let mut data = self.hostname.net_serialize();
        data.extend(VarInt(self.port.into()).net_serialize());
        Some(ClientboundNetworkPacket {
            client: self.client,
            id: Self::id(),
            data,
        })
    }
}
