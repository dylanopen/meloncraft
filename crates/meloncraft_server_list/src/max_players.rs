use std::ops::{Deref, DerefMut};
use bevy::ecs::resource::Resource;

#[derive(Resource)]
pub struct MaxPlayers(pub u32);

impl Deref for MaxPlayers {
    type Target = u32;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for MaxPlayers {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Default for MaxPlayers {
    fn default() -> Self {
        Self(20)
    }
}