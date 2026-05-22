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

    /// [`ChatMode`] component variant on a player, indicating that the player would like to receive
    /// *all* types of chat messages, including normal chat messages, system messages, and command
    /// feedback.
    ///
    /// **Protocol ID: `0`**.
    ///
    /// ## Enabled clientbound packets
    /// The server should send these types of packets to a player with this chat mode:
    /// - `ClientboundPlayerChatMessage`: Player-initiated chat messages, including the commands
    ///   /say, /me, /msg, /tell, /w and /teammsg.
    /// - `ClientboundDisguisedChatMessage`: Messages sent by non-players using the commands /say,
    ///   /me, /msg, /tell, /w and /teammsg.
    /// - `ClientboundSystemChatMessage`: Feedback from running a command, such as "Your game mode
    ///   has been updated to creative." 
    /// - `CilentboundSystemChatMessage` *with overlay*: Game state information that is displayed above
    ///   the hot bar, such as "You may not rest now, the bed is too far away". 
    ///
    /// ## Disabled clientbound packets
    /// None, the server should send all chat message types to a player with this chat mode.
    ///
    /// ## Enabled serverbound packets
    /// The *client* will send these types of packets to the server when the player has this chat
    /// mode:
    /// - `ServerboundChatMessage`: When the player sends a normal chat message, such as "Hello
    ///   world!".
    /// - `ServerboundChatCommand`: When the player runs any Minecraft command, such as "/gamemode
    ///   creative" or "/msg meloncrafter Hello world!".
    ///
    /// ## Disabled serverbound packets
    /// None, the client should send all chat message types to the server when the player has this
    /// chat mode.
    Full,

    /// [`ChatMode`] component variant on a player, indicating that the player would like to receive
    /// command feedback and system messages, but *not* normal chat messages **or** whispers.
    ///
    /// **Protocol ID: `1`**.
    ///
    /// ## Enabled clientbound packets
    /// The server should send only these types of packets to a player with this chat mode:
    /// - `ClientboundSystemChatMessage`: Feedback from running a command, such as "Your game mode
    ///   has been updated to creative." 
    /// - `CilentboundSystemChatMessage` *with overlay*: Game state information that is displayed above
    ///   the hot bar, such as "You may not rest now, the bed is too far away". 
    ///
    /// ## Disabled clientbound packets
    /// The server **should not** send these types of packets to a player with this chat mode:
    /// - `ClientboundPlayerChatMessage`: Player-initiated chat messages, including the commands
    ///   /say, /me, /msg, /tell, /w and /teammsg.
    /// - `ClientboundDisguisedChatMessage`: Messages sent by non-players using the commands /say,
    ///   /me, /msg, /tell, /w and /teammsg.
    ///
    /// ## Enabled serverbound packets
    /// The *client* will send these types of packets to the server when the player has this chat
    /// mode:
    /// - `ServerboundChatCommand`: When the player runs any Minecraft command, such as "/gamemode
    ///   creative" or "/msg meloncrafter Hello world!".
    ///
    /// ## Disabled serverbound packets
    /// The *client* **will not** send these types of packets to the server when the player has
    /// this chat mode:
    /// - `ServerboundChatMessage`: When the player sends a normal chat message, such as "Hello
    ///   world!".
    CommandsOnly,

    /// [`ChatMode`] component variant on a player, indicating that the player would not like to
    /// receive *any* chat messages at all, including normal chat messages, system messages, and
    /// command feedback.
    /// They would only like to receive game state information that is displayed above the hot bar,
    /// such as "You may not rest now, the bed is too far away" (an 'essential' message).
    ///
    /// **Protocol ID: `2`**.
    ///
    /// ## Enabled clientbound packets
    /// The server should send only these types of packets to a player with this chat mode:
    /// - `CilentboundSystemChatMessage` *with overlay*: Game state information that is displayed above
    ///   the hot bar, such as "You may not rest now, the bed is too far away". 
    ///
    /// ## Disabled clientbound packets
    /// The server **should not** send these types of packets to a player with this chat mode:
    /// - `ClientboundPlayerChatMessage`: Player-initiated chat messages, including the commands
    ///   /say, /me, /msg, /tell, /w and /teammsg.
    /// - `ClientboundDisguisedChatMessage`: Messages sent by non-players using the commands /say,
    ///   /me, /msg, /tell, /w and /teammsg.
    /// - `ClientboundSystemChatMessage`: Feedback from running a command, such as "Your game mode
    ///   has been updated to creative." 
    ///
    /// ## Enabled serverbound packets
    /// Clients in the [`ChatMode::Hidden`] state *shouldn't send any chat-related packets*.
    ///
    /// ## Disabled serverbound packets
    /// The *client* **will not** send these types of packets to the server when the player has
    /// this chat mode:
    /// - `ServerboundChatMessage`: When the player sends a normal chat message, such as "Hello
    ///   world!".
    /// - `ServerboundChatCommand`: When the player runs any Minecraft command, such as "/gamemode
    ///   creative" or "/msg meloncrafter Hello world!".
    Hidden,
}

impl TryFrom<i32> for ChatMode {
    type Error = ();
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        return match value {
            0 => Ok(ChatMode::Full),
            1 => Ok(ChatMode::CommandsOnly),
            2 => Ok(ChatMode::Hidden),
            _ => Err(()),
        };
    }
}
