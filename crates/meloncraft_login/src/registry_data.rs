use bevy::ecs::entity::Entity;
use bevy::ecs::message::{MessageReader, MessageWriter};
use bevy::ecs::system::Res;
use meloncraft_core::Identifier;
use meloncraft_default_registries::raw::RawRegistries;
use meloncraft_nbt::NbtValue;
use meloncraft_packets::clientbound::configuration::RegistryData;
use meloncraft_packets::serverbound::login::LoginAcknowledged;
use meloncraft_registry::RegistryEntry;

pub fn send_registry_data(
    mut login_acknowledged_pr: MessageReader<LoginAcknowledged>,
    mut registry_data_pw: MessageWriter<RegistryData>,
    raw_registries: Res<RawRegistries>,
) {
    for packet in login_acknowledged_pr.read() {
        dbg!("Sending registry data");
        registry_data_pw.write(send_registry(packet.client, "damage_type", &raw_registries));
        registry_data_pw.write(send_registry(packet.client, "dimension_type", &raw_registries));
        registry_data_pw.write(send_registry(packet.client, "painting_variant", &raw_registries));
        registry_data_pw.write(send_registry(packet.client, "banner_pattern", &raw_registries));
        registry_data_pw.write(send_registry(packet.client, "worldgen/biome", &raw_registries));
        dbg!("Sent registry data");
    }
}

fn send_registry(
    client: Entity,
    registry_name: &str,
    raw_registries: &RawRegistries
) -> RegistryData {
        let Some(NbtValue::Compound(raw_registry)) = raw_registries.0.get_value(registry_name) else {
            panic!("Missing {registry_name} registry or it is in an invalid format");
        };
        let mut entries = Vec::new();
        for raw_entry in raw_registry.iter() {
            let entry = RegistryEntry {
                id: Identifier(raw_entry.key.clone()),
                data: Some(raw_entry.value.clone()),
            };
            entries.push(entry);
        }
        RegistryData {
            client,
            registry_id: Identifier(format!("minecraft:{registry_name}")),
            registry_entries: entries,
        }
}
