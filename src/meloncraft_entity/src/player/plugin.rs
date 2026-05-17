use bevy::app::{App, Plugin};
use crate::player::moved::PlayerMoved;

pub struct MeloncraftPlayerPositionPlugin;

impl Plugin for MeloncraftPlayerPositionPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<PlayerMoved>();
    }
}