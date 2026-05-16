pub mod marker;
mod sender;

use bevy::app::{App, Plugin, Update};
use meloncraft_world::world::World;
use crate::marker::Overworld;

pub struct MeloncraftWorldManagerPlugin;

impl Plugin for MeloncraftWorldManagerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Overworld(World::default()));

        app.add_systems(Update, sender::send_chunk);
    }
}