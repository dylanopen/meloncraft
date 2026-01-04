use meloncraft_core::Identifier;

pub struct ItemTag {
    pub name: Identifier,
    pub entries: Vec<i32>,
}

pub struct RegistryTags {
    pub registry_name: Identifier,
    pub tags: Vec<ItemTag>,
}
