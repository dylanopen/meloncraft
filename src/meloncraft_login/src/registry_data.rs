use bevy::ecs::entity::Entity;
use bevy::ecs::message::{MessageReader, MessageWriter};
use meloncraft_packets::ClientboundRegistryData;
use meloncraft_packets::ServerboundSelectKnownPacks;
use meloncraft_packets::clientbound_packet::ClientboundPacket;
use meloncraft_packets::network_messages::{
    ClientboundNetworkPacket, ClientboundNetworkPacketReceived,
};

pub fn send_registry_data(
    mut select_known_packs_spr: MessageReader<ServerboundSelectKnownPacks>,
    mut network_pw: MessageWriter<ClientboundNetworkPacketReceived>,
) {
    for packet in select_known_packs_spr.read() {
        // This really could be the most cursed code I've ever written, but it works for now and I don't care.

        let registry_packets = vec![
            include_bytes!("../../meloncraft_login/registry_nbt/banner_pattern.nbt").to_vec(),
            include_bytes!("../../meloncraft_login/registry_nbt/cat_variant.nbt").to_vec(),
            include_bytes!("../../meloncraft_login/registry_nbt/chat_type.nbt").to_vec(),
            include_bytes!("../../meloncraft_login/registry_nbt/chicken_variant.nbt").to_vec(),
            include_bytes!("../../meloncraft_login/registry_nbt/cow_variant.nbt").to_vec(),
            include_bytes!("../../meloncraft_login/registry_nbt/damage_type.nbt").to_vec(),
            include_bytes!("../../meloncraft_login/registry_nbt/dialog.nbt").to_vec(),
            include_bytes!("../../meloncraft_login/registry_nbt/dimension_type.nbt").to_vec(),
            include_bytes!("../../meloncraft_login/registry_nbt/enchantment.nbt").to_vec(),
            include_bytes!("../../meloncraft_login/registry_nbt/frog_variant.nbt").to_vec(),
            include_bytes!("../../meloncraft_login/registry_nbt/instrument.nbt").to_vec(),
            include_bytes!("../../meloncraft_login/registry_nbt/jukebox_song.nbt").to_vec(),
            include_bytes!("../../meloncraft_login/registry_nbt/painting_variant.nbt").to_vec(),
            include_bytes!("../../meloncraft_login/registry_nbt/pig_variant.nbt").to_vec(),
            include_bytes!("../../meloncraft_login/registry_nbt/test_environment.nbt").to_vec(),
            include_bytes!("../../meloncraft_login/registry_nbt/test_instance.nbt").to_vec(),
            include_bytes!("../../meloncraft_login/registry_nbt/trim_material.nbt").to_vec(),
            include_bytes!("../../meloncraft_login/registry_nbt/trim_pattern.nbt").to_vec(),
            include_bytes!("../../meloncraft_login/registry_nbt/wolf_sound_variant.nbt").to_vec(),
            include_bytes!("../../meloncraft_login/registry_nbt/wolf_variant.nbt").to_vec(),
            include_bytes!("../../meloncraft_login/registry_nbt/worldgen.biome.nbt").to_vec(),
        ];

        for packet_data in registry_packets {
            network_pw.write(send_raw_registry(packet.client, packet_data));
        }

        // TODO: send update tags packet, data is in the registry_nbt folder.
        let mut update_tags_data =
            include_bytes!("../../meloncraft_login/registry_nbt/update_tags.nbt").to_vec();
        update_tags_data.remove(0); // Remove packet ID byte
        network_pw.write(ClientboundNetworkPacketReceived {
            packet: ClientboundNetworkPacket {
                client: packet.client,
                id: 0x0D,
                data: update_tags_data,
            },
        });
    }
}

fn send_raw_registry(client: Entity, packet_data: Vec<u8>) -> ClientboundNetworkPacketReceived {
    let mut data = packet_data.clone();
    data.remove(0); // Remove the packet ID byte
    ClientboundNetworkPacketReceived {
        packet: ClientboundNetworkPacket {
            client,
            id: ClientboundRegistryData::id(),
            data,
        },
    }
}

/*fn send_registry(
    client: Entity,
    registry_name: &str,
    raw_registries: &RawRegistries,
) -> RegistryData {
    let Some(NbtValue::Compound(raw_registry)) = raw_registries.0.get_value(registry_name) else {
        panic!("Missing {registry_name} registry or it is in an invalid format");
    };
    let mut entries = Vec::new();
    for raw_entry in raw_registry.iter() {
        let entry = RegistryEntry {
            id: Identifier(raw_entry.key.clone()),
            data: Some(NbtValue::Compound(NbtCompound(Vec::new())))
        };
        entries.push(entry);
    }
    RegistryData {
        client,
        registry_id: Identifier(format!("minecraft:{registry_name}")),
        registry_entries: entries,
    }
}*/
