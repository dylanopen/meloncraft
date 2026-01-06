use meloncraft_nbt::{NbtCompound, NbtList, NbtTag, NbtU8, NbtValue};

use crate::ProtocolBuffer;

pub fn tag(data: &mut Vec<u8>) -> Result<NbtTag, ()> {
    let tag_type = tagtype(data)?;
    let key = string(data)?;
    let value = value(tag_type, data)?;

    Ok(meloncraft_nbt::NbtTag::new(key, value))
}

fn tagtype(data: &mut Vec<u8>) -> Result<u8, ()> {
    if data.is_empty() {
        return Err(());
    }
    data.net_deserialize()
}

fn string(data: &mut Vec<u8>) -> Result<String, ()> {
    let length: i16 = data.net_deserialize()?;
    if data.len() < length as usize {
        return Err(());
    }
    let string_bytes = data.drain(0..length as usize).collect::<Vec<u8>>();
    match String::from_utf8(string_bytes) {
        Ok(string) => Ok(string),
        Err(_) => Err(()),
    }
}

pub fn value(tag_type: u8, data: &mut Vec<u8>) -> Result<NbtValue, ()> {
    match tag_type {
        1 => {
            let val: u8 = data.net_deserialize()?;
            Ok(NbtValue::U8(NbtU8(val)))
        }
        2 => {
            let val: i16 = data.net_deserialize()?;
            Ok(NbtValue::I16(meloncraft_nbt::NbtI16(val)))
        }
        3 => {
            let val: i32 = data.net_deserialize()?;
            Ok(NbtValue::I32(meloncraft_nbt::NbtI32(val)))
        }
        4 => {
            let val: i64 = data.net_deserialize()?;
            Ok(NbtValue::I64(meloncraft_nbt::NbtI64(val)))
        }
        5 => {
            let val: f32 = data.net_deserialize()?;
            Ok(NbtValue::F32(meloncraft_nbt::NbtF32(val)))
        }
        6 => {
            let val: f64 = data.net_deserialize()?;
            Ok(NbtValue::F64(meloncraft_nbt::NbtF64(val)))
        }
        7 => {
            let length: i32 = data.net_deserialize()?;
            byte_array(length, data)
        }
        8 => {
            let string = string(data)?;
            Ok(NbtValue::String(meloncraft_nbt::NbtString(string)))
        }
        9 => {
            let list_tag_type = tagtype(data)?;
            let size: i32 = data.net_deserialize()?;
            list(list_tag_type, size, data)
        }
        10 => compound(data),
        11 => {
            let length: i32 = data.net_deserialize()?;
            int_array(length, data)
        }
        12 => {
            let length: i32 = data.net_deserialize()?;
            long_array(length, data)
        }
        _ => Err(()),
    }
}

fn byte_array(length: i32, data: &mut Vec<u8>) -> Result<NbtValue, ()> {
    if data.len() < length as usize {
        return Err(());
    }
    let mut bytes = Vec::with_capacity(length as usize);
    for _ in 0..length {
        let byte: u8 = data.net_deserialize()?;
        bytes.push(byte);
    }
    Ok(NbtValue::ArrayU8(meloncraft_nbt::NbtArrayU8(bytes)))
}

fn int_array(length: i32, data: &mut Vec<u8>) -> Result<NbtValue, ()> {
    if data.len() < (length as usize) * 4 {
        return Err(());
    }
    let mut ints = Vec::with_capacity(length as usize);
    for _ in 0..length {
        let int: i32 = data.net_deserialize()?;
        ints.push(int);
    }
    Ok(NbtValue::ArrayI32(meloncraft_nbt::NbtArrayI32(ints)))
}

fn long_array(length: i32, data: &mut Vec<u8>) -> Result<NbtValue, ()> {
    if data.len() < (length as usize) * 8 {
        return Err(());
    }
    let mut longs = Vec::with_capacity(length as usize);
    for _ in 0..length {
        let long: i64 = data.net_deserialize()?;
        longs.push(long);
    }
    Ok(NbtValue::ArrayI64(meloncraft_nbt::NbtArrayI64(longs)))
}

fn list(tag_type: u8, size: i32, data: &mut Vec<u8>) -> Result<NbtValue, ()> {
    let finish_len = data.len() - size as usize;
    let mut list_items = Vec::new();
    while data.len() > finish_len {
        let item = value(tag_type, data)?;
        list_items.push(item);
    }
    Ok(NbtValue::List(NbtList(list_items)))
}

fn compound(data: &mut Vec<u8>) -> Result<NbtValue, ()> {
    let mut tags = Vec::new();
    loop {
        let tag_type = tagtype(data)?;
        if tag_type == 0 {
            break;
        }
        let key = string(data)?;
        let value = value(tag_type, data)?;
        tags.push(NbtTag::new(key, value));
    }
    dbg!(&tags);
    Ok(NbtValue::Compound(NbtCompound(tags)))
}
