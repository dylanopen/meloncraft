use core::ops::{Deref, DerefMut};

/// NBT wrapper to represent a signed 64-bit long integer (`i64`).
///
/// ## Deref
/// Dereferencing this (and most other [`NbtValue`](`crate::NbtValue`)s will yield the wrapped type.
/// In this case, the `i64` representation of the value.
///
/// ## Serialization
/// To serialize or deserialize an [`NbtI64`] to/from a network protocol format, you should use the
/// traits provided by the `meloncraft_protocol_types` crate, which implements `ProtocolType` on the
/// [`NbtValue`](`crate::NbtValue`) and [`NbtTag`](`crate::NbtTag`) types.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct NbtI64(
    /// A `i64` representing the value of this NBT value or tag.
    pub i64,
);

impl Deref for NbtI64 {
    type Target = i64;
    fn deref(&self) -> &Self::Target {
        return &self.0;
    }
}

impl DerefMut for NbtI64 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        return &mut self.0;
    }
}

impl From<i64> for NbtI64 {
    fn from(value: i64) -> Self {
        return Self(value);
    }
}
