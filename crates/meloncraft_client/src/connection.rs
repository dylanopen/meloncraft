use crate::connection_state::ConnectionState;
use bevy::ecs::component::Component;
use lazy_static::lazy_static;
use std::net::{SocketAddr, TcpStream};
use std::sync::Mutex;

#[derive(Component, Debug)]
pub struct ClientConnection {
    pub address: SocketAddr,
    pub tcp_stream: TcpStream,
    pub state: ConnectionState,
}

impl Clone for ClientConnection {
    fn clone(&self) -> ClientConnection {
        ClientConnection {
            tcp_stream: self.tcp_stream.try_clone().unwrap(),
            state: self.state,
            address: self.address,
        }
    }
}
lazy_static! {
    pub static ref CLIENT_CONNECTIONS: Mutex<Vec<TcpStream>> = {
        let client_connections = Vec::new();
        Mutex::new(client_connections)
    };
}
