//! Module for struct [`NbtCompound`].

use crate::{NbtTag, NbtValue};
use core::ops::{Deref, DerefMut};

/// NBT type to represent **a list of [`NbtTag`]s**, accessed like a `HashMap`.
///
/// ## Comparison with `NbtList`
/// An [`NbtTag`] differs from an [`NbtList`](`crate::NbtList`) in a few main ways:
/// - An `NbtList` can only store data of one [`NbtTag`] type; an `NbtCompound` can store multiple fields
///   of different types (they are more dynamic).
/// - An `NbtList` stores a `Vec` of [`NbtValue`]s. They have no keys, are accessed by index, and
///   are like a `Vec` type.
/// - An `NbtCompound` stores a `Vec` of [`NbtTag`]s. This means that each NBT element has an
///   [`NbtTag::key`] which it is referenced by - a bit like a `HashMap`. Indeed, this struct
///   provides methods for acting on the [`NbtTag`]s in the compoundd similarly to the `HashMap`'s
///   interface.
/// 
/// ## Deref
/// Dereferencing this (and most other [`NbtValue`](`crate::NbtValue`)s) will yield the wrapped type.
/// In this case, a `Vec<NbtTag>` representing all the [`NbtTag`] key-[`value`](`NbtValue`) pairs in the array.
///
/// ## Serialization
/// To serialize or deserialize an [`NbtCompound`] to/from a network protocol format, you should use the
/// traits provided by the `meloncraft_protocol_types` crate, which implements `ProtocolType` on the
/// [`NbtValue`](`crate::NbtValue`) and [`NbtTag`](`crate::NbtTag`) types.
///
/// ## Further reading
/// - See this struct's methods for info on what you can do with a compound.
/// - Wiki page on NBT: <https://minecraft.wiki/w/NBT_format>.
#[derive(Debug, Clone, PartialEq)]
pub struct NbtCompound(

    /// A list of [`NbtTag`]s.
    /// You shouldn't access this field manually. Instead, consider the methods on [`NbtCompound`]
    /// for getting and setting values *by their [`NbtTag`] key* instead.
    ///
    /// As each tag has a key and a value, this is effectively used internally as a `HashMap`.
    /// As most NBT structures should be small, there shouldn't be any noticable performance difference
    /// between using a `Vec` and `HashMap` for this purpose.
    pub Vec<NbtTag>,
);

impl Deref for NbtCompound {
    type Target = Vec<NbtTag>;
    fn deref(&self) -> &Self::Target {
        return &self.0;
    }
}

impl DerefMut for NbtCompound {
    fn deref_mut(&mut self) -> &mut Self::Target {
        return &mut self.0;
    }
}

impl From<Vec<NbtTag>> for NbtCompound {
    fn from(value: Vec<NbtTag>) -> Self {
        return Self(value);
    }
}

impl NbtCompound {
    
    /// Get a reference to a [`NbtTag`], querying by the [`NbtTag::key`] (a string key).
    ///
    /// ## Parameters
    /// - `key`: an `&str` representing the key (or key path) of the [`NbtTag`] to return from the compound.
    /// 
    /// ## Returns
    /// - `Some(&NbtTag)` with the full tag corresponding to the provided `key`, if that tag exists
    ///   in the compound.
    /// - `None` if the compound does not contain the key (path).
    ///
    /// ## Key paths
    /// - If your provided `key` contains a slash (`/`), then this function will do a *nested*
    ///   search of all sub-compounds.
    /// - For example, a key of `biome/plains` will search for a child compound (of this compound)
    ///   with the key `biome`. If it finds it (else returns null), it will then search *that*
    ///   compound for an [`NbtTag`] with a key of `plains`, and **return that [`NbtTag`]**.
    #[must_use]
    pub fn get(&self, key: &str) -> Option<&NbtTag> {
        if !key.contains('/') {
            return self.0.iter().find(|tag| return tag.key == key);
        }
        let mut tree: Vec<&str> = key.split('/').collect();
        let mut current_compound = self;
        while !tree.is_empty() {
            if tree.len() == 1 {
                return current_compound.get(tree.first()?);
            }
            let NbtValue::Compound(compound) = current_compound.get_value(tree.remove(0))? else {
                return None;
            };
            current_compound = compound;
        };
        return None;
    }

    /// Same as [`NbtCompound::get`], but returns an immutable reference to just the [`value`](`NbtTag::value`) field of the
    /// [`NbtTag`] (which is of type [`NbtValue`]).
    /// Use this if you don't need the actual tag and just want the value it stores (you usually
    /// don't need the tag, unless sending over the network, because you already know the key from
    /// the `key` parameter passed here).
    ///
    /// ## Parameters
    /// - `key`: an `&str` representing the key (or key path) of the [`NbtTag`]'s [`NbtValue`] to return from the compound.
    /// 
    /// ## Returns
    /// - `Some(&NbtValue)` with just the tag's [`NbtValue`] corresponding to the provided `key`, if that tag exists
    ///   in the compound.
    /// - `None` if the compound does not contain the key (path).
    ///
    /// ## Key paths
    /// - If your provided `key` contains a slash (`/`), then this function will do a *nested*
    ///   search of all sub-compounds.
    /// - For example, a key of `biome/plains` will search for a child compound (of this compound)
    ///   with the key `biome`. If it finds it (else returns null), it will then search *that*
    ///   compound for an [`NbtTag`] with a key of `plains`, and **return that tag's [`NbtValue`]**.
    #[must_use]
    pub fn get_value(&self, key: &str) -> Option<&NbtValue> {
        return self.get(key)
            .map(|tag| return &tag.value);
    }
    
    /// Get a ***mutable*** reference to a [`NbtTag`], querying by the [`NbtTag::key`] (a string key).
    /// > Mutable version of [`NbtCompound::get`].
    ///
    /// ## Parameters
    /// - `key`: an `&str` representing the key (or key path) of the [`NbtTag`] to return from the compound.
    /// 
    /// ## Returns
    /// - `Some(&mut NbtTag)` with the full tag corresponding to the provided `key`, if that tag exists
    ///   in the compound.
    /// - `None` if the compound does not contain the key (path).
    ///
    /// ## Key paths
    /// - If your provided `key` contains a slash (`/`), then this function will do a *nested*
    ///   search of all sub-compounds.
    /// - For example, a key of `biome/plains` will search for a child compound (of this compound)
    ///   with the key `biome`. If it finds it (else returns null), it will then search *that*
    ///   compound for an [`NbtTag`] with a key of `plains`, and **return that [`NbtTag`]**.
    pub fn get_mut(&mut self, key: &str) -> Option<&mut NbtTag> {
        if !key.contains('/') {
            return self.0.iter_mut().find(|tag| return tag.key == key);
        }
        let mut tree: Vec<&str> = key.split('/').collect();
        let mut current_compound = self;
        while !tree.is_empty() {
            if tree.len() == 1 {
                return current_compound.get_mut(tree.first()?);
            }
            let NbtValue::Compound(compound) = current_compound.get_value_mut(tree.remove(0))? else {
                return None;
            };
            current_compound = compound;
        };
        return None;
    }

    /// Get a ***mutable*** reference to a [`NbtValue`], querying by the [`NbtTag::key`] (a string key).
    /// > Mutable version of [`NbtCompound::get_value`].
    ///
    /// Same as [`NbtCompound::get_mut`], but returns a mutable reference to just the [`value`](`NbtTag::value`) field of the
    /// [`NbtTag`] (which is of type [`NbtValue`]).
    /// Use this if you don't need the actual tag and just want the value it stores (you usually
    /// don't need the tag, unless sending over the network, because you already know the key from
    /// the `key` parameter passed here).
    ///
    /// ## Parameters
    /// - `key`: an `&str` representing the key (or key path) of the [`NbtTag`]'s [`NbtValue`] to return from the compound.
    /// 
    /// ## Returns
    /// - `Some(&mut NbtValue)` with just the tag's [`NbtValue`] corresponding to the provided `key`, if that tag exists
    ///   in the compound.
    /// - `None` if the compound does not contain the key (path).
    ///
    /// ## Key paths
    /// - If your provided `key` contains a slash (`/`), then this function will do a *nested*
    ///   search of all sub-compounds.
    /// - For example, a key of `biome/plains` will search for a child compound (of this compound)
    ///   with the key `biome`. If it finds it (else returns null), it will then search *that*
    ///   compound for an [`NbtTag`] with a key of `plains`, and **return that tag's [`NbtValue`]**.
    pub fn get_value_mut(&mut self, key: &str) -> Option<&mut NbtValue> {
        return self.get_mut(key).map(|tag| return &mut tag.value);
    }

    /// Add an [`NbtTag`] to the [`NbtCompound`], or replace an existing one.
    ///
    /// ## Replacement
    /// - If a tag with the same key already exists in the compound, then it will be modified
    ///   in-place to take the new tag's value and type that was passed in.
    /// - This means that references to the existing tag will still be valid (of course though, Rust's
    ///   borrow-checker may prevent you from having any references to the tag when you call this
    ///   method).
    /// - The [`NbtTag`] itself is modified with the [`NbtTag::value`] being replaced with the
    ///   [`NbtValue`] of the passed in `tag` parameter.
    /// - Ignoring references, this is effectively the same as **removing the existing tag with the
    ///   same key** and then inserting the new tag.
    ///
    /// ## Insertion
    /// - If a tag with the name of the passed in `tag`'s [`NbtTag::key`] does not already exist in
    ///   the compound, then the passed in `tag` is simply added to the compound (pushed to the
    ///   internal `Vec` of [`NbtTag`]s).
    /// 
    /// ## Parameters
    /// - `&mut self`: the [`NbtTag`] to insert into.
    /// - `tag`: the [`NbtTag`] to insert into the compound. See above for details on how it is added.
    pub fn insert(&mut self, tag: NbtTag) {
        if let Some(existing_tag) = self.get_mut(&tag.key) {
            *existing_tag = tag;
        } else {
            self.0.push(tag);
        }
    }

    /// Remove an [`NbtTag`] from the compound, if it exists, by its key.
    /// 
    /// ## Nested key paths
    /// Note: currently this **does not** support nested key paths like the
    /// [`get`](`NbtCompound::get`) method (and derivatives) do. This may be added in the future,
    /// but for now, paths like `biome/plains` to refer to a tag with key `plains` inside a child
    /// compound with key `biome` are not supported by this method.
    ///
    /// ## Parameters
    /// - `&mut self`: the [`NbtCompound`] to remove the tag from.
    /// - `key`: the key of the tag to remove from the compound (see [`NbtTag::key`]).
    ///
    /// ## Returns
    /// - `Some(NbtTag)` with the removed tag, if a tag with the provided `key` existed in the compound.
    /// - `None` if the compound did not contain a tag with the provided `key`.
    ///
    /// In many situations, you may want to check whether this method returns [`None`], because in a
    /// lot of cases, trying to remove a tag that doesn't exist in the compound may be an error.
    /// If you want to verify that a tag with the provided `key` exists before trying to remove it,
    /// you can `unwrap` the result, or add proper error handling, instead of silently failing.
    pub fn remove(&mut self, key: &str) -> Option<NbtTag> {
        if let Some(pos) = self.0.iter().position(|tag| return tag.key == key) {
            return Some(self.0.remove(pos));
        }
        return None;
    }

    /// Check if the compound contains a tag with the provided `key`.
    ///
    /// ## Parameters
    /// - `&self`: immutable reference to the [`NbtCompound`] to check for the presence of the tag.
    /// - `key`: the key of the tag to check for in the compound (see [`NbtTag::key`]).
    ///
    /// ## Returns
    /// - `true` if the compound contains a tag with the provided `key`.
    /// - `false` if the compound does not contain a tag with the provided `key`.
    /// 
    /// ## Value checking
    /// If you want to check whether the compound contains a tag with a specific value, you can use
    /// the [`contains_value`](`NbtCompound::contains_value`) method instead, which checks whether
    /// the compound contains a tag with a specific value, regardless of the key.
    ///
    /// ## Nested key paths
    /// Currently this **does not** support nested key paths like the [`get`](`NbtCompound::get`)
    /// method (and derivatives) do. This may be added in the future, but for now, paths like
    /// `biome/plains` to refer to a tag with key `plains` inside a child compound with key `biome`
    /// are not supported by this method.
    #[must_use]
    pub fn contains_key(&self, key: &str) -> bool {
        return self.0.iter().any(|tag| return tag.key == key);
    }

    /// Check if the compound contains a tag with the provided `value`.
    ///
    /// ## Parameters
    /// - `&self`: immutable reference to the [`NbtCompound`] to check for the presence of the tag.
    /// - `value`: the value of the tag to check for in the compound (see [`NbtTag::value`]).
    ///
    /// ## Returns
    /// - `true` if the compound contains a tag with the provided `value`.
    /// - `false` if the compound does not contain a tag with the provided `value`.
    ///
    /// ## Key checking
    /// If you want to check whether the compound contains a tag with a specific key, you should
    /// instead use the [`NbtCompound::contains_key`] method, which checks for the presence of a
    /// key, not value.
    ///
    /// ## Nested search
    /// Currently, this function **does not** do a nested search. It only checks the tags in the
    /// current compound, and does not check any child compounds.
    /// So even if `false` is returned, that value may exist in a child compound. You would need to
    /// manually check the child compounds for that value if you want to be sure it doesn't exist
    /// anywhere in the compound.
    #[must_use]
    pub fn contains_value(&self, value: &NbtValue) -> bool {
        return self.0.iter().any(|tag| return &tag.value == value);
    }

    /// Returns the number of tags in the compound.
    ///
    /// This returns the total number of *direct* children: it does not sum any [`NbtTag`]s or
    /// [`NbtValue`]s in child compounds. So if you have a compound with 2 tags, and one of those
    /// tags is a compound with 3 tags, this will return `2`, not `5`.
    ///
    /// ## Parameters
    /// - `&self`: immutable reference to the [`NbtCompound`] to check the number of tags of.
    ///
    /// ## Returns
    /// - `usize` representing the number of tags in the compound.
    #[must_use]
    pub const fn len(&self) -> usize {
        return self.0.len();
    }

    #[must_use]
    pub const fn is_empty(&self) -> bool {
        return self.0.is_empty();
    }

    pub fn clear(&mut self) {
        return self.0.clear();
    }

    #[must_use]
    pub fn keys(&self) -> Vec<&str> {
        return self.0.iter().map(|tag| return tag.key.as_str()).collect();
    }

    #[must_use]
    pub fn values(&self) -> Vec<&crate::NbtValue> {
        return self.0.iter().map(|tag| return &tag.value).collect();
    }

    pub fn values_mut(&mut self) -> Vec<&mut crate::NbtValue> {
        return self.0.iter_mut().map(|tag| return &mut tag.value).collect();
    }
}
