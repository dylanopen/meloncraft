use bevy::prelude::Component;

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
