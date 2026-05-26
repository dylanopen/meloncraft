//! Module for struct [`NbtArrayI64`].

use core::ops::{Deref, DerefMut};

/// NBT wrapper to represent **a list of** 64-bit signed integers (a `Vec` of `i64`).
///
/// These are a special type of list in the NBT format; this struct acts as a marker that the
/// contained value should be treated as an NBT array of i64s.
///
/// ## Deref
/// Dereferencing this (and most other [`NbtValue`](`crate::NbtValue`)s) will yield the wrapped type.
/// In this case, a `Vec<i64>` representing all the bytes in the array.
///
/// ## Serialization
/// To serialize or deserialize an [`NbtArrayI64`] to/from a network protocol format, you should use the
/// traits provided by the `meloncraft_protocol_types` crate, which implements `ProtocolType` on the
/// [`NbtValue`](`crate::NbtValue`) and [`NbtTag`](`crate::NbtTag`) types.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct NbtArrayI64(
    /// A `Vec` containing a variable number of `i64`s. This is the *value* of the NBT value / tag.
    pub Vec<i64>,
);

impl Deref for NbtArrayI64 {
    type Target = Vec<i64>;
    fn deref(&self) -> &Self::Target {
        return &self.0;
    }
}

impl DerefMut for NbtArrayI64 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        return &mut self.0;
    }
}

impl From<Vec<i64>> for NbtArrayI64 {
    fn from(value: Vec<i64>) -> Self {
        return Self(value);
    }
}
