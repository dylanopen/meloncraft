use crate::value::NbtValue;

#[derive(Debug, Clone, PartialEq)]
pub struct NbtTag {
    pub key: String,
    pub value: NbtValue,
}

impl NbtTag {
    #[must_use]
    pub const fn new(key: String, value: NbtValue) -> Self {
        NbtTag { key, value }
    }
}
