use meloncraft_chunk::block_section::ChunkBlockSection;
use crate::ProtocolType;

const BITS_PER_BLOCK_ENTRY: u8 = 15;
const BITS_PER_BIOME_ENTRY: u8 = 7;

impl ProtocolType for ChunkBlockSection {
    fn net_serialize(&self) -> Vec<u8> {
        let mut output = Vec::new();
        output.extend(self.block_count.net_serialize());

        // blocks:
        output.extend(BITS_PER_BLOCK_ENTRY.net_serialize());
        let mut current_long = 0u64;
        let mut entries_in_long = 0;
        for block in &self.blocks {
            current_long <<= 15;
            current_long |= (block.state_id as u64) & 0x7FFF;
            entries_in_long += 1;
            if entries_in_long >= 64 / BITS_PER_BLOCK_ENTRY {
                output.extend(current_long.net_serialize());
                current_long = 0;
                entries_in_long = 0;
            }
        }
        if entries_in_long > 0 {
            output.extend(current_long.net_serialize());
        }

        // biomes:
        output.extend(BITS_PER_BIOME_ENTRY.net_serialize());
        current_long = 0;
        entries_in_long = 0;
        for biome in &self.biomes {
            current_long <<= 7;
            current_long |= (biome.state_id as u64) & 0x7FFF;
            entries_in_long += 1;
            if entries_in_long >= 64 / BITS_PER_BIOME_ENTRY {
                output.extend(current_long.net_serialize());
                current_long = 0;
                entries_in_long = 0;
            }
        }

        output
    }

    fn net_deserialize(_data: &mut Vec<u8>) -> Result<Self, ()> {
        todo!("Hopefully the client doesn't send chunk sections, as then we will need this to be deserializable at some point.")
    }
}