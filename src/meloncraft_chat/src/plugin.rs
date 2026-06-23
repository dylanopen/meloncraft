use bevy::app::{App, Plugin};

use crate::send::{SendChatMessage, SendTitleMessage};
use crate::sent::PlayerSentChatMessage;

/// This plugin initializes messages related to player chat messages.
///
/// You may want to forward these messages to/from packets (see the `packet_forwarding` crate), or
/// may want to read player messages and send them with word filtering, for example.
pub struct MeloncraftChatPlugin;

impl Plugin for MeloncraftChatPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<PlayerSentChatMessage>();
        app.add_message::<SendChatMessage>();
        app.add_message::<SendTitleMessage>();
    }
}
