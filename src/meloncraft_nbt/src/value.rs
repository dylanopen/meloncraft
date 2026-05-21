//! Module for enum [`NbtValue`].

use crate::tag::NbtTag;
use crate::{
    NbtArrayI32, NbtArrayI64, NbtArrayU8, NbtCompound, NbtF32, NbtF64, NbtI16, NbtI32, NbtI64,
    NbtList, NbtString, NbtU8,
};

/// Enum representing the different types of NBT values that can be stored in an NBT tag.
///
/// Each variant corresponds to a specific NBT type. Each NBT type stores a certain value and may
/// implement methods specific to that type. For example, the `Compound` variant provides
/// hashmap-like methods for accessing its children, and the `List` variant provides array-like
/// methods for it through a dereference to a `Vec`.
///
/// You are advised to use this enum if you want to store an NBT value without caring about its
/// specific type, or if you want to write code that can handle multiple NBT types at once. If you
/// know the specific type of NBT value you want to store, you should use the specific struct for
/// that type (e.g. [`NbtString`] for a string).
///
/// ## Serialization
/// `ProtocolType` is implemented on this type if you import it from `meloncraft_protocol_types`, so
/// you can serialize and deserialize this type to/from a network protocol format using the traits
/// provided by that crate.
///
/// ## Constraints
/// - Please see the constraints for the specific variant's types for more information about the
///   constraints on the values stored in this enum.
///
/// ## Possible variants
/// - [`NbtValue::U8`] to store an [`NbtU8`].
/// - [`NbtValue::I16`] to store an [`NbtI16`].
/// - [`NbtValue::I32`] to store an [`NbtI32`].
/// - [`NbtValue::I64`] to store an [`NbtI64`].
/// - [`NbtValue::F32`] to store an [`NbtF32`].
/// - [`NbtValue::F64`] to store an [`NbtF64`].
/// - [`NbtValue::ArrayU8`] to store an [`NbtArrayU8`].
/// - [`NbtValue::ArrayI32`] to store an [`NbtArrayI32`].
/// - [`NbtValue::ArrayI64`] to store an [`NbtArrayI64`].
/// - [`NbtValue::String`] to store an [`NbtString`].
/// - [`NbtValue::List`] to store an [`NbtList`].
/// - [`NbtValue::Compound`] to store an [`NbtCompound`].
///
/// See each variant or type's documentation to find out more.
#[derive(Debug, Clone, PartialEq)]
pub enum NbtValue {
    U8(NbtU8),
    I16(NbtI16),
    I32(NbtI32),
    I64(NbtI64),
    F32(NbtF32),
    F64(NbtF64),
    ArrayU8(NbtArrayU8),
    ArrayI32(NbtArrayI32),
    ArrayI64(NbtArrayI64),
    String(NbtString),
    List(NbtList),
    Compound(NbtCompound),
}

impl NbtValue {
    #[must_use]
    pub fn get_compound_children(&self) -> Option<&Vec<NbtTag>> {
        if let NbtValue::Compound(compound) = self {
            return Some(compound);
        }
        return None;
    }

    pub fn get_compound_children_mut(&mut self) -> Option<&mut Vec<NbtTag>> {
        if let NbtValue::Compound(compound) = self {
            return Some(compound);
        }
        return None;
    }

    #[must_use]
    pub fn get_list_children(&self) -> Option<&Vec<NbtValue>> {
        if let NbtValue::List(list) = self {
            return Some(list);
        }
        return None;
    }

    pub fn get_list_children_mut(&mut self) -> Option<&mut Vec<NbtValue>> {
        if let NbtValue::List(list) = self {
            return Some(list);
        }
        return None;
    }

    #[must_use]
    pub fn get(&self, key: &str) -> Option<&NbtValue> {
        let NbtValue::Compound(compound) = self else {
            return None;
        };
        for tag in compound.iter() {
            if tag.key == key {
                return Some(&tag.value);
            }
        }
        return None;
    }

    pub fn get_mut(&mut self, key: &str) -> Option<&mut NbtValue> {
        let NbtValue::Compound(compound) = self else {
            return None;
        };
        for tag in compound.iter_mut() {
            if tag.key == key {
                return Some(&mut tag.value);
            }
        }
        return None;
    }

    #[must_use]
    pub const fn to_id(&self) -> u8 {
        return match self {
            NbtValue::U8(_) => 1,
            NbtValue::I16(_) => 2,
            NbtValue::I32(_) => 3,
            NbtValue::I64(_) => 4,
            NbtValue::F32(_) => 5,
            NbtValue::F64(_) => 6,
            NbtValue::ArrayU8(_) => 7,
            NbtValue::String(_) => 8,
            NbtValue::List(_) => 9,
            NbtValue::Compound(_) => 10,
            NbtValue::ArrayI32(_) => 11,
            NbtValue::ArrayI64(_) => 12,
        };
    }
}
