use bevy::app::{App, Plugin, Update};
use crate::player_action;

pub struct MeloncraftPacketForwardingPlugin;

impl Plugin for MeloncraftPacketForwardingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, player_action::fwd_block_broken);
        app.add_systems(Update, player_action::fwd_player_broke_block);
    }
}