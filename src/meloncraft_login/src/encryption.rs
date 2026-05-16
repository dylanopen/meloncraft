use bevy::prelude::Resource;

#[derive(Resource, Copy, Clone, Debug, PartialOrd, PartialEq, Ord, Eq)]
pub enum EncryptionMode {
    OfflineUnencrypted,
    OfflineEncrypted,
    MojangEncrypted,
}
