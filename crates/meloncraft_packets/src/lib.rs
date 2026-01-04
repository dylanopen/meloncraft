pub mod clientbound_packet;
pub mod serverbound_packet;

pub mod clientbound;
pub mod clientbound_messenger;
pub mod serverbound;
pub mod serverbound_messenger;

use crate::clientbound_messenger::forward_clientbound_packet;
use crate::serverbound_messenger::{forward_serverbound_packet, read_new_packets};
use bevy::app::{App, Plugin, PostUpdate, PreUpdate, Update};
pub use serverbound_packet::ServerboundPacket;

pub struct MeloncraftPacketsPlugin;

impl Plugin for MeloncraftPacketsPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<serverbound::handshaking::Intention>();
        app.add_message::<serverbound::status::StatusRequest>();
        app.add_message::<serverbound::status::Ping>();
        app.add_message::<serverbound::login::LoginStart>();
        app.add_message::<serverbound::login::EncryptionResponse>();
        app.add_message::<serverbound::login::LoginAcknowledged>();
        app.add_message::<serverbound::configuration::ClientInformation>();
        app.add_message::<serverbound::configuration::CookieResponse>();
        app.add_message::<serverbound::configuration::FinishConfiguration>();
        app.add_message::<serverbound::configuration::KeepAlive>();

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
        app.add_message::<clientbound::configuration::RemoveResourcePack>();
        app.add_message::<clientbound::configuration::AddResourcePack>();
        app.add_message::<clientbound::configuration::StoreCookie>();
        app.add_message::<clientbound::configuration::Transfer>();
        app.add_message::<clientbound::configuration::SetFeatureFlags>();
        app.add_message::<clientbound::configuration::UpdateTags>();
        app.add_message::<clientbound::configuration::SelectKnownPacks>();
        app.add_message::<clientbound::configuration::CustomReportDetails>();
        app.add_message::<clientbound::configuration::ServerLinks>();
        app.add_message::<clientbound::configuration::ClearDialog>();
        app.add_message::<clientbound::configuration::ShowDialog>();
        app.add_message::<clientbound::configuration::CodeOfConduct>();

        app.add_systems(Update, read_new_packets);

        app.add_systems(
            PreUpdate,
            (
                forward_serverbound_packet::<serverbound::handshaking::Intention>,
                (
                    forward_serverbound_packet::<serverbound::status::StatusRequest>,
                    forward_serverbound_packet::<serverbound::status::Ping>,
                ),
                (
                    forward_serverbound_packet::<serverbound::login::LoginStart>,
                    forward_serverbound_packet::<serverbound::login::EncryptionResponse>,
                    forward_serverbound_packet::<serverbound::login::LoginAcknowledged>,
                ),
                (
                    forward_serverbound_packet::<serverbound::configuration::ClientInformation>,
                    forward_serverbound_packet::<serverbound::configuration::CookieResponse>,
                    forward_serverbound_packet::<serverbound::configuration::FinishConfiguration>,
                    forward_serverbound_packet::<serverbound::configuration::KeepAlive>,
                ),
            ),
        );

        app.add_systems(
            PostUpdate,
            (
                (
                    forward_clientbound_packet::<clientbound::status::StatusResponse>,
                    forward_clientbound_packet::<clientbound::status::Pong>,
                ),
                (
                    forward_clientbound_packet::<clientbound::login::Disconnect>,
                    forward_clientbound_packet::<clientbound::login::EncryptionRequest>,
                    forward_clientbound_packet::<clientbound::login::LoginSuccess>,
                    forward_clientbound_packet::<clientbound::login::SetCompression>,
                ),
                (
                    forward_clientbound_packet::<clientbound::configuration::CookieRequest>,
                    forward_clientbound_packet::<clientbound::configuration::Disconnect>,
                    forward_clientbound_packet::<clientbound::configuration::FinishConfiguration>,
                    forward_clientbound_packet::<clientbound::configuration::KeepAlive>,
                    forward_clientbound_packet::<clientbound::configuration::Ping>,
                    forward_clientbound_packet::<clientbound::configuration::ResetChat>,
                    forward_clientbound_packet::<clientbound::configuration::RegistryData>,
                    forward_clientbound_packet::<clientbound::configuration::RemoveResourcePack>,
                    forward_clientbound_packet::<clientbound::configuration::AddResourcePack>,
                    forward_clientbound_packet::<clientbound::configuration::AddResourcePack>,
                    forward_clientbound_packet::<clientbound::configuration::Transfer>,
                    forward_clientbound_packet::<clientbound::configuration::SetFeatureFlags>,
                    forward_clientbound_packet::<clientbound::configuration::SelectKnownPacks>,
                    forward_clientbound_packet::<clientbound::configuration::CustomReportDetails>,
                    forward_clientbound_packet::<clientbound::configuration::ServerLinks>,
                    forward_clientbound_packet::<clientbound::configuration::ClearDialog>,
                    forward_clientbound_packet::<clientbound::configuration::ShowDialog>,
                    forward_clientbound_packet::<clientbound::configuration::CodeOfConduct>,
                ),
            ),
        );
    }
}
