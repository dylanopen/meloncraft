//! Module for component struct [`ChatColors`].

use bevy::prelude::Component;

/// Component storing whether the player should be sent chat messages in colour or not.
/// You should check this component before sending chat messages to the user.
/// *Component for a player/client entity*.
///
/// - If true, the client supports colored chat, so you are able to send whatever formatting you
///   like in chat messages.
/// - If false, the server **should not** send any chat messages in colour; instead send it in white
///   text only. The vanilla server doesn't actually read this, though.
///
/// You technically do not have to acknowledge this request, but you *should*.
#[derive(Component, Debug, Clone, Copy)]
pub struct ChatColors(pub bool);
