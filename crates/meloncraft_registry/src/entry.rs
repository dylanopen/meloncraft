use meloncraft_core::Identifier;
use meloncraft_nbt::tag::NbtTag;

pub struct RegistryEntry {
    pub id: Identifier,
    pub data: Option<NbtTag>,
}
