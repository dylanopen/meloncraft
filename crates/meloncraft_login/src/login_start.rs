use crate::encryption::EncryptionMode;
use bevy::ecs::system::Commands;
use bevy::prelude::{MessageReader, MessageWriter, Res};
use meloncraft_packets::clientbound::login::LoginSuccess;
use meloncraft_packets::serverbound::login::LoginStart;
use meloncraft_player::GameProfile;

pub fn login_offline_unencrypted_listener(
    mut login_start_pr: MessageReader<LoginStart>,
    encryption_mode: Res<EncryptionMode>,
    mut login_success_pw: MessageWriter<LoginSuccess>,
    mut commands: Commands,
) {
    if *encryption_mode != EncryptionMode::OfflineUnencrypted {
        return; // we only handle offline-mode and unencrypted in this system
    }
    for packet in login_start_pr.read() {
        commands.entity(packet.client).insert(GameProfile {
            username: packet.name.clone(),
            uuid: packet.uuid.clone(),
        });
        login_success_pw.write(LoginSuccess {
            client: packet.client,
            game_profile: GameProfile {
                username: packet.name.clone(),
                uuid: packet.uuid.clone(),
            },
        });
    }
}
