use bevy::prelude::{MessageReader, Query};
use meloncraft_client::connection::{CLIENT_CONNECTIONS, ClientConnection, ClientConnectionId};
use meloncraft_packets::incoming::handshaking::Intention;

pub fn update_connection_states(
    mut handshake_pr: MessageReader<Intention>,
    mut client_connections: Query<&ClientConnectionId>,
) {
    for packet in handshake_pr.read() {
        let entity = packet.client;
        let mut client_connection = client_connections.get(entity).unwrap();
        dbg!(packet.next_state);
        CLIENT_CONNECTIONS.lock().unwrap()[client_connection.0].state = packet.next_state;
    }
}
