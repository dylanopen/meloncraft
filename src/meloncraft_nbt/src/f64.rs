use core::ops::{Deref, DerefMut};

#[derive(Debug, Clone, PartialEq)]
pub struct NbtF64(pub f64);

impl Deref for NbtF64 {
    type Target = f64;
    fn deref(&self) -> &Self::Target {
        return &self.0;
    }
}

impl DerefMut for NbtF64 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        return &mut self.0;
    }
}

impl From<f64> for NbtF64 {
    fn from(value: f64) -> Self {
        return Self(value);
    }
}
