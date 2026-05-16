use std::fs;

use bevy::ecs::resource::Resource;
use meloncraft_nbt::NbtCompound;
use meloncraft_protocol_types::ProtocolBuffer as _;

#[derive(Resource, Debug)]
pub struct RawRegistries(pub NbtCompound);

impl Default for RawRegistries {
    fn default() -> Self {
        let mut raw_registries = NbtCompound(Vec::new());
        for registry in fs::read_dir("generated/registries").unwrap() {
            let entry = registry.unwrap();
            let path = entry.path();
            if path.extension().and_then(|s| return s.to_str()) == Some("nbt") {
                let mut nbt_bytes = fs::read(&path).unwrap();
                raw_registries.push(nbt_bytes.net_deserialize().unwrap());
            }
        }
        return Self(raw_registries);
    }
}
