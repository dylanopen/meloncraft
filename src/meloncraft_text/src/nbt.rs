//! Module for enum [`NbtText`].

use meloncraft_nbt::{NbtCompound, NbtString};

/// Binary, NBT format for representing text.
///
/// Text in Meloncraft can be represented as an [`NbtValue`](meloncraft_nbt::NbtValue), using this
/// struct. This is the main type for text, and it can be either a plain string or
/// formatted text, depending on the variant you create.
#[derive(Debug, Clone)]
pub enum NbtText {
    /// A plain string, without any formatting.
    /// This is the simplest form of text; it takes just one [`NbtString`] type.
    Plain(NbtString),

    /// A formatted text, which can include various formatting options like colors, styles, and
    /// additional text components.
    /// This variant uses an [`NbtCompound`] to represent a more complex text component.
    /// See the documentation for [`NbtCompound`] for info on the structure of a compound.
    ///
    /// The format of the compound is very similar to the [`SnbtText`](crate::snbt::SnbtText) struct,
    /// but it is represented in binary NBT format instead of SNBT. More documentation may be
    /// available for [`SnbtText`](crate::snbt::SnbtText) if you want to understand the structure of
    /// text components.
    Formatted(NbtCompound),
}
