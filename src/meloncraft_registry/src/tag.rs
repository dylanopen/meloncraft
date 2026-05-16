use meloncraft_core::Identifier;

#[derive(Debug, Clone)]
pub struct ItemTag {
    pub name: Identifier,
    pub entries: Vec<i32>,
}

#[derive(Debug, Clone)]
pub struct RegistryTags {
    pub registry_name: Identifier,
    pub tags: Vec<ItemTag>,
}
