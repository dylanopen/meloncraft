use std::ops::{Deref, DerefMut};

#[derive(Debug, Clone, PartialEq)]
pub struct NbtArrayI64(pub Vec<i64>);

impl Deref for NbtArrayI64 {
    type Target = Vec<i64>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for NbtArrayI64 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<Vec<i64>> for NbtArrayI64 {
    fn from(value: Vec<i64>) -> Self {
        Self(value)
    }
}
