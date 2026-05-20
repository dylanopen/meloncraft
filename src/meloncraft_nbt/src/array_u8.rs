//! Module for struct [`NbtArrayU8`].

use core::ops::{Deref, DerefMut};

/// NBT wrapper to represent **a list of** 8-bit unsigned bytes (a `Vec` of `u8`).
///
/// These are a special type of list in the NBT format; this struct acts as a marker that the
/// contained value should be treated as an NBT array of u8s.
/// 
/// ## Deref
/// Dereferencing this (and most other [`NbtValue`](`crate::NbtValue`)s) will yield the wrapped type.
/// In this case, a `Vec<u8>` representing all the bytes in the array.
///
/// ## Serialization
/// To serialize or deserialize an [`NbtArrayU8`] to/from a network protocol format, you should use the
/// traits provided by the `meloncraft_protocol_types` crate, which implements `ProtocolType` on the
/// [`NbtValue`](`crate::NbtValue`) and [`NbtTag`](`crate::NbtTag`) types.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct NbtArrayU8(

    /// A `Vec` containing a variable number of `u8`s. This is the *value* of the NBT value / tag.
    pub Vec<u8>
);

impl Deref for NbtArrayU8 {
    type Target = Vec<u8>;
    fn deref(&self) -> &Self::Target {
        return &self.0;
    }
}

impl DerefMut for NbtArrayU8 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        return &mut self.0;
    }
}
impl From<Vec<u8>> for NbtArrayU8 {
    fn from(value: Vec<u8>) -> Self {
        return Self(value);
    }
}
