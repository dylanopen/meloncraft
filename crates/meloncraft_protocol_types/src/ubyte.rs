use crate::{ProtocolBuffer as _, ProtocolType};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct UByte(pub u8);

impl ProtocolType for UByte {
    fn net_serialize(&self) -> Vec<u8> {
        return self.0.net_serialize();
    }

    fn net_deserialize(data: &mut Vec<u8>) -> Result<Self, ()> {
        return Ok(Self(data.net_deserialize()?));
    }
}
