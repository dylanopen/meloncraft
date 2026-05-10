use bevy::ecs::entity::Entity;
use bevy::ecs::message::{MessageReader, MessageWriter};
use bevy::ecs::system::Res;
use meloncraft_core::Identifier;
use meloncraft_default_registries::raw::RawRegistries;
use meloncraft_nbt::{NbtCompound, NbtValue};
use meloncraft_packets::clientbound::configuration::RegistryData;
use meloncraft_packets::clientbound_packet::ClientboundPacket;
use meloncraft_packets::serverbound::login::LoginAcknowledged;
use meloncraft_registry::RegistryEntry;
use meloncraft_network::packet::{ClientboundNetworkPacket, ClientboundNetworkPacketReceived};

pub fn send_registry_data(
    mut login_acknowledged_pr: MessageReader<LoginAcknowledged>,
    mut network_pw: MessageWriter<ClientboundNetworkPacketReceived>,
    raw_registries: Res<RawRegistries>,
) {
    for packet in login_acknowledged_pr.read() {
        network_pw.write(send_raw_registry(packet.client, vec![0x18,0x6D,0x69,0x6E,0x65,0x63,0x72,0x61,0x66,0x74,0x3A,0x64,0x69,0x6D,0x65,0x6E,0x73,0x69,0x6F,0x6E,0x5F,0x74,0x79,0x70,0x65,0x04,0x13,0x6D,0x69,0x6E,0x65,0x63,0x72,0x61,0x66,0x74,0x3A,0x6F,0x76,0x65,0x72,0x77,0x6F,0x72,0x6C,0x64,0x00,0x19,0x6D,0x69,0x6E,0x65,0x63,0x72,0x61,0x66,0x74,0x3A,0x6F,0x76,0x65,0x72,0x77,0x6F,0x72,0x6C,0x64,0x5F,0x63,0x61,0x76,0x65,0x73,0x00,0x11,0x6D,0x69,0x6E,0x65,0x63,0x72,0x61,0x66,0x74,0x3A,0x74,0x68,0x65,0x5F,0x65,0x6E,0x64,0x00,0x14,0x6D,0x69,0x6E,0x65,0x63,0x72,0x61,0x66,0x74,0x3A,0x74,0x68,0x65,0x5F,0x6E,0x65,0x74,0x68,0x65,0x72,0x00]));
    }
}

fn send_raw_registry(client: Entity, packet_data: Vec<u8>) -> ClientboundNetworkPacketReceived {
    ClientboundNetworkPacketReceived { packet: ClientboundNetworkPacket {
        client,
        id: RegistryData::id(),
        data: packet_data,
    }}
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
