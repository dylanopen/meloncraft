use bevy::prelude::{MessageReader, MessageWriter};
use meloncraft_packets::clientbound::configuration::FinishConfiguration;
use meloncraft_packets::serverbound::configuration::SelectKnownPacks;

pub fn finish_configuration(
    mut login_start_pr: MessageReader<SelectKnownPacks>,
    mut finish_configuration_pw: MessageWriter<FinishConfiguration>,
) {
    for packet in login_start_pr.read() {
        finish_configuration_pw.write(FinishConfiguration {
            client: packet.client
        });
    }
}
