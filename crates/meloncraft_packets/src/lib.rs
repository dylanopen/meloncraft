pub mod incoming_packet;
pub mod outgoing_packet;

pub mod incoming;
pub mod incoming_messenger;
pub mod outgoing;
pub mod outgoing_messenger;

use crate::incoming_messenger::{forward_incoming_packet, read_new_packets};
use crate::outgoing_messenger::forward_outgoing_packet;
use bevy::app::{App, Plugin, Update};
pub use incoming_packet::IncomingPacket;

pub struct MeloncraftPacketsPlugin;

impl Plugin for MeloncraftPacketsPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<incoming::handshaking::Intention>();
        app.add_message::<incoming::status::StatusRequest>();
        app.add_message::<incoming::status::Ping>();
        app.add_message::<incoming::login::LoginStart>();
        app.add_message::<incoming::login::EncryptionResponse>();

        app.add_message::<outgoing::status::StatusResponse>();
        app.add_message::<outgoing::status::Pong>();

        app.add_systems(Update, read_new_packets);

        app.add_systems(
            Update,
            (
                forward_incoming_packet::<incoming::handshaking::Intention>,
                forward_incoming_packet::<incoming::status::StatusRequest>,
                forward_incoming_packet::<incoming::status::Ping>,
                forward_incoming_packet::<incoming::login::LoginStart>,
                forward_incoming_packet::<incoming::login::EncryptionResponse>,
            ),
        );

        app.add_systems(
            Update,
            (
                forward_outgoing_packet::<outgoing::status::StatusResponse>,
                forward_outgoing_packet::<outgoing::status::Pong>,
            ),
        );
    }
}
