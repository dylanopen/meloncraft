pub mod marker;

use bevy::app::{App, Plugin};
use bevy::prelude::World;
use crate::marker::Overworld;

pub struct MeloncraftWorldManagerPlugin;

impl Plugin for MeloncraftWorldManagerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Overworld(World::default()));
    }
}