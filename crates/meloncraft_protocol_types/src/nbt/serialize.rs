use crate::ProtocolType;
use meloncraft_nbt::NbtValue;
use meloncraft_nbt::{NbtCompound, NbtTag};

pub fn tag(tag: NbtTag) -> Vec<u8> {
    let mut output = Vec::new();
    output.push(tag.value.to_id());
    output.extend(string(tag.key)); // Empty name for root
    output.extend(value(tag.value));
    output
}

pub fn value(payload: NbtValue) -> Vec<u8> {
    match payload {
        NbtValue::U8(p) => p.net_serialize(),
        NbtValue::I16(p) => p.net_serialize(),
        NbtValue::I32(p) => p.net_serialize(),
        NbtValue::I64(p) => p.net_serialize(),
        NbtValue::F32(p) => p.net_serialize(),
        NbtValue::F64(p) => p.net_serialize(),
        NbtValue::ArrayU8(p) => byte_array(&p),
        NbtValue::String(p) => string((*p).clone()),
        NbtValue::List(p) => list((*p).clone()),
        NbtValue::Compound(p) => compound((*p).clone()),
        NbtValue::ArrayI32(p) => int_array((*p).clone()),
        NbtValue::ArrayI64(p) => long_array((*p).clone()),
    }
}

pub fn byte_array(payload: &[u8]) -> Vec<u8> {
    let mut output = Vec::new();
    let length: i32 = payload.len().try_into().unwrap();
    output.extend(length.net_serialize());
    output.extend(payload.to_vec());
    output
}

pub fn string(payload: String) -> Vec<u8> {
    let mut output = Vec::new();
    let length: i16 = payload.len().try_into().unwrap();
    output.extend(length.net_serialize());
    output.extend(payload.into_bytes());
    output
}

pub fn list(mut payload: Vec<NbtValue>) -> Vec<u8> {
    let mut output = Vec::new();
    if payload.is_empty() {
        output.push(10); // Default to Compound type for empty list
        output.extend(0i32.net_serialize());
        output.push(0); // Tag ID for end tag
        output.append(&mut 0i32.net_serialize()); // Length 0
        return output;
    }

    // check if all same type
    if !payload.iter().all(|v| v.to_id() == payload[0].to_id()) {
        // make all same type by converting to compound of key "":
        for value in &mut payload {
            if let NbtValue::Compound(_) = value {} else {
                let new_compound = vec![NbtTag {
                    key: String::new(),
                    value: value.clone(),
                }];
                *value = NbtValue::Compound(NbtCompound(new_compound));
            }
        }
    }

    let tag_id: u8 = payload[0].to_id();
    output.extend(tag_id.net_serialize());
    let length: i32 = payload.len().try_into().unwrap();
    output.extend(length.net_serialize());
    for item in payload {
        match item {
            NbtValue::U8(p) => output.append(&mut p.net_serialize()),
            NbtValue::I16(p) => output.append(&mut p.net_serialize()),
            NbtValue::I32(p) => output.append(&mut p.net_serialize()),
            NbtValue::I64(p) => output.append(&mut p.net_serialize()),
            NbtValue::F32(p) => output.append(&mut p.net_serialize()),
            NbtValue::F64(p) => output.append(&mut p.net_serialize()),
            NbtValue::ArrayU8(p) => output.append(&mut byte_array(&p)),
            NbtValue::String(p) => output.append(&mut string((*p).clone())),
            NbtValue::List(p) => output.append(&mut list((*p).clone())),
            NbtValue::Compound(p) => output.append(&mut compound((*p).clone())),
            NbtValue::ArrayI32(p) => output.append(&mut int_array((*p).clone())),
            NbtValue::ArrayI64(p) => output.append(&mut long_array((*p).clone())),
        }
    }
    output
}

pub fn compound(payload: Vec<NbtTag>) -> Vec<u8> {
    let mut output = vec![];
    for tag in payload {
        output.push(tag.value.to_id());
        output.append(&mut string(tag.key.clone()));
        output.append(&mut value(tag.value.clone()));
    }
    output.push(0); // End tag
    output
}

pub fn int_array(payload: Vec<i32>) -> Vec<u8> {
    let mut output = Vec::new();
    let length: i32 = payload.len().try_into().unwrap();
    output.append(&mut length.net_serialize());
    for item in payload {
        output.append(&mut item.net_serialize());
    }
    output
}

pub fn long_array(payload: Vec<i64>) -> Vec<u8> {
    let mut output = Vec::new();
    let length: i32 = payload.len().try_into().unwrap();
    output.append(&mut length.net_serialize());
    for item in payload {
        output.append(&mut item.net_serialize());
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;
    use meloncraft_nbt::NbtTag;
    use meloncraft_nbt::NbtValue;

    #[test]
    fn test_serialize_root() {
        let root_tag = NbtTag {
            key: String::new(),
            value: NbtValue::Compound(
                vec![
                    NbtTag::new("byteTag".to_string(), NbtValue::U8(42.into())),
                    NbtTag::new(
                        "stringTag".to_string(),
                        NbtValue::String("Hello, NBT!".into()),
                    ),
                ]
                .into(),
            ),
        };

        let serialized = tag(root_tag);
        let expected: Vec<u8> = vec![
            10, // Compound/root tag ID
            0, 0, // Length of empty string for root name
            1, // U8 tag ID
            0, 7, // Length of "byteTag"
            b'b', b'y', b't', b'e', b'T', b'a', b'g', // "byteTag"
            42,   // U8 value
            8,    // String tag ID
            0, 9, // Length of "stringTag"
            b's', b't', b'r', b'i', b'n', b'g', b'T', b'a', b'g', // "stringTag"
            0, 11, // Length of "Hello, NBT!"
            b'H', b'e', b'l', b'l', b'o', b',', b' ', b'N', b'B', b'T', b'!', // "Hello, NBT!"
            0,    // End tag
        ];

        assert_eq!(serialized, expected);
    }
}
