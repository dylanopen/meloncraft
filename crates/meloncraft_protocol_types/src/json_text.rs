use crate::{ProtocolBuffer, ProtocolType};
use meloncraft_text::JsonText;

impl ProtocolType for JsonText {
    fn net_serialize(&self) -> Vec<u8> {
        self.data.net_serialize()
    }
    fn net_deserialize(data: &mut Vec<u8>) -> Result<Self, ()> {
        Ok(JsonText {
            data: data.net_deserialize()?,
        })
    }
}
