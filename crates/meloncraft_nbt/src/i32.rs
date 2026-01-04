use std::ops::{Deref, DerefMut};

#[derive(Debug, Clone, PartialEq)]
pub struct NbtI32(pub i32);

impl Deref for NbtI32 {
    type Target = i32;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for NbtI32 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<i32> for NbtI32 {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
