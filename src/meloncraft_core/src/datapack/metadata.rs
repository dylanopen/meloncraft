//! Module for struct [`DatapackMetadata`].

/// Represents the metadata of a datapack, which is used to identify a datapack by its information
/// in the game.
///
/// See <https://minecraft.wiki/w/Data_pack> for general information about datapacks.
///
/// ## Packet usage
/// Potentially used in many places: the main ones that come to mind is
/// `ClientboundSelectKnownPacks` and `ServerboundSelectKnownPacks`.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct DatapackMetadata {

    /// The **namespcae** of the datapack's name, which is the part **before** the colon (`:`) in
    /// the datapack's name.
    ///
    /// This is usually `minecraft` for vanilla datapacks.
    ///
    /// ## Example
    /// In the datapack name `minecraft:core`, the namespace is `minecraft`.
    pub namespace: String,

    pub id: String,

    pub version: String,
}
