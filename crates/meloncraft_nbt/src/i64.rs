use std::ops::{Deref, DerefMut};

#[derive(Debug, Clone, PartialEq)]
pub struct NbtI64(pub i64);

impl Deref for NbtI64 {
    type Target = i64;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for NbtI64 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<i64> for NbtI64 {
    fn from(value: i64) -> Self {
        Self(value)
    }
}
