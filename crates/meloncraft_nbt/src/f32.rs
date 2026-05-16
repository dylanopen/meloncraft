use core::ops::{Deref, DerefMut};

#[derive(Debug, Clone, PartialEq)]
pub struct NbtF32(pub f32);

impl Deref for NbtF32 {
    type Target = f32;
    fn deref(&self) -> &Self::Target {
        return &self.0;
    }
}

impl DerefMut for NbtF32 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        return &mut self.0;
    }
}

impl From<f32> for NbtF32 {
    fn from(value: f32) -> Self {
        return Self(value);
    }
}
