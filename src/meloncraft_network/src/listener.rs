use core::time::Duration;
use std::net::TcpListener;

use bevy::ecs::resource::Resource;
use bevy::ecs::system::{Commands, Res};
use meloncraft_client::connection::ClientConnection;
use meloncraft_client::connection_state::ConnectionState;
use meloncraft_logger::{errorlog, tracelog};

#[derive(Resource, Debug)]
pub struct NewClientListener(pub TcpListener);

pub fn handle_new_clients(mut commands: Commands, listener: Res<NewClientListener>) {
    while let Ok((tcp_stream, address)) = listener.0.accept() {
        if let Err(err) = tcp_stream.set_nonblocking(true) {
            errorlog!("Failed to set TcpStream to non-blocking for client {address}. Error: {err}");
            continue;
        }
        if let Err(err) = tcp_stream.set_read_timeout(Some(Duration::from_secs(15))) {
            errorlog!(
                "Failed to set TcpStream's read timeout to 15 seconds for client {address}. Error: {err}"
            );
            continue;
        }
        tracelog!("TCP connection established with client IP {address}");
        commands.spawn(ClientConnection {
            state: ConnectionState::Handshaking,
            address,
            tcp_stream,
            serverbound_packets_processed: 0,
        });
    }
}
