extern crate core;

pub mod clientbound_packet;
pub mod serverbound_packet;

pub mod clientbound;
pub mod clientbound_messenger;
pub mod network_messages;
pub mod serverbound;
pub mod serverbound_messenger;

use bevy::app::{App, Plugin};
pub use clientbound_packet::ClientboundPacket;
pub use serverbound_packet::ServerboundPacket;

pub use clientbound::*;
pub use serverbound::*;

pub struct MeloncraftPacketsPlugin;

impl Plugin for MeloncraftPacketsPlugin {
    fn build(&self, app: &mut App) {
        register_serverbound_handshaking_packets(app);
        register_serverbound_status_packets(app);
        register_serverbound_login_packets(app);
        register_serverbound_configuration_packets(app);
        register_serverbound_play_packets(app);

        register_clientbound_status_packets(app);
        register_clientbound_login_packets(app);
        register_clientbound_configuration_packets(app);
        register_clientbound_play_packets(app);
    }
}
