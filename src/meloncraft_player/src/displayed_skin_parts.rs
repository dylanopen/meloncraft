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
#[derive(Component, Debug, Clone, Eq, PartialEq)]
#[expect(clippy::struct_excessive_bools, reason = "Not a state machine, but could be replaced with a packet bit set in the future.")]
pub struct DisplayedSkinParts {
    pub cape: bool,
    pub jacket: bool,
    pub left_sleeve: bool,
    pub right_sleeve: bool,
    pub left_pants_leg: bool,
    pub right_pants_leg: bool,
    pub hat: bool,
}
