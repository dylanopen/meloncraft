//! Module for struct [`NbtF32`].

use core::ops::{Deref, DerefMut};

/// NBT wrapper to represent a 32-bit floating-point fractional number (`f32`).
///
/// This is used when we need to send numbers with decimal (fractional) parts *but don't need as
/// much precision as an `f64`, or we use [`NbtF64`](`crate::NbtF64`).
///
/// ## Deref
/// Dereferencing this (and most other [`NbtValue`](`crate::NbtValue`)s) will yield the wrapped type.
/// In this case, the `f32` representation of the value.
///
/// ## Serialization
/// To serialize or deserialize an [`NbtF32`] to/from a network protocol format, you should use the
/// traits provided by the `meloncraft_protocol_types` crate, which implements `ProtocolType` on the
/// [`NbtValue`](`crate::NbtValue`) and [`NbtTag`](`crate::NbtTag`) types.
#[derive(Debug, Clone, PartialEq)]
pub struct NbtF32(
    /// An `f32` representing the floating-point value of this NBT value or tag.
    pub f32,
);

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
