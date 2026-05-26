use bevy::ecs::system::Query;
use bevy::prelude::MessageReader;
use meloncraft_client::connection::ClientConnection;
use meloncraft_client::connection_state::ConnectionState;
use meloncraft_packets::ServerboundLoginAcknowledged;

pub fn login_acknowledged_listener(
    mut login_acknowledged_pr: MessageReader<ServerboundLoginAcknowledged>,
    mut client_connections: Query<&mut ClientConnection>,
) {
    for packet in login_acknowledged_pr.read() {
        client_connections.get_mut(packet.client).unwrap().state = ConnectionState::Configuration;
    }
}
