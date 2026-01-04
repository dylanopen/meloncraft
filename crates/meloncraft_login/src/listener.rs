use crate::encryption::EncryptionMode;
use bevy::prelude::{MessageReader, MessageWriter, Query, Res};
use meloncraft_client::connection::ClientConnection;
use meloncraft_client::connection_state::ConnectionState;
use meloncraft_packets::clientbound::login::LoginSuccess;
use meloncraft_packets::incoming::login::LoginStart;
use meloncraft_player::GameProfile;

pub fn login_offline_unencrypted_listener(
    mut login_start_pr: MessageReader<LoginStart>,
    encryption_mode: Res<EncryptionMode>,
    mut client_connections: Query<&mut ClientConnection>,
    mut login_success_pw: MessageWriter<LoginSuccess>,
) {
    if *encryption_mode != EncryptionMode::OfflineUnencrypted {
        return; // we only handle offline-mode and unencrypted in this system
    }
    for packet in login_start_pr.read() {
        login_success_pw.write(LoginSuccess {
            client: packet.client,
            game_profile: GameProfile {
                username: packet.name.clone(),
                uuid: packet.uuid.clone(),
            },
        });
        client_connections.get_mut(packet.client).unwrap().state = ConnectionState::Configuration;
    }
}
