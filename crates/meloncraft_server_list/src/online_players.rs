use bevy::ecs::resource::Resource;
use core::ops::{Deref, DerefMut};

#[derive(Resource)]
pub struct OnlinePlayers(pub u32);

impl Deref for OnlinePlayers {
    type Target = u32;
    fn deref(&self) -> &Self::Target {
        return &self.0;
    }
}

impl DerefMut for OnlinePlayers {
    fn deref_mut(&mut self) -> &mut Self::Target {
        return &mut self.0;
    }
}

impl Default for OnlinePlayers {
    fn default() -> Self {
        return Self(0);
    }
}
