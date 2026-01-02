use crate::ProtocolType;

impl ProtocolType for u8 {
    fn net_serialize(&self) -> Vec<u8> {
        vec![*self]
    }

    fn net_deserialize(data: &mut Vec<u8>) -> Result<Self, ()> {
        Ok(data.remove(0))
    }
}
