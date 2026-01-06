mod serialize;
mod deserialize;

use crate::{ProtocolBuffer, ProtocolType};
use meloncraft_nbt::{NbtTag, NbtValue};

impl ProtocolType for NbtTag {
    fn net_serialize(&self) -> Vec<u8> {
        serialize::tag(self.clone())
    }

    fn net_deserialize(data: &mut Vec<u8>) -> Result<Self, ()> {
        deserialize::tag(data)
    }
}

impl ProtocolType for NbtValue {
    fn net_serialize(&self) -> Vec<u8> {
        serialize::value(self.clone())
    }

    fn net_deserialize(data: &mut Vec<u8>) -> Result<Self, ()> {
        let tag_type = data.net_deserialize()?;
        deserialize::value(tag_type, data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use meloncraft_nbt::NbtList;
    use meloncraft_nbt::NbtTag;
    use meloncraft_nbt::NbtValue;

    #[test]
    fn test_serialize_deserialize() {
        // very complex NBT tag:

        let original_tag = NbtTag {
            key: "root".to_string(),
            value: NbtValue::Compound(
                vec![
                    NbtTag::new("byteTag".to_string(), NbtValue::U8(42.into())),
                    NbtTag::new(
                        "stringTag".to_string(),
                        NbtValue::String("Hello, NBT!".into()),
                    ),
                    NbtTag::new(
                        "listTag".to_string(),
                        NbtValue::List(NbtList(vec![
                            NbtValue::I32(1.into()),
                            NbtValue::I32(2.into()),
                            NbtValue::I32(3.into()),
                        ])),
                    ),
                ]
                .into(),
            ),
        };
        let mut serialized_data = original_tag.net_serialize();
        let deserialized_tag = NbtTag::net_deserialize(&mut serialized_data).unwrap();
        assert_eq!(original_tag.key, deserialized_tag.key);
    }
}

