//! Module for struct [`SnbtText`].

/// Container for Minecraft text, in the form of an SNBT (stringified NBT) value.
///
/// The protocol has various packets where text, in the form of *components*, need to be sent. These
/// can be sent either in the form of [`SnbtText`], or, more commonly, as
/// [`NbtText`](`crate::NbtText`).
/// The main difference between the two is that [`SnbtText`] is a stringified SNBT (similar to JSON,
/// but with some differences) and is usually used when *users* need to input text, while
/// [`NbtText`] is a binary NBT structure encoded as a regular NBT tag value.
///
/// Some packets use [`SnbtText`], while some others use [`NbtText`]. Check the packet's
/// Meloncraft documentation, or the wiki for packets.
///
/// The SNBT format for text components is explained at <https://minecraft.wiki/w/Text_component_format>.
///
/// See [`SnbtText::data`] for more information about the data this type should hold.
#[derive(Debug, Clone)]
pub struct SnbtText {
    pub data: String,
}
