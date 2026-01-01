use crate::connection_state::ConnectionState;
use bevy::ecs::component::Component;
use std::net::{SocketAddr, TcpStream};

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
