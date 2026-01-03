use crate::encryption::EncryptionMode;
use bevy::prelude::Res;

pub fn verify_encryption(encryption_mode: Res<EncryptionMode>) {
    match *encryption_mode {
        EncryptionMode::OfflineUnencrypted => {}
        EncryptionMode::OfflineEncrypted => {
            panic!("Encryption is not yet supported - consider choosing a different EncryptionMode")
        }
        EncryptionMode::MojangEncrypted => {
            panic!(
                "Encryption and Mojang authentication are not yet supported - consider choosing a different EncryptionMode"
            )
        }
    }
}
