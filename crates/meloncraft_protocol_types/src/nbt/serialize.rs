use crate::ProtocolType;
use meloncraft_nbt::NbtTag;
use meloncraft_nbt::NbtValue;

pub fn root(tag: NbtTag) -> Vec<u8> {
    let mut output = Vec::new();
    let root_compound = tag.value;
    if let NbtValue::Root(compound_tag) = root_compound {
        output.push(10); // Tag ID for Compound/Root
        output.append(&mut string(tag.key)); // Empty name for root
        output.append(&mut tag_compound(compound_tag));
    } else {
        panic!("Root value must be of type Root");
    }
    output
}

fn value(payload: NbtValue) -> Vec<u8> {
    match payload {
        NbtValue::U8(p) => p.net_serialize(),
        NbtValue::I16(p) => p.net_serialize(),
        NbtValue::I32(p) => p.net_serialize(),
        NbtValue::I64(p) => p.net_serialize(),
        NbtValue::F32(p) => p.net_serialize(),
        NbtValue::F64(p) => p.net_serialize(),
        NbtValue::ArrayU8(p) => byte_array(p),
        NbtValue::String(p) => string(p),
        NbtValue::List(p) => list(p),
        NbtValue::Compound(p) => tag_compound(p),
        NbtValue::Root(_) => panic!("Root tag cannot be nested"),
        NbtValue::ArrayI32(p) => int_array(p),
        NbtValue::ArrayI64(p) => long_array(p),
    }
}

pub fn byte_array(payload: Vec<u8>) -> Vec<u8> {
    let mut output = Vec::new();
    let length = payload.len() as i32;
    output.append(&mut length.net_serialize());
    output.append(&mut payload.to_vec());
    output
}

pub fn string(payload: String) -> Vec<u8> {
    let mut output = Vec::new();
    let length = payload.len() as i16;
    output.append(&mut length.net_serialize());
    output.append(&mut payload.into_bytes());
    output
}

pub fn list(payload: Vec<NbtValue>) -> Vec<u8> {
    let mut output = Vec::new();
    if payload.is_empty() {
        output.push(0); // Tag ID for end tag
        output.append(&mut 0i32.net_serialize()); // Length 0
        return output;
    }
    let tag_id = payload[0].to_id();
    output.push(tag_id);
    let length = payload.len() as i32;
    output.append(&mut length.net_serialize());
    for item in payload {
        match item {
            NbtValue::U8(p) => output.append(&mut p.net_serialize()),
            NbtValue::I16(p) => output.append(&mut p.net_serialize()),
            NbtValue::I32(p) => output.append(&mut p.net_serialize()),
            NbtValue::I64(p) => output.append(&mut p.net_serialize()),
            NbtValue::F32(p) => output.append(&mut p.net_serialize()),
            NbtValue::F64(p) => output.append(&mut p.net_serialize()),
            NbtValue::ArrayU8(p) => output.append(&mut byte_array(p)),
            NbtValue::String(p) => output.append(&mut string(p)),
            NbtValue::List(p) => output.append(&mut list(p)),
            NbtValue::Compound(p) => output.append(&mut tag_compound(p)),
            NbtValue::Root(_) => panic!("Root tag cannot be in a list"),
            NbtValue::ArrayI32(p) => output.append(&mut int_array(p)),
            NbtValue::ArrayI64(p) => output.append(&mut long_array(p)),
        }
    }
    output
}

pub fn tag_compound(payload: Vec<NbtTag>) -> Vec<u8> {
    let mut output = Vec::new();
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
    let length = payload.len() as i32;
    output.append(&mut length.net_serialize());
    for item in payload {
        output.append(&mut item.net_serialize());
    }
    output
}

pub fn long_array(payload: Vec<i64>) -> Vec<u8> {
    let mut output = Vec::new();
    let length = payload.len() as i32;
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
            value: NbtValue::Root(vec![
                NbtTag::new("byteTag".to_string(), NbtValue::U8(42)),
                NbtTag::new(
                    "stringTag".to_string(),
                    NbtValue::String("Hello, NBT!".to_string()),
                ),
            ]),
        };

        let serialized = root(root_tag);
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
