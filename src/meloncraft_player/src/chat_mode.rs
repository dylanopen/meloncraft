//! Module for component struct [`ChatColors`].

use bevy::prelude::Component;

/// Component storing the player's chat mode preferences. This is used to determine whether the
/// player wants to see chat messages at all, or only command feedback, or all chat messages.
///
/// You should check this component before sending chat messages or command outputs to the user.
///
/// *Component for a player/client entity*.
///
/// See the variants' documentation for each mode and its implications.
///
/// Further reading: <https://minecraft.wiki/w/Java_Edition_protocol/Chat#Client_chat_mode>.

#[derive(Component, Debug, Clone, Copy)]
#[repr(i32)]
pub enum ChatMode {
    Enabled,
    CommandsOnly,
    Hidden,
}

impl TryFrom<i32> for ChatMode {
    type Error = ();
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        return match value {
            0 => Ok(ChatMode::Enabled),
            1 => Ok(ChatMode::CommandsOnly),
            2 => Ok(ChatMode::Hidden),
            _ => Err(()),
        };
    }
}
