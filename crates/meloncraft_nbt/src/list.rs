use crate::NbtValue;
use std::ops::{Deref, DerefMut};

#[derive(Debug, Clone, PartialEq)]
pub struct NbtList(pub Vec<NbtValue>);

impl Deref for NbtList {
    type Target = Vec<NbtValue>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for NbtList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<Vec<NbtValue>> for NbtList {
    fn from(value: Vec<NbtValue>) -> Self {
        Self(value)
    }
}
