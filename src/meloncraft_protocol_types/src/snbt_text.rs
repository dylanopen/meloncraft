use crate::{ProtocolBuffer as _, ProtocolType};
use meloncraft_text::SnbtText;

impl ProtocolType for SnbtText {
    fn net_serialize(&self) -> Vec<u8> {
        return self.data.net_serialize();
    }
    fn net_deserialize(data: &mut Vec<u8>) -> Result<Self, ()> {
        return Ok(SnbtText {
            data: data.net_deserialize()?,
        });
    }
}
