use crate::clientbound_packet::ClientboundPacket;
use bevy::ecs::message::Message;
use bevy::prelude::Entity;
use meloncraft_client::connection_state::ConnectionState;
use meloncraft_core::Identifier;
use meloncraft_nbt::{NbtCompound, NbtTag, NbtValue};
use meloncraft_network::packet::ClientboundNetworkPacket;
use meloncraft_protocol_types::{PrefixedArray, ProtocolType};
use meloncraft_registry::RegistryEntry;
use std::fs;

#[derive(Message, Debug, Clone)]
pub struct RegistryData {
    pub client: Entity,
    pub registry_id: Identifier,
    pub registry_entries: Vec<RegistryEntry>,
}

impl ClientboundPacket for RegistryData {
    fn id() -> i32 {
        0x07
    }

    fn state() -> ConnectionState {
        ConnectionState::Configuration
    }

    fn serialize(&self) -> Option<ClientboundNetworkPacket> {
        /*let mut data = self.registry_id.net_serialize();
        data.extend(PrefixedArray(self.registry_entries.clone()).net_serialize());
        let regnbt = format!("{:#?}", self.registry_entries);
        fs::write("registry_data_debug_nbt.txt", regnbt).unwrap();
        let reghex = format!("{:x?}", data);
        fs::write("registry_data_debug_hex.txt", reghex).unwrap();
        println!("SERIALISED REGISTRY DATA: {:x?}", &data);
        println!("Deserialised NBT: {:#?}", PrefixedArray::<RegistryEntry>::net_deserialize(&mut data.clone()).unwrap());
        Some(ClientboundNetworkPacket {
            client: self.client,
            id: Self::id(),
            data,
        })*/
        let mut data = self.registry_id.net_serialize();
        let mut entries_data = Vec::new();
        for entry in &self.registry_entries {
            let entry_compound = NbtTag {
                key: entry.id.0.clone(),
                // empty value
                value: NbtValue::Compound(NbtCompound(Vec::new())),
            };
            entries_data.extend(entry_compound.net_serialize());
        }
        data.extend(PrefixedArray(entries_data).net_serialize());
        println!("SERIALISED REGISTRY DATA: {:x?}", &data);
        Some(ClientboundNetworkPacket {
            client: self.client,
            id: Self::id(),
            data,
        })
    }
}
