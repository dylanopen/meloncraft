pub mod incoming_packet;
pub mod outgoing_packet;

pub mod incoming;
pub mod outgoing;

use bevy::app::{App, Plugin};
pub use incoming_packet::IncomingPacket;

pub struct MeloncraftPacketsPlugin;

impl Plugin for MeloncraftPacketsPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<incoming::handshaking::Intention>();
        app.add_message::<incoming::status::StatusRequest>();

        app.add_message::<outgoing::status::StatusResponse>();
    }
}
