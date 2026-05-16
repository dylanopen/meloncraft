use bevy::prelude::Resource;
use std::net::TcpListener;
use core::ops::{Deref, DerefMut};

#[derive(Resource)]
pub struct ConnectionListener(pub TcpListener);

impl Deref for ConnectionListener {
    type Target = TcpListener;
    fn deref(&self) -> &Self::Target {
        return &self.0;
    }
}

impl DerefMut for ConnectionListener {
    fn deref_mut(&mut self) -> &mut Self::Target {
        return &mut self.0;
    }
}
