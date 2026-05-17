use bevy::ecs::entity::Entity;
use bevy::ecs::message::Message;
use meloncraft_chunk::block_section::ChunkBlockSection;
use meloncraft_client::connection_state::ConnectionState;
use crate::network_messages::ClientboundNetworkPacket;
use meloncraft_protocol_types::{PrefixedArray, ProtocolType as _, VarInt};
use meloncraft_protocol_types::chunk_lighting::ChunkLighting;
use crate::clientbound_packet::ClientboundPacket;

#[derive(Message, Debug, Clone)]
pub struct ClientboundChunkData {
    pub client: Entity,
    pub chunk_x: i32,
    pub chunk_z: i32,
    pub data: Vec<ChunkBlockSection>,
    pub light: ChunkLighting,

    // TODO: heightmap and block entities
}

impl ClientboundPacket for ClientboundChunkData {
    fn id() -> i32 {
        return 0x2C
    }

    fn state() -> ConnectionState {
        return ConnectionState::Play
    }

    fn serialize(&self) -> Option<ClientboundNetworkPacket> {
        let mut data = Vec::new();

        data.extend(self.chunk_x.net_serialize());
        data.extend(self.chunk_z.net_serialize());
        data.extend(VarInt(0).net_serialize()); // heightmap array length 0, as not strictly required

        let mut data_bytes = Vec::new();
        for section in &self.data {
            data_bytes.extend(section.net_serialize());
        }
        data.extend(PrefixedArray(data_bytes).net_serialize());

        data.extend(VarInt(0).net_serialize()); // block entities array length 0, again, not strictly required
        data.extend(self.light.clone().net_serialize());

        return Some(ClientboundNetworkPacket {
            client: self.client,
            id: Self::id(),
            data,
        })
    }
}

