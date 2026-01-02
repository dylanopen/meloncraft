use crate::incoming::{forward_incoming_packet, read_new_packets};
use crate::outgoing::forward_outgoing_packet;
use bevy::app::{App, Plugin, Update};
use meloncraft_packets as packets;

pub mod incoming;
pub mod outgoing;

pub struct MeloncraftPacketGeneratorsPlugin;

impl Plugin for MeloncraftPacketGeneratorsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, read_new_packets);

        app.add_systems(
            Update,
            (
                forward_incoming_packet::<packets::incoming::handshaking::Intention>,
                forward_incoming_packet::<packets::incoming::status::StatusRequest>,
                forward_incoming_packet::<packets::incoming::status::Ping>,
            ),
        );

        app.add_systems(
            Update,
            (
                forward_outgoing_packet::<packets::outgoing::status::StatusResponse>,
                forward_outgoing_packet::<packets::outgoing::status::Pong>,
            ),
        );
    }
}
