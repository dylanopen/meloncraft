pub mod clientbound_packet;
pub mod serverbound_packet;

pub mod clientbound;
pub mod clientbound_messenger;
pub mod serverbound;
pub mod serverbound_messenger;

use bevy::app::{App, Plugin};
pub use serverbound_packet::ServerboundPacket;
pub use clientbound_packet::ClientboundPacket;

pub use clientbound::all::*;

pub struct MeloncraftPacketsPlugin;

impl Plugin for MeloncraftPacketsPlugin {
    fn build(&self, app: &mut App) {
        serverbound::handshaking::register_packets(app);
        serverbound::status::register_packets(app);
        serverbound::login::register_packets(app);
        serverbound::configuration::register_packets(app);
        serverbound::play::register_packets(app);

        clientbound::status::register_packets(app);
        clientbound::login::register_packets(app);
        clientbound::configuration::register_packets(app);
        clientbound::play::register_packets(app);
    }
}
