use std::net::TcpListener;

use bevy::ecs::resource::Resource;
use bevy::ecs::system::{Commands, Res};
use meloncraft_client::connection::ClientConnection;
use meloncraft_client::connection_state::ConnectionState;
use meloncraft_logger::debuglog;

#[derive(Resource, Debug)]
pub struct NewClientListener(pub TcpListener);

pub fn handle_new_clients(mut commands: Commands, listener: Res<NewClientListener>) {
    while let Ok((tcp_stream, address)) = listener.0.accept() {
        debuglog!(
            "TCP connection established with client IP {}",
            address.to_string()
        );
        commands.spawn(ClientConnection {
            state: ConnectionState::Handshaking,
            address,
            tcp_stream,
        });
    }
}
