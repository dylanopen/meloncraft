use crate::NbtTag;
use std::ops::{Deref, DerefMut};

#[derive(Debug, Clone, PartialEq)]
pub struct NbtCompound(pub Vec<NbtTag>);

impl Deref for NbtCompound {
    type Target = Vec<NbtTag>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for NbtCompound {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<Vec<NbtTag>> for NbtCompound {
    fn from(value: Vec<NbtTag>) -> Self {
        Self(value)
    }
}
