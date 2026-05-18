use bevy::ecs::message::Message;
use bevy::prelude::Entity;
use meloncraft_client::connection_state::ConnectionState;
use crate::network_messages::ClientboundNetworkPacket;
use meloncraft_protocol_types::{ProtocolType as _, VarInt};
use crate::clientbound_packet::ClientboundPacket;

#[derive(Message, Debug, Clone)]
pub struct ClientboundSetCenterChunk {
    pub client: Entity,
    pub x: i32,
    pub z: i32,
}

impl ClientboundPacket for ClientboundSetCenterChunk {
    fn id() -> i32 {
        return 0x5C
    }

    fn state() -> ConnectionState {
        return ConnectionState::Play
    }

    fn serialize(&self) -> Option<ClientboundNetworkPacket> {
        let mut data = Vec::new();

        data.extend(VarInt(self.x).net_serialize());
        data.extend(VarInt(self.z).net_serialize());

        return Some(ClientboundNetworkPacket {
            client: self.client,
            id: Self::id(),
            data,
        })
    }
}

