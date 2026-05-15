use bevy::ecs::message::Message;
use bevy::prelude::Entity;
use meloncraft_client::connection_state::ConnectionState;
use meloncraft_network::packet::ClientboundNetworkPacket;
use meloncraft_protocol_types::{ProtocolType, VarInt};
use crate::clientbound_packet::ClientboundPacket;

#[derive(Message, Debug, Clone)]
pub struct SetCenterChunk {
    pub client: Entity,
    pub x: i32,
    pub z: i32,
}

impl ClientboundPacket for SetCenterChunk {
    fn id() -> i32 {
        0x5C
    }

    fn state() -> ConnectionState {
        ConnectionState::Play
    }

    fn serialize(&self) -> Option<ClientboundNetworkPacket> {
        let mut data = Vec::new();

        data.extend(VarInt(self.x).net_serialize());
        data.extend(VarInt(self.z).net_serialize());

        Some(ClientboundNetworkPacket {
            client: self.client,
            id: Self::id(),
            data,
        })
    }
}

