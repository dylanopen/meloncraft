use crate::bitset::BitSet;
use crate::{PrefixedArray, ProtocolBuffer as _, ProtocolType};
use crate::chunk_section_light_data::ChunkSectionLightData;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChunkLighting {
    pub sky_mask: BitSet,
    pub block_mask: BitSet,
    pub empty_sky_mask: BitSet,
    pub empty_block_mask: BitSet,
    pub sky_data: Vec<ChunkSectionLightData>,
    pub block_data: Vec<ChunkSectionLightData>,
}

impl ProtocolType for ChunkLighting {
    fn net_serialize(&self) -> Vec<u8> {
        let mut serial = Vec::new();
        serial.extend(self.sky_mask.net_serialize());
        serial.extend(self.block_mask.net_serialize());
        serial.extend(self.empty_sky_mask.net_serialize());
        serial.extend(self.empty_block_mask.net_serialize());
        serial.extend(PrefixedArray(self.sky_data.clone()).net_serialize());
        serial.extend(PrefixedArray(self.block_data.clone()).net_serialize());
        return serial;
    }

    fn net_deserialize(data: &mut Vec<u8>) -> Result<Self, ()> {
        let sky_mask = data.net_deserialize()?;
        let block_mask = data.net_deserialize()?;
        let empty_sky_mask = data.net_deserialize()?;
        let empty_block_mask = data.net_deserialize()?;
        let sky_data = PrefixedArray::net_deserialize(data)?.0;
        let block_data = PrefixedArray::net_deserialize(data)?.0;

        return Ok(Self {
            sky_mask,
            block_mask,
            empty_sky_mask,
            empty_block_mask,
            sky_data,
            block_data,
        });
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chunk_lighting_serde() {
        let chunk_lighting = ChunkLighting {
            sky_mask: BitSet { bits: vec![0b10101010, 0b01010101] },
            block_mask: BitSet { bits: vec![0b11110000, 0b00001111] },
            empty_sky_mask: BitSet { bits: vec![0b11001100, 0b00110011] },
            empty_block_mask: BitSet { bits: vec![0b11111111, 0b00000000] },
            sky_data: vec![
                ChunkSectionLightData { data: [1u8; 2048] },
                ChunkSectionLightData { data: [2u8; 2048] },
            ],
            block_data: vec![
                ChunkSectionLightData { data: [3u8; 2048] },
                ChunkSectionLightData { data: [4u8; 2048] },
            ],
        };
        let mut serialized = chunk_lighting.net_serialize();
        let deserialized = serialized.net_deserialize();
        assert_eq!(chunk_lighting, deserialized.unwrap());
    }
}
