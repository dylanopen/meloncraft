use bevy::app::{App, Plugin, Update};
use crate::movement;

pub struct MeloncraftPlayerStatePlugin;

impl Plugin for MeloncraftPlayerStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, movement::save_new_location);
    }
}