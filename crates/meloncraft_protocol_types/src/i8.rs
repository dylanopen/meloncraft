use crate::ProtocolType;

impl ProtocolType for i8 {
    fn net_serialize(&self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }
    fn net_deserialize(data: &mut Vec<u8>) -> Result<Self, ()> {
        Ok(Self::from_be_bytes([data.remove(0)]))
    }
}
