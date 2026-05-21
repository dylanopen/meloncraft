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

    /// Protocol ID: `0x01` (1).
    /// Stores an unsigned 8-bit integer (`u8`), wrapped in an [`NbtU8`] struct.
    /// See [`NbtU8`] for more information about this type.
    U8(NbtU8),

    /// Protocol ID: `0x02` (2).
    /// Stores a signed 16-bit integer (`i16`), wrapped in an [`NbtI16`] struct.
    /// See [`NbtI16`] for more information about this type.
    I16(NbtI16),

    /// Protocol ID: `0x03` (3).
    /// Stores a signed 32-bit integer (`i32`), wrapped in an [`NbtI32`] struct.
    /// See [`NbtI32`] for more information about this type.
    I32(NbtI32),

    /// Protocol ID: `0x04` (4).
    /// Stores a signed 64-bit integer (`i64`), wrapped in an [`NbtI64`] struct.
    /// See [`NbtI64`] for more information about this type.
    I64(NbtI64),

    /// Protocol ID: `0x05` (5).
    /// Stores a 32-bit floating point number (`f32`), wrapped in an [`NbtF32`] struct.
    /// See [`NbtF32`] for more information about this type.
    F32(NbtF32),

    /// Protocol ID: `0x06` (6).
    /// Stores a 64-bit floating point number (`f64`), wrapped in an [`NbtF64`] struct.
    /// See [`NbtF64`] for more information about this type.
    F64(NbtF64),

    /// Protocol ID: `0x07` (7).
    /// Stores an array of unsigned 8-bit integers (`Vec<u8>`), wrapped in an [`NbtArrayU8`] struct.
    /// This is a **byte array**. It is an efficient way of sending a dynamic list of bytes; it
    /// cannot store any other type of value. It is different from an [`NbtList`] of
    /// [`NbtValue::U8`]s, which is a list of unsigned 8-bit integers that is less efficient to
    /// serialize and deserialize.
    /// See [`NbtArrayU8`] for more information about this type.
    ArrayU8(NbtArrayU8),

    /// Protocol ID: `0x08` (8).
    /// Stores a string (`String`), wrapped in an [`NbtString`] struct.
    /// See [`NbtString`] for more information about this type.
    String(NbtString),

    /// Protocol ID: `0x09` (9).
    /// Stores a list of NBT values (`Vec<NbtValue>`), wrapped in an [`NbtList`] struct.
    /// This is a **list**. It can store a variable number of NBT values, but all values should be
    /// of the same type. It is different from an array, which is a more efficient way of storing a
    /// list of values of a specific type. It is also different from a compound, which is a
    /// hashmap-like structure that can store values of different types and has string keys.
    /// Can seemingly hold any type, but only one type per list.
    /// See [`NbtList`] for more information about this type.
    List(NbtList),

    /// Protocol ID: `0x0A` (10).
    /// Stores a list of NBT tags (`Vec<NbtTag>`), wrapped in an [`NbtCompound`] struct.
    /// This is a **compound**. It can store a variable number of NBT tags, which are key-value
    /// pairs where the key is a string and the value is an NBT value. It is different from a list,
    /// which is an array-like structure that can only store values of the same type and has no
    /// keys. It is also different from an array, which is a more efficient way of storing a list of
    /// values of a specific type and has no keys.
    /// Can hold multiple different types of values in one compound.
    /// See [`NbtCompound`] for a detailed breakdown of this NBT type.
    Compound(NbtCompound),

    /// Protocol ID: `0x0B` (11).
    /// Stores an array of signed 32-bit integers (`Vec<i32>`), wrapped in an [`NbtArrayI32`] struct.
    /// This is an **integer array**. It is an efficient way of sending a dynamic list of signed
    /// 32-bit integers; it cannot store any other type of value. It is different from an
    /// [`NbtList`] of [`NbtValue::I32`]s, which is a list of signed 32-bit integers that is less
    /// efficient to serialize and deserialize.
    /// See [`NbtArrayI32`] for more information about this type.
    ArrayI32(NbtArrayI32),

    /// Protocol ID: `0x0C` (12).
    /// Stores an array of signed 64-bit integers (`Vec<i64>`), wrapped in an [`NbtArrayI64`]
    /// struct. This is a **long array**. It is an efficient way of sending a dynamic list of signed
    /// 64-bit integers; it cannot store any other type of value. It is different from an
    /// [`NbtList`] of [`NbtValue::I64`]s, which is a list of signed 64-bit integers that is less
    /// efficient to serialize and deserialize.
    /// See [`NbtArrayI64`] for more information about this type.
    ArrayI64(NbtArrayI64),
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
