use bevy::prelude::{MessageReader, Query};
use meloncraft_client::connection::ClientConnection;
use meloncraft_client::connection_state::ConnectionState;
use meloncraft_handshaking::handshaken::{LoginHandshaken, StatusHandshaken, TransferHandshaken};

pub fn update_connection_state_status(
    mut status_handshaken_mr: MessageReader<StatusHandshaken>,
    mut client_connections: Query<&mut ClientConnection>,
) {
    for handshake in status_handshaken_mr.read() {
        let mut client_connection = client_connections.get_mut(handshake.player).unwrap();
        client_connection.state = ConnectionState::Status;
    }
}

pub fn update_connection_state_login(
    mut status_handshaken_mr: MessageReader<LoginHandshaken>,
    mut client_connections: Query<&mut ClientConnection>,
) {
    for handshake in status_handshaken_mr.read() {
        let mut client_connection = client_connections.get_mut(handshake.player).unwrap();
        client_connection.state = ConnectionState::Login;
    }
}

pub fn update_connection_state_transfer(
    mut status_handshaken_mr: MessageReader<TransferHandshaken>,
    mut client_connections: Query<&mut ClientConnection>,
) {
    for handshake in status_handshaken_mr.read() {
        // TODO: check whether server transfers are blocked. If they are, send a disconnect packet and don't update the state.
        let mut client_connection = client_connections.get_mut(handshake.player).unwrap();
        client_connection.state = ConnectionState::Login;
    }
}
