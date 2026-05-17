use bevy::app::{App, Plugin};
use crate::player::messages;

pub struct MeloncraftPlayerPositionPlugin;

impl Plugin for MeloncraftPlayerPositionPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<messages::PlayerMoved>();
    }
}