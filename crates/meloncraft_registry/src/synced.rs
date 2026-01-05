use crate::RegistryEntry;
use meloncraft_core::Identifier;

pub struct SyncedRegistry {
    pub registry_id: Identifier,
    pub entries: Vec<RegistryEntry>,
}
