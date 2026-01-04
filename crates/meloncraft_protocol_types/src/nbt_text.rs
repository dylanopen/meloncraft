use crate::{ProtocolBuffer, ProtocolType};
use meloncraft_nbt::{NbtTag, NbtValue};
use meloncraft_text::NbtText;

impl ProtocolType for NbtText {
    fn net_serialize(&self) -> Vec<u8> {
        match self {
            NbtText::Plain(nbt) => {
                NbtTag::new("".to_owned(), NbtValue::String(nbt.clone())).net_serialize()
            }
            NbtText::Formatted(nbt) => {
                NbtTag::new("".to_owned(), NbtValue::Compound(nbt.clone())).net_serialize()
            }
        }
    }

    fn net_deserialize(data: &mut Vec<u8>) -> Result<Self, ()> {
        let nbt: NbtTag = data.net_deserialize()?;
        match nbt.value {
            NbtValue::String(nbt) => Ok(NbtText::Plain(nbt)),
            NbtValue::Compound(nbt) => Ok(NbtText::Formatted(nbt)),
            _ => Err(()),
        }
    }
}
