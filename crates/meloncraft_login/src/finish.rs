use bevy::prelude::{MessageReader, MessageWriter};
use meloncraft_packets::ClientboundFinishConfiguration;
use meloncraft_packets::ServerboundSelectKnownPacks;

pub fn finish_configuration(
    mut login_start_pr: MessageReader<ServerboundSelectKnownPacks>,
    mut finish_configuration_pw: MessageWriter<ClientboundFinishConfiguration>,
) {
    for packet in login_start_pr.read() {
        finish_configuration_pw.write(ClientboundFinishConfiguration {
            client: packet.client
        });
    }
}
