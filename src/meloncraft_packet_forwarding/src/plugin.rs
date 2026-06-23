use crate::{chat_command, chat_message, intention, player_action};
use bevy::app::{App, Plugin, Update};

pub struct MeloncraftPacketForwardingPlugin;

impl Plugin for MeloncraftPacketForwardingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, player_action::fwd_block_broken);
        app.add_systems(Update, player_action::fwd_player_broke_block);

        app.add_systems(Update, intention::fwd_handshaken);

        app.add_systems(Update, chat_message::fwd_player_sent);
        app.add_systems(Update, chat_message::fwd_send_chat);
        app.add_systems(Update, chat_message::fwd_send_title);
        app.add_systems(Update, chat_message::fwd_clear_titles);

        app.add_systems(Update, chat_command::fwd_raw_command);
    }
}
