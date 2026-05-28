use bevy::app::{App, Plugin};

/// This plugin initializes messages related to player chat messages.
///
/// You may want to forward these messages to/from packets (see the `packet_forwarding` crate), or
/// may want to read player messages and send them with word filtering, for example.
pub struct MeloncraftChatPlugin;

impl Plugin for MeloncraftChatPlugin {
    fn build(&self, app: &mut App) {}
}
