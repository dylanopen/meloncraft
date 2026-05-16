#![expect(clippy::non_std_lazy_statics, reason = "I will use LazyLock soon, but for now, lazy_static! works.")]

use crate::connection_state::ConnectionState;
use bevy::ecs::component::Component;
use lazy_static::lazy_static;
use core::net::SocketAddr;
use std::net::TcpStream;
use std::sync::Mutex;

#[derive(Component, Debug)]
pub struct ClientConnection {
    pub address: SocketAddr,
    pub tcp_stream: TcpStream,
    pub state: ConnectionState,
}

impl Clone for ClientConnection {
    fn clone(&self) -> Self {
        return ClientConnection {
            tcp_stream: self.tcp_stream.try_clone().unwrap(),
            state: self.state,
            address: self.address,
        };
    }
}

lazy_static! {
    pub static ref CLIENT_CONNECTIONS: Mutex<Vec<TcpStream>> = {
        let client_connections = Vec::new();
        Mutex::new(client_connections)
    };
}
