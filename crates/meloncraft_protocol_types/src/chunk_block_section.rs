use meloncraft_chunk::block_section::ChunkBlockSection;
use crate::ProtocolType;

const BITS_PER_BLOCK_ENTRY: u8 = 15;
const BITS_PER_BIOME_ENTRY: u8 = 7;

impl ProtocolType for ChunkBlockSection {
    fn net_serialize(&self) -> Vec<u8> {
        let mut output = Vec::new();
        output.extend(self.block_count.net_serialize());

        dbg!(output.len());

        // Some code was adapted from Oxide: thank you, as without it, I was completely lost with this serialization <3
        // https://git.thetxt.io/thetxt/oxide/src/lib/src/packets/clientbound/play.rs
        let mut data_iter = self.blocks.into_iter();
        let entries_per_long = 64 / BITS_PER_BLOCK_ENTRY;
        let bits_per_entry = BITS_PER_BLOCK_ENTRY;
        output.push(bits_per_entry);
        while data_iter.len() != 0 {
            let mut entry: u64 = 0;
            for i in 0..entries_per_long {
                if let Some(next) = data_iter.next() {
                    entry += u64::try_from(next.state_id).unwrap() << u64::from(i * bits_per_entry);
                }
            }
            output.extend(entry.net_serialize());
        }
        let mut data_iter = self.biomes.into_iter();
        let entries_per_long = 64 / BITS_PER_BIOME_ENTRY;
        let bits_per_entry = BITS_PER_BIOME_ENTRY;
        output.push(bits_per_entry);
        while data_iter.len() != 0 {
            let mut entry: u64 = 0;
            for i in 0..entries_per_long {
                if let Some(next) = data_iter.next() {
                    entry += u64::try_from(next.state_id).unwrap() << u64::from(i * bits_per_entry);
                }
            }
            output.extend(entry.net_serialize());
        }

        output
    }

    fn net_deserialize(_data: &mut Vec<u8>) -> Result<Self, ()> {
        todo!("Hopefully the client doesn't send chunk sections, as then we will need this to be deserializable at some point.")
    }
}