mod client_information;
mod encryption;
mod finish;
mod known_packs;
mod login_acknowledged;
mod login_start;
pub mod messages;
mod registry_data;
mod verify;

pub use encryption::EncryptionMode;

use crate::client_information::client_information_listener;
use crate::login_acknowledged::login_acknowledged_listener;
use crate::login_start::login_offline_unencrypted_fwd;
use crate::verify::verify_encryption;
use bevy::app::{App, Plugin, PostStartup, Update};
use bevy::prelude::IntoScheduleConfigs;

use self::finish::finish_configuration;
use self::known_packs::select_known_packs;
use self::registry_data::send_registry_data;

pub struct MeloncraftLoginPlugin;

impl Plugin for MeloncraftLoginPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, verify_encryption);
        app.add_systems(Update, login_offline_unencrypted_fwd);
        app.add_systems(
            Update,
            (login_acknowledged_listener, client_information_listener).chain(),
        );
        app.add_systems(
            Update,
            (select_known_packs, send_registry_data, finish_configuration).chain(),
        );

        app.add_message::<messages::OfflineLoggedIn>();
        app.add_message::<messages::ClientLocaleReceived>();
        app.add_message::<messages::ClientViewDistanceReceived>();
        app.add_message::<messages::ClientChatModeReceived>();
        app.add_message::<messages::ClientChatColorsReceived>();
        app.add_message::<messages::ClientDisplayedSkinPartsReceived>();
        app.add_message::<messages::ClientMainHandReceived>();
        app.add_message::<messages::ClientEnableTextFilteringReceived>();
        app.add_message::<messages::ClientAllowPlayerListingsReceived>();
        app.add_message::<messages::ClientParticleRenderingModeReceived>();
    }
}
