use bevy::ecs::resource::Resource;
use std::ops::{Deref, DerefMut};

#[derive(Resource)]
pub struct Motd(pub String);

impl Deref for Motd {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Motd {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
