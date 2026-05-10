use crate::encryption::EncryptionMode;
use bevy::prelude::{MessageReader, MessageWriter, Res};
use meloncraft_core::Identifier;
use meloncraft_core::datapack::DatapackMetadata;
use meloncraft_packets::clientbound::configuration::{SelectKnownPacks, StoreCookie};
use meloncraft_packets::serverbound::configuration::ClientInformation;
use meloncraft_protocol_types::Byte;

pub fn select_known_packs(
    mut login_start_pr: MessageReader<ClientInformation>,
    mut select_known_packs_cpw: MessageWriter<SelectKnownPacks>,
    mut temp_store_cookie_cpw: MessageWriter<StoreCookie>,
) {
    for packet in login_start_pr.read() {
        select_known_packs_cpw.write(SelectKnownPacks {
            client: packet.client,
            known_packs: vec![DatapackMetadata {
                namespace: "minecraft".to_string(),
                id: "core".to_string(),
                version: "1.21.10".to_string(),
            }]
        });
    }
}
