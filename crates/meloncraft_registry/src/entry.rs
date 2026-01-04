use meloncraft_core::Identifier;
use meloncraft_nbt::NbtTag;

#[derive(Debug, Clone)]
pub struct RegistryEntry {
    pub id: Identifier,
    pub data: Option<NbtTag>,
}
