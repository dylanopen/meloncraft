use std::ops::{Deref, DerefMut};

#[derive(Debug, Clone, PartialEq)]
pub struct NbtI16(pub i16);

impl Deref for NbtI16 {
    type Target = i16;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for NbtI16 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<i16> for NbtI16 {
    fn from(value: i16) -> Self {
        Self(value)
    }
}
