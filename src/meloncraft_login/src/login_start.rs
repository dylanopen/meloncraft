use crate::encryption::EncryptionMode;
use bevy::prelude::{MessageReader, MessageWriter, Res};
use meloncraft_packets::ServerboundLoginStart;
use meloncraft_player::GameProfile;
use crate::messages::OfflineLoggedIn;

pub fn login_offline_unencrypted_fwd(
    mut login_start_pr: MessageReader<ServerboundLoginStart>,
    encryption_mode: Res<EncryptionMode>,
    mut offline_logged_in_mw: MessageWriter<OfflineLoggedIn>,
) {
    if *encryption_mode != EncryptionMode::OfflineUnencrypted {
        return; // we only handle offline-mode and unencrypted in this system
    }
    for packet in login_start_pr.read() {
        offline_logged_in_mw.write(OfflineLoggedIn {
            client: packet.client,
            profile: GameProfile {
                username: packet.name.clone(),
                uuid: packet.uuid.clone(),
            },
        });
    }
}
