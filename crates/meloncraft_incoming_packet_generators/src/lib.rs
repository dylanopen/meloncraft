use bevy::app::{App, Plugin, Update};

mod handshaking;

pub struct MeloncraftPacketGeneratorsPlugin;

impl Plugin for MeloncraftPacketGeneratorsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handshaking::handshake::fwd_handshake);
    }
}