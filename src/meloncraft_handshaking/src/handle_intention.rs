use bevy::prelude::{MessageReader, MessageWriter, Query};
use meloncraft_client::connection::ClientConnection;
use meloncraft_client::connection_state::ConnectionState;
use meloncraft_client::intention_type::IntentionType;
use meloncraft_packets::ServerboundIntention;
use crate::message::{LoginHandshakeReceived, StatusHandshakeReceived, TransferHandshakeReceived};

pub fn update_connection_states(
    mut handshake_pr: MessageReader<ServerboundIntention>,
    mut client_connections: Query<&mut ClientConnection>,
    mut status_received_mw: MessageWriter<StatusHandshakeReceived>,
    mut login_received_mw: MessageWriter<LoginHandshakeReceived>,
    mut transfer_received_mw: MessageWriter<TransferHandshakeReceived>,
) {
    for packet in handshake_pr.read() {
        let entity = packet.client;

        match packet.next_state {
            IntentionType::Status => {
                status_received_mw.write(StatusHandshakeReceived);
            },
            IntentionType::Login => {
                login_received_mw.write(LoginHandshakeReceived);
            },
            IntentionType::Transfer => {
                transfer_received_mw.write(TransferHandshakeReceived);
            },
        }

        let mut client_connection = client_connections.get_mut(entity).unwrap();
        client_connection.state = match packet.next_state {
            IntentionType::Status => ConnectionState::Status,
            IntentionType::Login | IntentionType::Transfer => ConnectionState::Login,
        }
    }
}
