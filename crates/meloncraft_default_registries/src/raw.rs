use std::fs;

use bevy::ecs::resource::Resource;
use meloncraft_nbt::{NbtCompound, NbtValue};
use meloncraft_protocol_types::ProtocolBuffer;

#[derive(Resource, Debug)]
pub struct RawRegistries(pub NbtCompound);

impl Default for RawRegistries {
    fn default() -> Self {
        if let NbtValue::Compound(compound) = fs::read("generated/registries.nbt")
            .expect("failed to read generated/registries.nbt - have you run `cargo run --package meloncraft_generator` yet?")
            .net_deserialize()
            .expect("failed to deserialize generated/registries.nbt")
        {
            Self(compound)
        } else {
            panic!("registries.nbt does not contain a compound at the root!");
        }
    }
}
