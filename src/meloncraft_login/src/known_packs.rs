use bevy::prelude::{MessageReader, MessageWriter};
use meloncraft_core::datapack::DatapackMetadata;
use meloncraft_packets::ClientboundSelectKnownPacks;
use meloncraft_packets::ServerboundClientInformation;

pub fn select_known_packs(
    mut login_start_pr: MessageReader<ServerboundClientInformation>,
    mut select_known_packs_cpw: MessageWriter<ClientboundSelectKnownPacks>,
) {
    for packet in login_start_pr.read() {
        select_known_packs_cpw.write(ClientboundSelectKnownPacks {
            client: packet.client,
            known_packs: vec![DatapackMetadata {
                namespace: "minecraft".to_string(),
                id: "core".to_string(),
                version: "1.21.10".to_string(),
            }]
        });
    }
}
