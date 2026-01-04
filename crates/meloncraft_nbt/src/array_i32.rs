use std::ops::{Deref, DerefMut};

#[derive(Debug, Clone, PartialEq)]
pub struct NbtArrayI32(pub Vec<i32>);

impl Deref for NbtArrayI32 {
    type Target = Vec<i32>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for NbtArrayI32 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<Vec<i32>> for NbtArrayI32 {
    fn from(value: Vec<i32>) -> Self {
        Self(value)
    }
}
