use crate::SERVERBOUND_PACKETS;
use crate::packet::ServerboundNetworkPacketReceived;
use crate::tcp_reader::handle_client;
use bevy::prelude::{Commands, MessageWriter, Query};
use meloncraft_client::connection::{CLIENT_CONNECTIONS, ClientConnection};
use meloncraft_client::connection_state::ConnectionState;
use std::thread;

pub fn connection_manager(
    mut commands: Commands,
    client_connection_ecs: Query<&ClientConnection>,
    mut incoming_packet_mw: MessageWriter<ServerboundNetworkPacketReceived>,
) {
    if client_connection_ecs.iter().len() < CLIENT_CONNECTIONS.lock().unwrap().len() {
        let client_connections = CLIENT_CONNECTIONS.lock().unwrap();
        for client_connection in client_connections.iter() {
            if !client_connection_ecs.iter().any(|c| {
                c.tcp_stream.peer_addr().unwrap() == client_connection.peer_addr().unwrap()
            }) {
                let entity = commands
                    .spawn(ClientConnection {
                        address: client_connection.peer_addr().unwrap(),
                        state: ConnectionState::Handshaking,
                        tcp_stream: client_connection.try_clone().unwrap(),
                    })
                    .id();
                let thread_connection = client_connection.try_clone().unwrap();
                thread::spawn(move || handle_client(thread_connection, entity));
            }
        }
    }

    for packet in SERVERBOUND_PACKETS.lock().unwrap().iter() {
        incoming_packet_mw.write(ServerboundNetworkPacketReceived {
            packet: packet.clone(),
        });
    }
    SERVERBOUND_PACKETS.lock().unwrap().clear();
}
