use bevy::prelude::{MessageReader, MessageWriter};
use meloncraft_core::datapack::DatapackMetadata;
use meloncraft_packets::clientbound::configuration::{FinishConfiguration, SelectKnownPacks};
use meloncraft_packets::serverbound::configuration::ClientInformation;

pub fn finish_configuration(
    mut login_start_pr: MessageReader<ClientInformation>,
    mut finish_configuration_cpw: MessageWriter<FinishConfiguration>,
) {
    for packet in login_start_pr.read() {
        finish_configuration_cpw.write(FinishConfiguration {
            client: packet.client
        });
    }
}
