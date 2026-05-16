use bevy::ecs::resource::Resource;
use core::ops::{Deref, DerefMut};

#[derive(Resource)]
pub struct MaxPlayers(pub u32);

impl Deref for MaxPlayers {
    type Target = u32;
    fn deref(&self) -> &Self::Target {
        return &self.0;
    }
}

impl DerefMut for MaxPlayers {
    fn deref_mut(&mut self) -> &mut Self::Target {
        return &mut self.0;
    }
}

impl Default for MaxPlayers {
    fn default() -> Self {
        return Self(20);
    }
}
