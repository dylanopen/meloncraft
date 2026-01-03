use crate::value::NbtValue;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct NbtTag {
    pub key: String,
    pub value: NbtValue,
}

impl NbtTag {
    pub fn new(key: String, value: NbtValue) -> Self {
        NbtTag { key, value }
    }
}
