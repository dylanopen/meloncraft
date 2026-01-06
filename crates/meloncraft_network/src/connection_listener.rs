use bevy::prelude::Resource;
use std::net::TcpListener;
use std::ops::{Deref, DerefMut};

#[derive(Resource)]
pub struct ConnectionListener(pub TcpListener);

impl Deref for ConnectionListener {
    type Target = TcpListener;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ConnectionListener {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
