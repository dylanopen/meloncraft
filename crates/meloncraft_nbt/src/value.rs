use crate::tag::NbtTag;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum NbtValue {
    Root,
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
}
