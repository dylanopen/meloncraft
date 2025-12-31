mod incoming_packet;
pub mod incoming;

use bevy::app::{App, Plugin};
pub use incoming_packet::IncomingPacket;

pub struct MeloncraftPacketsPlugin;

impl Plugin for MeloncraftPacketsPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<incoming::handshaking::Handshake>();
    }
}
