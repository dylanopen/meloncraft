use meloncraft_nbt::tag::NbtTag;
use meloncraft_protocol_types::Identifier;

pub struct RegistryEntry {
    pub id: Identifier,
    pub data: Option<NbtTag>,
}
