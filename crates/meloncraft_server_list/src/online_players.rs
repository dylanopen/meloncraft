use std::ops::{Deref, DerefMut};
use bevy::ecs::resource::Resource;

#[derive(Resource)]
pub struct OnlinePlayers(pub u32);

impl Deref for OnlinePlayers {
    type Target = u32;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for OnlinePlayers {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[expect(clippy::derivable_impls)]
impl Default for OnlinePlayers {
    fn default() -> Self {
        Self(0)
    }
}