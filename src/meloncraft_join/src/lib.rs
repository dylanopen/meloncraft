mod state;

use bevy::app::{App, Plugin};
use meloncraft_handshaking::MeloncraftHandshakingPlugin;

pub struct MeloncraftJoinPlugin;

impl Plugin for MeloncraftJoinPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MeloncraftHandshakingPlugin);
    }
}