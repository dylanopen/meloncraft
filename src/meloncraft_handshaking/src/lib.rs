mod handle_intention;

use bevy::app::Plugin;
use bevy::app::{App, Update};

pub struct MeloncraftHandshakingPlugin;

impl Plugin for MeloncraftHandshakingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_intention::update_connection_states);
    }
}
