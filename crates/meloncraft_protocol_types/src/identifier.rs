use crate::{ProtocolBuffer as _, ProtocolType};
use meloncraft_core::Identifier;

impl ProtocolType for Identifier {
    fn net_serialize(&self) -> Vec<u8> {
        return self.0.net_serialize();
    }

    fn net_deserialize(data: &mut Vec<u8>) -> Result<Self, ()> {
        return Ok(Self(data.net_deserialize()?));
    }
}
