use crate::incoming::{forward_incoming_packet, read_new_packets};
use crate::outgoing::forward_outgoing_packet;
use bevy::app::{App, Plugin, Update};
use bevy::prelude::IntoScheduleConfigs;
use meloncraft_packets::incoming::handshaking::Intention;
use meloncraft_packets::incoming::status::StatusRequest;
use meloncraft_packets::outgoing::status::StatusResponse;

pub mod incoming;
pub mod outgoing;

pub struct MeloncraftPacketGeneratorsPlugin;

impl Plugin for MeloncraftPacketGeneratorsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, read_new_packets);

        app.add_systems(
            Update,
            (
                forward_incoming_packet::<Intention>,
                forward_incoming_packet::<StatusRequest>,
            ),
        );

        app.add_systems(Update, forward_outgoing_packet::<StatusResponse>);
    }
}
