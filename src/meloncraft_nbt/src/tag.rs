//! Module for struct [`NbtTag`].

use crate::value::NbtValue;

/// Stores a key-value pair for an NBT tag (in an [`NbtCompound`](`crate::NbtCompound`)).
/// This can act similarly to a hashmap entry.
///
/// It is essentially a compounded type of both a key (`String`) and a value ([`NbtValue`]).
/// You are advised to use this struct if you need to store both the NBT's key *and* value.
///
/// Many functions are provided by the [`NbtValue`] struct, instead of here. If you can't find a
/// function on [`NbtTag`], try finding it in the [`NbtTag::value`] field, which is an [`NbtValue`].
///
/// ## Alternatives
/// - If you just need the value, not the key, use [`NbtValue`].
/// - If in addition, you know the NBT tag tyep of your data, you can use a specific NBT type from
///   the [`meloncraft_nbt`](`crate`) crate (e.g. `NbtU8`).
#[derive(Debug, Clone, PartialEq)]
pub struct NbtTag {

    /// The key that can be used to refer to this tag.
    ///
    /// If this tag is part of a compound, this can be used as an index to this tag's [`NbtValue`],
    /// see the [`NbtTag::value`] field.
    ///
    /// ## Root compound
    /// To represent the *root compound* (the outermost compound used in many different structures
    /// of NBT data) you can just use an empty string (`""`) as the key, since the root compound does not have a key.
    /// Some other implementations may use a different string as the key for the root compound.
    pub key: String,

    /// The value of this [`NbtTag`], which is its actual data.
    /// This can be any of the NBT types, see [`NbtValue`] for more details.
    pub value: NbtValue,
}

impl NbtTag {

    /// Creates a new [`NbtTag`] with the given key and value.
    ///
    /// ## Parameters
    /// - `key`: The key for this tag, which may be used to refer to this tag in its parent compound.
    /// - `value`: The value of this tag, which can be any of the NBT types, see [`NbtValue`] for more details.
    ///
    /// ## Returns
    /// - A new [`NbtTag`] with the given key and value.
    /// - `NbtTag { key, value }`
    #[must_use]
    pub const fn new(key: String, value: NbtValue) -> Self {
        return NbtTag { key, value };
    }
}
