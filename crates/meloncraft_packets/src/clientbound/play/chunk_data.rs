use bevy::ecs::entity::Entity;
use bevy::ecs::message::Message;
use meloncraft_client::connection_state::ConnectionState;
use meloncraft_network::packet::ClientboundNetworkPacket;
use meloncraft_protocol_types::{PrefixedArray, ProtocolType, VarInt};

use crate::clientbound_packet::ClientboundPacket;

#[derive(Message, Debug, Clone)]
pub struct ChunkData {
    pub client: Entity,
    pub chunk_x: i32,
    pub chunk_z: i32,
    pub data: Vec<u8>,
    pub light: Vec<u8>,

    // TODO: heightmap and block entities
}

impl ClientboundPacket for ChunkData {
    fn id() -> i32 {
        0x2C
    }

    fn state() -> ConnectionState {
        ConnectionState::Play
    }

    fn serialize(&self) -> Option<ClientboundNetworkPacket> {
        let mut data = Vec::new();

            data.extend(VarInt(self.chunk_x).net_serialize());
            data.extend(VarInt(self.chunk_z).net_serialize());
            data.push(0); // heightmap array length 0, as not strictly required
            data.extend(PrefixedArray(self.data.clone()).net_serialize());
            data.push(0); // block entities array length 0, again, not strictly required
            data.extend(PrefixedArray(self.light.clone()).net_serialize());

        Some(meloncraft_network::packet::ClientboundNetworkPacket {
            client: self.client,
            id: Self::id(),
            data,
        })
    }
}

