use crate::clientbound_packet::ClientboundPacket;
use bevy::ecs::message::Message;
use bevy::prelude::Entity;
use meloncraft_client::connection_state::ConnectionState;
use meloncraft_core::Identifier;
use meloncraft_nbt::{NbtCompound, NbtTag, NbtValue};
use crate::network_messages::ClientboundNetworkPacket;
use meloncraft_protocol_types::{PrefixedArray, ProtocolType as _};
use meloncraft_registry::RegistryEntry;

#[derive(Message, Debug, Clone)]
pub struct ClientboundRegistryData {
    pub client: Entity,
    pub registry_id: Identifier,
    pub registry_entries: Vec<RegistryEntry>,
}

impl ClientboundPacket for ClientboundRegistryData {
    fn id() -> i32 {
        return 0x07
    }

    fn state() -> ConnectionState {
        return ConnectionState::Configuration
    }


    fn client(&self) -> Entity {
        return self.client;
    }

    fn data(&self, data: &mut Vec<u8>) {
        data.extend(self.registry_id.net_serialize());
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
    }
}
