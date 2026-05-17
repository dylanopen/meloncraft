use std::net::TcpStream;
use std::sync::Mutex;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref CLIENT_CONNECTIONS: Mutex<Vec<TcpStream>> = {
        let client_connections = Vec::new();
        Mutex::new(client_connections)
    };
}
