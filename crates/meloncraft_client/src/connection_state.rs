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
