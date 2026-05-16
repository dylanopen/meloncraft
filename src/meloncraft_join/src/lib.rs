mod state;
mod login_start;

use bevy::app::{App, Plugin, Update};
use meloncraft_handshaking::MeloncraftHandshakingPlugin;

pub struct MeloncraftJoinPlugin;

impl Plugin for MeloncraftJoinPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MeloncraftHandshakingPlugin);

        app.add_systems(Update, (
            login_start::save_profile,
            login_start::respond_success,
        ));
    }
}