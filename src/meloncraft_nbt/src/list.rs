//! Module for struct [`NbtList`].

use crate::NbtValue;
use core::ops::{Deref, DerefMut};

/// NBT wrapper to represent **a list of a single data type**.
/// Please See [`NbtList::0`] for more information about what an [`NbtList`] stores.
///
/// ## Deref
/// Dereferencing this (and most other [`NbtValue`](`crate::NbtValue`)s) will yield the wrapped type.
/// In this case, a `Vec<NbtValue>` representing all the [`NbtValue`]s in the array.
///
/// ## Static typing
/// **`NbtList`s are not fully dynamic!**
/// All elements should be of the same type, or this struct will not serialize properly.
///
/// ## Serialization
/// To serialize or deserialize an [`NbtList`] to/from a network protocol format, you should use the
/// traits provided by the `meloncraft_protocol_types` crate, which implements `ProtocolType` on the
/// [`NbtValue`](`crate::NbtValue`) and [`NbtTag`](`crate::NbtTag`) types.
#[derive(Debug, Clone, PartialEq)]
pub struct NbtList(
    /// A `Vec` containing the [`NbtValue`]s contained within this list.
    ///
    /// It seems like lists can contain other lists and compounds (and maybe arrays), but I can't
    /// confirm this at the time of writing this documentation.
    ///
    /// ## Constraints
    /// - All values in this vec should be of the same NBT type! Serializers should catch this and
    ///   panic, but please also be sure yourself.
    ///
    /// ## Alternatives
    /// If possible, you should use an array. The three NBT arrays are:
    /// - [`NbtArrayU8`](`crate::NbtArrayU8`) for a `Vec<u8>`
    /// - [`NbtArrayI32`](`crate::NbtArrayI32`) for a `Vec<i32>`
    /// - [`NbtArrayI64`](`crate::NbtArrayI64`) for a `Vec<i64>`
    ///
    /// Check the protocol to find out if your specific use case uses a list or array.
    pub Vec<NbtValue>,
);

impl Deref for NbtList {
    type Target = Vec<NbtValue>;
    fn deref(&self) -> &Self::Target {
        return &self.0;
    }
}

impl DerefMut for NbtList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        return &mut self.0;
    }
}

impl From<Vec<NbtValue>> for NbtList {
    fn from(value: Vec<NbtValue>) -> Self {
        return Self(value);
    }
}
