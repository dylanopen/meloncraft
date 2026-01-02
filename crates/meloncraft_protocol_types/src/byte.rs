use crate::{ProtocolBuffer, ProtocolType};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Byte(pub i8);

impl ProtocolType for Byte {
    fn net_serialize(&self) -> Vec<u8> {
        self.0.net_serialize()
    }

    fn net_deserialize(data: &mut Vec<u8>) -> Result<Self, ()> {
        Ok(Self(data.net_deserialize()?))
    }
}
