//! Module for component struct [`DisplayedSkinParts`].

use bevy::prelude::Component;

/// Player [`Component`] defining the parts of their skin that the player would like to display to
/// other players on the server.
///
/// This is sent by the client in the `ServerboundClientInformation` packet.
///
/// ## Protocol representation
/// In the protocol, this is represented as a `u8`, where each bit is a field in a bitmask.
/// You can use `&` or `|` to set/get a specific field's value, from its binary representation.
/// See this struct's fields for more the mask of each field.
///
/// Further reading: <https://minecraft.wiki/w/Java_Edition_protocol/Packets#Client_Information_(configuration)>.
#[derive(Component, Debug, Clone, Eq, PartialEq)]
#[expect(clippy::struct_excessive_bools, reason = "Not a state machine, but could be replaced with a packet bit set in the future.")]
pub struct DisplayedSkinParts {

    /// [`DisplayedSkinParts`] field boolean, where the value is:
    /// - `true` if the player's cape is visible.
    /// - `false` if the player's cape is not visible.
    ///
    /// **Protocol mask: `0x_01`**, or `0b_0000_0001` (least significant bit).
    pub cape: bool,

    /// [`DisplayedSkinParts`] field boolean, where the value is:
    /// - `true` if the player's jacket is visible.
    /// - `false` if the player's jacket is not visible.
    ///
    /// **Protocol mask: `0x_02`**, or `0b_0000_0010` (second-to-least significant bit).
    pub jacket: bool,

    /// [`DisplayedSkinParts`] field boolean, where the value is:
    /// - `true` if the player's left sleeve is visible.
    /// - `false` if the player's left sleeve is not visible.
    ///
    /// **Protocol mask: `0x_04`**, `0b_0000_0100` (third-to-least significant bit).
    pub left_sleeve: bool,
    pub right_sleeve: bool,
    pub left_pants_leg: bool,
    pub right_pants_leg: bool,
    pub hat: bool,
}
