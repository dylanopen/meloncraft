use bevy::app::{App, Plugin};
use crate::player::plugin::MeloncraftPlayerPositionPlugin;

pub struct MeloncraftEntityPlugin;

impl Plugin for MeloncraftEntityPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MeloncraftPlayerPositionPlugin);
    }
}