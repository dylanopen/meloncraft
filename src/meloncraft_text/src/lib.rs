//! Crate containing data types for representing text components as NBT and SNBT formats.
//! Used internally to store text, as well as in clientbound and serverbound packets.
//!
//! For the [Meloncraft](https://github.com/dylanopen/meloncraft) server implementation.
//!
//! See the documentation for the individual modules, structs and enums for more information on the
//! data types contained in this crate, and their usage in packets and systems throughout the
//! codebase.
//!
//! ## Pure `type` crate
//! `meloncraft_text` is a *type* crate. This means it doesn't hold any systems or stateful
//! functions. It just holds datatypes, which themselves are *used* by systems and functions in other
//! crates.
//! Impure functions are not allowed in this crate, and so should any functions not directly related
//! to operating on the datatype itself only (utility functions for conversions, instantiation,
//! logic, etc. are fine but only if they operate on the datatype itself, and not on any external
//! state or external mutable parameters).

mod snbt;
mod nbt;

pub use snbt::SnbtText;
pub use nbt::NbtText;
