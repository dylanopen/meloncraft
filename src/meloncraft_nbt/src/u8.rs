//! Module for struct [`NbtU8`].

use core::ops::{Deref, DerefMut};

/// NBT wrapper to represent an unsigned byte (`u8`).
///
/// ## Deref
/// Dereferencing this (and most other [`NbtValue`](`crate::NbtValue`)s will yield the wrapped type.
/// In this case, the `u8` representation of the value.
///
/// ## Serialization
/// To serialize or deserialize an [`NbtU8`] to/from a network protocol format, you should use the
/// traits provided by the `meloncraft_protocol_types` crate, which implements `ProtocolType` on the
/// [`NbtValue`](`crate::NbtValue`) and [`NbtTag`](`crate::NbtTag`) types.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct NbtU8(
    /// A `u8` representing the value of this NBT value or tag.
    pub u8,
);

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
