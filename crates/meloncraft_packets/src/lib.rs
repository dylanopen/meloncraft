pub mod clientbound_packet;
pub mod incoming_packet;

pub mod clientbound;
pub mod clientbound_messenger;
pub mod incoming;
pub mod incoming_messenger;

use crate::clientbound_messenger::forward_clientbound_packet;
use crate::incoming_messenger::{forward_incoming_packet, read_new_packets};
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
        app.add_message::<incoming::login::LoginAcknowledged>();

        app.add_message::<clientbound::status::StatusResponse>();
        app.add_message::<clientbound::status::Pong>();
        app.add_message::<clientbound::login::Disconnect>();
        app.add_message::<clientbound::login::EncryptionRequest>();
        app.add_message::<clientbound::login::LoginSuccess>();
        app.add_message::<clientbound::login::SetCompression>();
        app.add_message::<clientbound::configuration::CookieRequest>();
        app.add_message::<clientbound::configuration::Disconnect>();
        app.add_message::<clientbound::configuration::FinishConfiguration>();
        app.add_message::<clientbound::configuration::KeepAlive>();
        app.add_message::<clientbound::configuration::Ping>();
        app.add_message::<clientbound::configuration::ResetChat>();
        app.add_message::<clientbound::configuration::RegistryData>();

        app.add_systems(Update, read_new_packets);

        app.add_systems(
            Update,
            (
                forward_incoming_packet::<incoming::handshaking::Intention>,
                forward_incoming_packet::<incoming::status::StatusRequest>,
                forward_incoming_packet::<incoming::status::Ping>,
                forward_incoming_packet::<incoming::login::LoginStart>,
                forward_incoming_packet::<incoming::login::EncryptionResponse>,
                forward_incoming_packet::<incoming::login::LoginAcknowledged>,
            ),
        );

        app.add_systems(
            Update,
            (
                forward_clientbound_packet::<clientbound::status::StatusResponse>,
                forward_clientbound_packet::<clientbound::status::Pong>,
                forward_clientbound_packet::<clientbound::login::Disconnect>,
                forward_clientbound_packet::<clientbound::login::EncryptionRequest>,
                forward_clientbound_packet::<clientbound::login::LoginSuccess>,
                forward_clientbound_packet::<clientbound::login::SetCompression>,
                forward_clientbound_packet::<clientbound::configuration::CookieRequest>,
                forward_clientbound_packet::<clientbound::configuration::Disconnect>,
                forward_clientbound_packet::<clientbound::configuration::FinishConfiguration>,
                forward_clientbound_packet::<clientbound::configuration::KeepAlive>,
                forward_clientbound_packet::<clientbound::configuration::Ping>,
                forward_clientbound_packet::<clientbound::configuration::ResetChat>,
                forward_clientbound_packet::<clientbound::configuration::RegistryData>,
            ),
        );
    }
}
