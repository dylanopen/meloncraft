use core::ops::{Deref, DerefMut};

/// NBT wrapper to represent a signed 16-bit short integer (`i16`).
/// 
/// ## Deref
/// Dereferencing this (and most other [`NbtValue`](`crate::NbtValue`)s will yield the wrapped type.
/// In this case, the `i16` representation of the value.
///
/// ## Serialization
/// To serialize or deserialize an [`NbtI16`] to/from a network protocol format, you should use the
/// traits provided by the `meloncraft_protocol_types` crate, which implements `ProtocolType` on the
/// [`NbtValue`](`crate::NbtValue`) and [`NbtTag`](`crate::NbtTag`) types.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct NbtI16(

    /// An `i16` representing the value of this NBT value or tag.
    pub i16
);

impl Deref for NbtI16 {
    type Target = i16;
    fn deref(&self) -> &Self::Target {
        return &self.0;
    }
}

impl DerefMut for NbtI16 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        return &mut self.0;
    }
}

impl From<i16> for NbtI16 {
    fn from(value: i16) -> Self {
        return Self(value);
    }
}
