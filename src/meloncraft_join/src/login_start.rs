use bevy::prelude::{Commands, MessageReader, MessageWriter};
use meloncraft_login::messages::OfflineLoggedIn;
use meloncraft_packets::ClientboundLoginSuccess;

pub fn save_profile(
    mut offline_login_started_mr: MessageReader<OfflineLoggedIn>,
    mut commands: Commands,
) {
    for message in offline_login_started_mr.read() {
        commands.entity(message.client).insert(message.profile.clone());
    }
}

pub fn respond_success(
    mut offline_logged_in_mr: MessageReader<OfflineLoggedIn>,
    mut login_success_pw: MessageWriter<ClientboundLoginSuccess>,
) {
    for login in offline_logged_in_mr.read() {
        login_success_pw.write(ClientboundLoginSuccess {
            client: login.client,
            game_profile: login.profile.clone(),
        });
    }
}
