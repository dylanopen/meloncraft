use crate::tag::NbtTag;
use crate::{
    NbtArrayI32, NbtArrayI64, NbtArrayU8, NbtCompound, NbtF32, NbtF64, NbtI16, NbtI32, NbtI64,
    NbtList, NbtString, NbtU8,
};

#[derive(Debug, Clone, PartialEq)]
pub enum NbtValue {
    U8(NbtU8),
    I16(NbtI16),
    I32(NbtI32),
    I64(NbtI64),
    F32(NbtF32),
    F64(NbtF64),
    ArrayU8(NbtArrayU8),
    ArrayI32(NbtArrayI32),
    ArrayI64(NbtArrayI64),
    String(NbtString),
    List(NbtList),
    Compound(NbtCompound),
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
        let NbtValue::Compound(compound) = self else {
            return None;
        };
        for tag in compound.iter() {
            if tag.key == key {
                return Some(&tag.value);
            }
        }
        None
    }

    pub fn get_mut(&mut self, key: &str) -> Option<&mut NbtValue> {
        let NbtValue::Compound(compound) = self else {
            return None;
        };
        for tag in compound.iter_mut() {
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
            NbtValue::ArrayI32(_) => 11,
            NbtValue::ArrayI64(_) => 12,
        }
    }
}
