use bevy::app::{App, Plugin, Update};

use crate::send;

pub struct MeloncraftChatBroadcastingPlugin;

impl Plugin for MeloncraftChatBroadcastingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, send::send_player_chat);
    }
}
