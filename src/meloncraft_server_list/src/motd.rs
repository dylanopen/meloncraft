use bevy::ecs::resource::Resource;
use core::ops::{Deref, DerefMut};

#[derive(Resource)]
pub struct Motd(pub String);

impl Deref for Motd {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        return &self.0;
    }
}

impl DerefMut for Motd {
    fn deref_mut(&mut self) -> &mut Self::Target {
        return &mut self.0;
    }
}
