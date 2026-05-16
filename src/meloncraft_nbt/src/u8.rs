use core::ops::{Deref, DerefMut};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct NbtU8(pub u8);

impl Deref for NbtU8 {
    type Target = u8;
    fn deref(&self) -> &Self::Target {
        return &self.0;
    }
}

impl DerefMut for NbtU8 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        return &mut self.0;
    }
}

impl From<u8> for NbtU8 {
    fn from(value: u8) -> Self {
        return Self(value);
    }
}
