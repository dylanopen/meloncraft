use bevy::app::{App, Plugin, Update};
use crate::{intention, player_action, chat_command};

pub struct MeloncraftPacketForwardingPlugin;

impl Plugin for MeloncraftPacketForwardingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, player_action::fwd_block_broken);
        app.add_systems(Update, player_action::fwd_player_broke_block);

        app.add_systems(Update, intention::fwd_handshaken);

        app.add_systems(Update, chat_command::fwd_raw_command);
    }
}
