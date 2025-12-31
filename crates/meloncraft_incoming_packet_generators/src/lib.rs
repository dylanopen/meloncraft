use bevy::app::{App, Plugin, Update};
use meloncraft_packets::incoming::handshaking::Intention;
use meloncraft_packets::incoming::status::StatusRequest;

pub mod forward;

pub struct MeloncraftPacketGeneratorsPlugin;

impl Plugin for MeloncraftPacketGeneratorsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, forward::forward_packet::<Intention>);
        app.add_systems(Update, forward::forward_packet::<StatusRequest>);
    }
}
