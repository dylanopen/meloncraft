use bevy::prelude::{MessageReader, Query};
use meloncraft_client::connection::ClientConnection;
use meloncraft_packets::incoming::handshaking::Handshake;

pub fn update_connection_states(
    mut handshake_pr: MessageReader<Handshake>,
    mut client_connections: Query<&mut ClientConnection>,
) {
    for packet in handshake_pr.read() {
        let entity = packet.client;
        let mut client_connection = client_connections.get_mut(entity).unwrap();
        client_connection.state = packet.next_state;
    }
}