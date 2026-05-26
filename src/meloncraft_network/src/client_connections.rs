use lazy_static::lazy_static;
use std::net::TcpStream;
use std::sync::Mutex;

lazy_static! {
    pub static ref CLIENT_CONNECTIONS: Mutex<Vec<TcpStream>> = {
        let client_connections = Vec::new();
        Mutex::new(client_connections)
    };
}
