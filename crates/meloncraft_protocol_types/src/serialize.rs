// Thank you https://git.thetxt.io/thetxt/oxide for the heavy inspiration!
use crate::nbt::NbtTag;

const SEGMENT_BITS: u8 = 0x7F;
const CONTINUE_BIT: u8 = 0x80;

//TODO: doesnt work with negative numbers
pub fn varint(mut value: i32) -> Vec<u8> {
    let mut output: Vec<u8> = Vec::new();

    loop {
        if value as u8 & !SEGMENT_BITS == 0 {
            output.push(value as u8);
            break;
        }

        output.push((value as u8 & SEGMENT_BITS) | CONTINUE_BIT);

        value >>= 7;

        if output.len() >= 5 {
            break;
        }
    }

    return output;
}

pub fn float(input: f32) -> Vec<u8> {
    return input.to_be_bytes().to_vec();
}

pub fn double(input: f64) -> Vec<u8> {
    return input.to_be_bytes().to_vec();
}

pub fn short(input: i16) -> Vec<u8> {
    return input.to_be_bytes().to_vec();
}

pub fn int(input: i32) -> Vec<u8> {
    return input.to_be_bytes().to_vec();
}

pub fn long(input: i64) -> Vec<u8> {
    return input.to_be_bytes().to_vec();
}

pub fn string(input: &str) -> Vec<u8> {
    let mut output: Vec<u8> = varint(input.len() as i32);

    output.append(&mut input.as_bytes().to_vec());

    return output;
}

pub fn uuid(input: &u128) -> Vec<u8> {
    return input.to_be_bytes().to_vec();
}

pub fn bool(input: &bool) -> Vec<u8> {
    return match input {
        true => vec![1],
        false => vec![0],
    }
}

pub fn nbt(input: NbtTag) -> Vec<u8> {
    let mut nbt = nbt_tag_compound(None, vec![input], false);
    nbt.pop(); //Otherwise we have one 0x00 byte too much at the end
    return nbt;
}

fn nbt_byte(description: Option<&str>, payload: u8, include_id: bool) -> Vec<u8> {
    let mut output: Vec<u8> = Vec::new();

    if include_id {
        output.push(0x01);
    }

    if description.is_some() {
        output.append(&mut nbt_string(None, description.unwrap(), false));
    }

    output.push(payload);

    return output;
}

fn nbt_short(description: Option<&str>, payload: i16, include_id: bool) -> Vec<u8> {
    let mut output: Vec<u8> = Vec::new();

    if include_id {
        output.push(0x02);
    }

    if description.is_some() {
        output.append(&mut nbt_string(None, description.unwrap(), false));
    }

    output.append(&mut payload.to_be_bytes().into());

    return output;
}

fn nbt_int(description: Option<&str>, payload: i32, include_id: bool) -> Vec<u8> {
    let mut output: Vec<u8> = Vec::new();

    if include_id {
        output.push(0x03);
    }

    if description.is_some() {
        output.append(&mut nbt_string(None, description.unwrap(), false));
    }

    output.append(&mut payload.to_be_bytes().into());

    return output;
}

fn nbt_long(description: Option<&str>, payload: i64, include_id: bool) -> Vec<u8> {
    let mut output: Vec<u8> = Vec::new();

    if include_id {
        output.push(0x04);
    }

    if description.is_some() {
        output.append(&mut nbt_string(None, description.unwrap(), false));
    }

    output.append(&mut payload.to_be_bytes().into());

    return output;
}

fn nbt_float(description: Option<&str>, payload: f32, include_id: bool) -> Vec<u8> {
    let mut output: Vec<u8> = Vec::new();

    if include_id {
        output.push(0x05);
    }

    if description.is_some() {
        output.append(&mut nbt_string(None, description.unwrap(), false));
    }

    output.append(&mut payload.to_be_bytes().into());

    return output;
}

fn nbt_double(description: Option<&str>, payload: f64, include_id: bool) -> Vec<u8> {
    let mut output: Vec<u8> = Vec::new();

    if include_id {
        output.push(0x06);
    }

    if description.is_some() {
        output.append(&mut nbt_string(None, description.unwrap(), false));
    }

    output.append(&mut payload.to_be_bytes().into());

    return output;
}

fn nbt_byte_array(description: Option<&str>, payload: &[u8], include_id: bool) -> Vec<u8> {
    let mut output: Vec<u8> = Vec::new();

    if include_id {
        output.push(0x07);
    }

    if description.is_some() {
        output.append(&mut nbt_string(None, description.unwrap(), false));
    }

    output.append(&mut (payload.len() as i32).to_be_bytes().to_vec());

    output.append(&mut payload.to_vec());

    return output;
}

fn nbt_string(description: Option<&str>, payload: &str, include_id: bool) -> Vec<u8> {
    let mut output: Vec<u8> = Vec::new();

    if include_id {
        output.push(0x08);
    }

    if description.is_some() {
        output.append(&mut nbt_string(None, description.unwrap(), false));
    }

    output.append(&mut nbt_short(None, payload.len() as i16, false));
    output.append(&mut payload.as_bytes().to_vec());

    return output;
}

fn nbt_list(description: Option<&str>, payload: Vec<NbtTag>, include_id: bool) -> Vec<u8> {
    let mut output: Vec<u8> = Vec::new();

    if include_id {
        output.push(0x09);
    }

    if description.is_some() {
        output.append(&mut nbt_string(None, description.unwrap(), false));
    }

    if payload.len() == 0 {
        return output;
    }

    let length: i32 = payload.len() as i32;

    match payload[0] {
        NbtTag::Byte(_, _) => {
            output.push(0x01);
            output.append(&mut length.to_be_bytes().into());
            payload.into_iter().for_each(|x| output.append(&mut nbt_byte(None, match x {
                NbtTag::Byte(_, x) => x,
                _ => panic!("impossible to reach"),
            }, false)));
        },
        NbtTag::Short(_, _) => {
            output.push(0x02);
            output.append(&mut length.to_be_bytes().into());
            payload.into_iter().for_each(|x| output.append(&mut nbt_short(None, match x {
                NbtTag::Short(_, x) => x,
                _ => panic!("impossible to reach"),
            }, false)));
        },
        NbtTag::Int(_, _) => {
            output.push(0x03);
            output.append(&mut length.to_be_bytes().into());
            payload.into_iter().for_each(|x| output.append(&mut nbt_int(None, match x {
                NbtTag::Int(_, x) => x,
                _ => panic!("impossible to reach"),
            }, false)));
        },
        NbtTag::Long(_, _) => {
            output.push(0x04);
            output.append(&mut length.to_be_bytes().into());
            payload.into_iter().for_each(|x| output.append(&mut nbt_long(None, match x {
                NbtTag::Long(_, x) => x,
                _ => panic!("impossible to reach"),
            }, false)));
        },
        NbtTag::Float(_, _) => {
            output.push(0x05);
            output.append(&mut length.to_be_bytes().into());
            payload.into_iter().for_each(|x| output.append(&mut nbt_float(None, match x {
                NbtTag::Float(_, x) => x,
                _ => panic!("impossible to reach"),
            }, false)));
        },
        NbtTag::Double(_, _) => {
            output.push(0x06);
            output.append(&mut length.to_be_bytes().into());
            payload.into_iter().for_each(|x| output.append(&mut nbt_double(None, match x {
                NbtTag::Double(_, x) => x,
                _ => panic!("impossible to reach"),
            }, false)));
        },
        NbtTag::ByteArray(_, _) => {
            output.push(0x07);
            output.append(&mut length.to_be_bytes().into());
            payload.into_iter().for_each(|x| output.append(&mut nbt_byte_array(None, match x {
                NbtTag::ByteArray(_, x) => x,
                _ => panic!("impossible to reach"),
            }, false)));
        },
        NbtTag::String(_, _) => {
            output.push(0x08);
            output.append(&mut length.to_be_bytes().into());
            payload.into_iter().for_each(|x| output.append(&mut nbt_string(None, match x {
                NbtTag::String(_, x) => x,
                _ => panic!("impossible to reach"),
            }, false)));
        },
        NbtTag::List(_, _) => {
            output.push(0x09);
            output.append(&mut length.to_be_bytes().into());
            payload.into_iter().for_each(|x| output.append(&mut nbt_list(None, match x {
                NbtTag::List(_, x) => x,
                _ => panic!("impossible to reach"),
            }, false)));
        },
        NbtTag::TagCompound(_, _) => {
            output.push(0x0a);
            output.append(&mut length.to_be_bytes().into());
            payload.into_iter().for_each(|x| output.append(&mut nbt_tag_compound(None, match x {
                NbtTag::TagCompound(_, x) => x,
                _ => panic!("impossible to reach"),
            }, false)));
        },
        NbtTag::IntArray(_, _) => {
            output.push(0x0b);
            output.append(&mut length.to_be_bytes().into());
            payload.into_iter().for_each(|x| output.append(&mut nbt_int_array(None, match x {
                NbtTag::IntArray(_, x) => x,
                _ => panic!("impossible to reach"),
            }, false)));
        },
        NbtTag::LongArray(_, _) => {
            output.push(0x0c);
            output.append(&mut length.to_be_bytes().into());
            payload.into_iter().for_each(|x| output.append(&mut nbt_long_array(None, match x {
                NbtTag::LongArray(_, x) => x,
                _ => panic!("impossible to reach"),
            }, false)));
        },
    };

    return output;
}

fn nbt_tag_compound(description: Option<&str>, payload: Vec<NbtTag>, include_id: bool) -> Vec<u8> {
    let mut output: Vec<u8> = Vec::new();

    if include_id {
        output.push(0x0a);
    }

    if description.is_some() {
        output.append(&mut nbt_string(None, description.unwrap(), false));
    }

    if payload.len() == 0 {
        return output;
    }

    payload.into_iter().for_each(|x| {
        match x {
            NbtTag::Byte(d, p) => output.append(&mut nbt_byte(d, p, true)),
            NbtTag::Short(d, p) => output.append(&mut nbt_short(d, p, true)),
            NbtTag::Int(d, p) => output.append(&mut nbt_int(d, p, true)),
            NbtTag::Long(d, p) => output.append(&mut nbt_long(d, p, true)),
            NbtTag::Float(d, p) => output.append(&mut nbt_float(d, p, true)),
            NbtTag::Double(d, p) => output.append(&mut nbt_double(d, p, true)),
            NbtTag::ByteArray(d, p) => output.append(&mut nbt_byte_array(d, p, true)),
            NbtTag::String(d, p) => output.append(&mut nbt_string(d, p, true)),
            NbtTag::List(d, p) => output.append(&mut nbt_list(d, p, true)),
            NbtTag::TagCompound(d, p) => output.append(&mut nbt_tag_compound(d, p, true)),
            NbtTag::IntArray(d, p) => output.append(&mut nbt_int_array(d, p, true)),
            NbtTag::LongArray(d, p) => output.append(&mut nbt_long_array(d, p, true)),
        };
    });

    output.push(0x00);
    return output;
}

fn nbt_int_array(description: Option<&str>, payload: &[i32], include_id: bool) -> Vec<u8> {
    let mut output: Vec<u8> = Vec::new();

    if include_id {
        output.push(0x0b);
    }

    if description.is_some() {
        output.append(&mut nbt_string(None, description.unwrap(), false));
    }

    let length: i32 = payload.len() as i32;
    output.append(&mut length.to_be_bytes().into());

    payload.into_iter().for_each(|x| output.append(&mut nbt_int(None, *x, false)));

    return output;
}

fn nbt_long_array(description: Option<&str>, payload: &[i64], include_id: bool) -> Vec<u8> {
    let mut output: Vec<u8> = Vec::new();

    if include_id {
        output.push(0x0c);
    }

    if description.is_some() {
        output.append(&mut nbt_string(None, description.unwrap(), false));
    }

    let length: i32 = payload.len() as i32;
    output.append(&mut length.to_be_bytes().into());

    payload.into_iter().for_each(|x| output.append(&mut nbt_long(None, *x, false)));

    return output;
}
