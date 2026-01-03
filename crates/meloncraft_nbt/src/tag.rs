use crate::value::NbtValue;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct NbtTag {
    pub key: String,
    pub value: NbtValue,
}
