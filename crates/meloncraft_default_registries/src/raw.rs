use std::fs;

use bevy::ecs::resource::Resource;
use meloncraft_nbt::{NbtCompound, NbtTag, NbtValue};
use meloncraft_protocol_types::ProtocolBuffer;

#[derive(Resource, Debug)]
pub struct RawRegistries(pub NbtCompound);

impl Default for RawRegistries {
    fn default() -> Self {
        let root: NbtTag = fs::read("generated/registries.nbt")
            .expect("failed to read generated/registries.nbt")
            .net_deserialize()
            .expect("failed to deserialize generated/registries.nbt");
        if let NbtValue::Compound(compound) = root.value {
            println!("Loaded registries.nbt successfully.");
            dbg!(&compound.keys());
            Self(compound)
        } else {
            panic!("registries.nbt does not contain a compound at the root!");
        }
    }
}
