use crate::{PrefixedArray, ProtocolType};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChunkSectionLightData {
    pub data: [u8; 2048],
}

impl ProtocolType for ChunkSectionLightData {
    fn net_serialize(&self) -> Vec<u8> {
        PrefixedArray(self.data.to_vec()).net_serialize()
    }

    fn net_deserialize(data: &mut Vec<u8>) -> Result<Self, ()> {
        let array = PrefixedArray::net_deserialize(data)?.0;
        if array.len() != 2048 {
            return Err(());
        }
        let mut data_array = [0u8; 2048];
        data_array.copy_from_slice(&array);
        Ok(Self { data: data_array })
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chunk_section_light_data_serde() {
        let light_data = ChunkSectionLightData { data: [1u8; 2048] };
        let serialized = light_data.net_serialize();
        let deserialized = ChunkSectionLightData::net_deserialize(&mut serialized.clone()).unwrap();
        assert_eq!(light_data.data, deserialized.data);
    }
}
