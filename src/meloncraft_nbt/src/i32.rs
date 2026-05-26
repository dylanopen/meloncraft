//! Module for struct [`NbtI32`].

use core::ops::{Deref, DerefMut};

/// NBT wrapper to represent a signed 32-bit integer (`i32`).
///
/// ## Deref
/// Dereferencing this (and most other [`NbtValue`](`crate::NbtValue`)s will yield the wrapped type.
/// In this case, the `i32` representation of the value.
///
/// ## Serialization
/// To serialize or deserialize an [`NbtI32`] to/from a network protocol format, you should use the
/// traits provided by the `meloncraft_protocol_types` crate, which implements `ProtocolType` on the
/// [`NbtValue`](`crate::NbtValue`) and [`NbtTag`](`crate::NbtTag`) types.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct NbtI32(
    /// An `i32` representing the value of this NBT value or tag.
    pub i32,
);

impl Deref for NbtI32 {
    type Target = i32;
    fn deref(&self) -> &Self::Target {
        return &self.0;
    }
}

impl DerefMut for NbtI32 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        return &mut self.0;
    }
}

impl From<i32> for NbtI32 {
    fn from(value: i32) -> Self {
        return Self(value);
    }
}
