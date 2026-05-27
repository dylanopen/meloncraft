use std::net::TcpListener;

use bevy::ecs::resource::Resource;
use bevy::ecs::system::Res;
use meloncraft_logger::{debuglog, errorlog};

#[derive(Resource, Debug)]
pub struct NewClientListener(pub TcpListener);

pub fn handle_new_clients(listener: Res<NewClientListener>) {
    for stream in listener.0.incoming() {
        let Ok(stream) = stream else {
            errorlog!(
                "Failed to get incoming TcpStream from client, ignoring connection. Error: {}",
                stream.unwrap_err()
            );
            continue;
        };
        debuglog!(
            "TCP connection established with client IP {}",
            stream.peer_addr().unwrap()
        );
    }
}
