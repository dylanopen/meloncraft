use std::ops::{Deref, DerefMut};

#[derive(Debug, Clone, PartialEq)]
pub struct NbtArrayU8(pub Vec<u8>);

impl Deref for NbtArrayU8 {
    type Target = Vec<u8>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for NbtArrayU8 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<Vec<u8>> for NbtArrayU8 {
    fn from(value: Vec<u8>) -> Self {
        Self(value)
    }
}
