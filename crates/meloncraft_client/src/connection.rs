use std::net::SocketAddr;
use bevy::ecs::component::Component;
use crate::connection_state::ConnectionState;

#[derive(Component, Clone, Debug)]
pub struct ClientConnection {
    pub address: SocketAddr,
    pub state: ConnectionState,
}