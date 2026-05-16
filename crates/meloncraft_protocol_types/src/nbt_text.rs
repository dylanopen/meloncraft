use crate::{ProtocolBuffer as _, ProtocolType};
use meloncraft_nbt::{NbtTag, NbtValue};
use meloncraft_text::NbtText;

impl ProtocolType for NbtText {
    fn net_serialize(&self) -> Vec<u8> {
        match self {
            NbtText::Plain(nbt) => {
                NbtTag::new(String::new(), NbtValue::String(nbt.clone())).net_serialize()
            }
            NbtText::Formatted(nbt) => {
                NbtTag::new(String::new(), NbtValue::Compound(nbt.clone())).net_serialize()
            }
        }
    }

    fn net_deserialize(data: &mut Vec<u8>) -> Result<Self, ()> {
        let nbt: NbtTag = data.net_deserialize()?;
        #[expect(clippy::wildcard_enum_match_arm, reason = "Any future NbtValue variants will not be deserializable to text, so we will always ignore other variants.")]
        match nbt.value {
            NbtValue::String(nbt) => Ok(NbtText::Plain(nbt)),
            NbtValue::Compound(nbt) => Ok(NbtText::Formatted(nbt)),
            _ => Err(()),
        }
    }
}
