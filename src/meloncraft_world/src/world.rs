use std::collections::HashMap;
use bevy::prelude::Resource;
use meloncraft_chunk::Chunk;

#[derive(Resource)]
pub struct World {
    chunks: HashMap<(i32, i32), Chunk>,
}