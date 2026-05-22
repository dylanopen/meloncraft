//! Crate containing data types for representing NBT structures in Rust code, built
//! for the [Meloncraft](https://github.com/dylanopen/meloncraft) server implementation.
//!
//! See the documentation for the individual modules, structs and enums for more information on the
//! data types contained in this crate, and their usage in packets and systems throughout the
//! codebase.
//!
//! ## Pure `type` crate
//! `meloncraft_nbt` is a *type* crate. This means it doesn't hold any systems or stateful functions.
//! It just holds datatypes which themselves are *used* by systems and functions in other crates.
//! Impure functions are not allowed in this crate, and so should any functions not directly related
//! to operating on the datatype itself only (utility functions for conversions, instantiation,
//! logic, etc. are fine but only if they operate on the datatype itself, and not on any external
//! state or external mutable parameters).

mod tag;
mod value;

mod array_i32;
mod array_i64;
mod array_u8;
mod compound;
mod f32;
mod f64;
mod i16;
mod i32;
mod i64;
mod json;
mod list;
mod string;
mod u8;

pub use tag::NbtTag;
pub use value::NbtValue;

pub use array_i32::NbtArrayI32;
pub use array_i64::NbtArrayI64;
pub use array_u8::NbtArrayU8;
pub use compound::NbtCompound;
pub use f32::NbtF32;
pub use f64::NbtF64;
pub use i16::NbtI16;
pub use i32::NbtI32;
pub use i64::NbtI64;
pub use list::NbtList;
pub use string::NbtString;
pub use u8::NbtU8;
