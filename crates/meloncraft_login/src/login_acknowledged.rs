use crate::encryption::EncryptionMode;
use bevy::prelude::{MessageReader, MessageWriter, Query, Res};
use meloncraft_client::connection::ClientConnection;
use meloncraft_client::connection_state::ConnectionState;
use meloncraft_packets::clientbound::login::LoginSuccess;
use meloncraft_packets::serverbound::login::{LoginAcknowledged, LoginStart};
use meloncraft_player::GameProfile;

pub fn login_acknowledged_listener(
    mut login_acknowledged_pr: MessageReader<LoginAcknowledged>,
    mut client_connections: Query<&mut ClientConnection>,
) {
    for packet in login_acknowledged_pr.read() {
        client_connections.get_mut(packet.client).unwrap().state = ConnectionState::Configuration;
    }
}
