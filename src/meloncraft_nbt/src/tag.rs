//! Module for struct [`NbtTag`].

use crate::value::NbtValue;

/// Stores a key-value pair for an NBT tag (in an [`NbtCompound`](`crate::NbtCompound`)).
/// This can act similarly to a hashmap entry.
///
/// ## Alternatives
/// - If you just need the value, not the key, use [`NbtValue`].
/// - If in addition, you know the NBT tag tyep of your data, you can use a specific NBT type from
///   the [`meloncraft_nbt`](`crate`) crate (e.g. `NbtU8`).
#[derive(Debug, Clone, PartialEq)]
pub struct NbtTag {

    pub key: String,

    pub value: NbtValue,
}

impl NbtTag {
    #[must_use]
    pub const fn new(key: String, value: NbtValue) -> Self {
        return NbtTag { key, value };
    }
}
