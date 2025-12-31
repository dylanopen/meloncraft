use bevy::ecs::resource::Resource;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::ops::{Deref, DerefMut};


#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ConnectionState {
    Handshaking,
    Status,
    Login,
    Play,
}


#[derive(Resource)]
pub struct ConnectionStates(pub HashMap<SocketAddr, ConnectionState>);

impl Deref for ConnectionStates {
    type Target = HashMap<SocketAddr, ConnectionState>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ConnectionStates {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

