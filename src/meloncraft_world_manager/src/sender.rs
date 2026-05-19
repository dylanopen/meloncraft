use bevy::prelude::{MessageReader, MessageWriter};
use meloncraft_packets::ClientboundChunkData;
use meloncraft_world::messages::SendChunk;
use meloncraft_protocol_types::bitset::BitSet;
use meloncraft_protocol_types::chunk_lighting::ChunkLighting;

pub fn send_chunk(
    mut send_chunk_mr: MessageReader<SendChunk>,
    mut chunk_data_pw: MessageWriter<ClientboundChunkData>,
) {
    for send_chunk in send_chunk_mr.read() {
        chunk_data_pw.write(ClientboundChunkData {
            client: send_chunk.client,
            chunk_x: send_chunk.chunk_x,
            chunk_z: send_chunk.chunk_z,
            data: send_chunk.chunk.to_chunk_sections(),
            // Lighting is purely temporary:
            light: ChunkLighting {
                sky_mask: BitSet::with_capacity(send_chunk.chunk.get_height_in_chunks() + 2),
                block_mask: BitSet::with_capacity(send_chunk.chunk.get_height_in_chunks() + 2),
                empty_sky_mask: BitSet::with_capacity(send_chunk.chunk.get_height_in_chunks() + 2),
                empty_block_mask: BitSet::with_capacity(send_chunk.chunk.get_height_in_chunks() + 2),
                sky_data: vec![],
                block_data: vec![],
            },
        });
    }
}
