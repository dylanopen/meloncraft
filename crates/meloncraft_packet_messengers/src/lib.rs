use bevy::app::{App, Plugin, Update};
use crate::incoming::forward_incoming_packet;
use crate::outgoing::forward_outgoing_packet;
use meloncraft_packets::incoming::handshaking::Intention;
use meloncraft_packets::incoming::status::StatusRequest;
use meloncraft_packets::outgoing::status::StatusResponse;

pub mod incoming;
pub mod outgoing;

pub struct MeloncraftPacketGeneratorsPlugin;

impl Plugin for MeloncraftPacketGeneratorsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, forward_incoming_packet::<Intention>);
        app.add_systems(Update, forward_incoming_packet::<StatusRequest>);

        app.add_systems(Update, forward_outgoing_packet::<StatusResponse>);
    }
}
