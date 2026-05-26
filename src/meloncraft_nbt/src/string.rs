//! Module for struct [`NbtString`].

use core::ops::{Deref, DerefMut};

/// NBT wrapper to represent a **UTF-8-encoded string**.
///
/// ## Deref
/// Dereferencing this (and most other [`NbtValue`](`crate::NbtValue`)s) will yield the wrapped type.
/// In this case, a `String` representing the *value* of the NBT value / tag.
///
/// ## Constraints
/// The length of the string must be less than or equal to `65_535` bytes in length.
///
/// ## Serialization
/// To serialize or deserialize an [`NbtString`] to/from a network protocol format, you should use the
/// traits provided by the `meloncraft_protocol_types` crate, which implements `ProtocolType` on the
/// [`NbtValue`](`crate::NbtValue`) and [`NbtTag`](`crate::NbtTag`) types.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct NbtString(
    /// A `String` representing the *value* of the NBT value / tag.
    pub String,
);

impl Deref for NbtString {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        return &self.0;
    }
}

impl DerefMut for NbtString {
    fn deref_mut(&mut self) -> &mut Self::Target {
        return &mut self.0;
    }
}

impl From<String> for NbtString {
    fn from(value: String) -> Self {
        return Self(value);
    }
}

impl From<&str> for NbtString {
    fn from(value: &str) -> Self {
        return value.to_owned().into();
    }
}
