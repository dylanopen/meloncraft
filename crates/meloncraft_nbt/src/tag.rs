#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct NbtTag {
    pub key: String,
    pub value: NbtValue,
}

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
