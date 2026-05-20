use core::ops::{Deref, DerefMut};

/// NBT wrapper to represent a 64-bit floating-point fractional number (`f64`).
///
/// This is used when we need to send numbers with decimal (fractional) parts *and need more
/// precision than an `f32`. If not, we use [`NbtF32`](`crate::NbtF32`).
/// 
/// ## Deref
/// Dereferencing this (and most other [`NbtValue`](`crate::NbtValue`)s) will yield the wrapped type.
/// In this case, the `f64` representation of the value.
///
/// ## Serialization
/// To serialize or deserialize an [`NbtF64`] to/from a network protocol format, you should use the
/// traits provided by the `meloncraft_protocol_types` crate, which implements `ProtocolType` on the
/// [`NbtValue`](`crate::NbtValue`) and [`NbtTag`](`crate::NbtTag`) types.
#[derive(Debug, Clone, PartialEq)]
pub struct NbtF64(

    /// An `f64` representing the floating-point value of this NBT value or tag.
    pub f64
);

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
