//! Module for enum [`NbtText`].

use meloncraft_nbt::{NbtCompound, NbtString};

/// Binary, NBT format for representing text.
///
/// Text in Meloncraft can be represented as an [`NbtValue`](meloncraft_nbt::NbtValue), using this
/// struct. This is the main type for text, and it can be either a plain string or
/// formatted text, depending on the variant you create.
#[derive(Debug, Clone)]
pub enum NbtText {
    Plain(NbtString),
    Formatted(NbtCompound),
}

