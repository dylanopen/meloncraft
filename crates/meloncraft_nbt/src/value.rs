use crate::tag::NbtTag;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum NbtValue {
    Root(Vec<NbtTag>),
    U8(u8),
    I16(i16),
    I32(i32),
    I64(i64),
    F32(f32),
    F64(f64),
    ArrayU8(Vec<u8>),
    ArrayI32(Vec<i32>),
    ArrayI64(Vec<i64>),
    String(String),
    List(Vec<NbtValue>),
    Compound(Vec<NbtTag>),
}

impl NbtValue {
    pub fn get_compound_children(&self) -> Option<&Vec<NbtTag>> {
        if let NbtValue::Compound(compound) = self {
            Some(compound)
        } else {
            None
        }
    }

    pub fn get_compound_children_mut(&mut self) -> Option<&mut Vec<NbtTag>> {
        if let NbtValue::Compound(compound) = self {
            Some(compound)
        } else {
            None
        }
    }

    pub fn get_list_children(&self) -> Option<&Vec<NbtValue>> {
        if let NbtValue::List(list) = self {
            Some(list)
        } else {
            None
        }
    }

    pub fn get_list_children_mut(&mut self) -> Option<&mut Vec<NbtValue>> {
        if let NbtValue::List(list) = self {
            Some(list)
        } else {
            None
        }
    }

    pub fn get(&self, key: &str) -> Option<&NbtValue> {
        let compound = match self {
            NbtValue::Compound(compound) => compound,
            NbtValue::Root(compound) => compound,
            _ => return None,
        };
        for tag in compound {
            if tag.key == key {
                return Some(&tag.value);
            }
        }
        None
    }

    pub fn get_mut(&mut self, key: &str) -> Option<&mut NbtValue> {
        let compound = match self {
            NbtValue::Compound(compound) => compound,
            NbtValue::Root(compound) => compound,
            _ => return None,
        };
        for tag in compound {
            if tag.key == key {
                return Some(&mut tag.value);
            }
        }
        None
    }

    pub fn to_id(&self) -> u8 {
        match self {
            NbtValue::U8(_) => 1,
            NbtValue::I16(_) => 2,
            NbtValue::I32(_) => 3,
            NbtValue::I64(_) => 4,
            NbtValue::F32(_) => 5,
            NbtValue::F64(_) => 6,
            NbtValue::ArrayU8(_) => 7,
            NbtValue::String(_) => 8,
            NbtValue::List(_) => 9,
            NbtValue::Compound(_) => 10,
            NbtValue::Root(_) => 10, // internally, a compound
            NbtValue::ArrayI32(_) => 11,
            NbtValue::ArrayI64(_) => 12,
        }
    }
}
