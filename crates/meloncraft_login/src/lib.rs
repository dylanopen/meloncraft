mod encryption;
mod listener;
mod verify;

pub use encryption::EncryptionMode;

use crate::listener::login_offline_unencrypted_listener;
use crate::verify::verify_encryption;
use bevy::app::{App, Plugin, PostStartup, Update};

pub struct MeloncraftLoginPlugin;

impl Plugin for MeloncraftLoginPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, verify_encryption);
        app.add_systems(Update, login_offline_unencrypted_listener);
    }
}
