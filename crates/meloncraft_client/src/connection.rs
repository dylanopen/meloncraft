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
#[derive(Component, Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct ClientConnectionId(pub usize);

impl ClientConnectionId {
    pub fn get(self) -> ClientConnection {
        CLIENT_CONNECTIONS.lock().unwrap()[self.0].clone()
    }
}

lazy_static! {
    pub static ref CLIENT_CONNECTIONS: Mutex<Vec<ClientConnection>> = {
        let client_connections = Vec::new();
        Mutex::new(client_connections)
    };
}
