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
pub struct NbtCompound(pub Vec<NbtTag>);

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

    #[must_use]
    pub fn get_value(&self, key: &str) -> Option<&NbtValue> {
        return self.get(key).map(|tag| return &tag.value);
    }

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

    pub fn get_value_mut(&mut self, key: &str) -> Option<&mut NbtValue> {
        return self.get_mut(key).map(|tag| return &mut tag.value);
    }

    pub fn insert(&mut self, tag: NbtTag) {
        if let Some(existing_tag) = self.get_mut(&tag.key) {
            *existing_tag = tag;
        } else {
            self.0.push(tag);
        }
    }

    pub fn remove(&mut self, key: &str) -> Option<NbtTag> {
        if let Some(pos) = self.0.iter().position(|tag| return tag.key == key) {
            return Some(self.0.remove(pos));
        }
        return None;
    }

    #[must_use]
    pub fn contains_key(&self, key: &str) -> bool {
        return self.0.iter().any(|tag| return tag.key == key);
    }

    #[must_use]
    pub fn contains_value(&self, value: &NbtValue) -> bool {
        return self.0.iter().any(|tag| return &tag.value == value);
    }

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
