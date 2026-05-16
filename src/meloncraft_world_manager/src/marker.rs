use bevy::prelude::Resource;
use meloncraft_world::world::World;

#[derive(Resource)]
pub struct Overworld(pub World);