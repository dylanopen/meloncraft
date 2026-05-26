//! Module for resource struct [`ServerIcon`].

use core::ops::{Deref, DerefMut};

use bevy::ecs::resource::Resource;

/// Resource for the server's icon.
/// This stores a byte array, representing the *PNG encoding* of the server icon.
#[derive(Resource, Debug, Clone)]
pub struct ServerIcon(pub Vec<u8>);

impl Deref for ServerIcon {
    type Target = Vec<u8>;
    fn deref(&self) -> &Self::Target {
        return &self.0;
    }
}

impl DerefMut for ServerIcon {
    fn deref_mut(&mut self) -> &mut Self::Target {
        return &mut self.0;
    }
}
