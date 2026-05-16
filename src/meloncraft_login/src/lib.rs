mod client_information;
mod encryption;
mod login_acknowledged;
mod login_start;
mod known_packs;
mod registry_data;
mod verify;
mod finish;
pub mod messages;

pub use encryption::EncryptionMode;

use crate::client_information::client_information_listener;
use crate::login_acknowledged::login_acknowledged_listener;
use crate::login_start::login_offline_unencrypted_listener;
use crate::verify::verify_encryption;
use bevy::app::{App, Plugin, PostStartup, Update};
use bevy::prelude::IntoScheduleConfigs;

use self::registry_data::send_registry_data;
use self::known_packs::select_known_packs;
use self::finish::finish_configuration;

pub struct MeloncraftLoginPlugin;

impl Plugin for MeloncraftLoginPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, verify_encryption);
        app.add_systems(Update, login_offline_unencrypted_listener);
        app.add_systems(
            Update,
            (login_acknowledged_listener, client_information_listener).chain(),
        );
        app.add_systems(Update, (select_known_packs, send_registry_data, finish_configuration).chain());
        
        app.add_message::<messages::OfflineLoggedIn>();
    }
}
