mod client_information;
mod encryption;
mod login_acknowledged;
mod login_start;
mod verify;
mod registry_data;

pub use encryption::EncryptionMode;

use crate::client_information::client_information_listener;
use crate::login_acknowledged::login_acknowledged_listener;
use crate::login_start::login_offline_unencrypted_listener;
use crate::verify::verify_encryption;
use bevy::app::{App, Plugin, PostStartup, Update};
use bevy::prelude::IntoScheduleConfigs;

use self::registry_data::send_registry_data;

pub struct MeloncraftLoginPlugin;

impl Plugin for MeloncraftLoginPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, verify_encryption);
        app.add_systems(Update, login_offline_unencrypted_listener);
        app.add_systems(
            Update,
            (login_acknowledged_listener, client_information_listener).chain(),
        );
        app.add_systems(Update, send_registry_data);
    }
}
