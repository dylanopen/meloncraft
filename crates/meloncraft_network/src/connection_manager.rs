use crate::INBOUND_PACKETS;
use crate::connection_listener::ConnectionListener;
use crate::packet::{IncomingNetworkPacket, IncomingNetworkPacketReceived};
use crate::tcp_reader::{handle_client, receive_new_clients};
use bevy::prelude::{Commands, Component, MessageWriter, Query, ResMut, Resource};
use lazy_static::lazy_static;
use meloncraft_client::connection::{CLIENT_CONNECTIONS, ClientConnection};
use meloncraft_client::connection_state::ConnectionState;
use std::sync::Mutex;
use std::thread;

pub fn connection_manager(
    mut commands: Commands,
    client_connection_ecs: Query<&ClientConnection>,
    mut incoming_packet_mw: MessageWriter<IncomingNetworkPacketReceived>,
) {
    let client_connection_ecs_vec: Vec<&ClientConnection> = client_connection_ecs.iter().collect();

    if client_connection_ecs.iter().len() < CLIENT_CONNECTIONS.lock().unwrap().len() {
        let client_connections = CLIENT_CONNECTIONS.lock().unwrap();
        for client_connection in client_connections.iter() {
            if let None = client_connection_ecs.iter().position(|c| {
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

    for packet in INBOUND_PACKETS.lock().unwrap().iter() {
        incoming_packet_mw.write(IncomingNetworkPacketReceived {
            packet: packet.clone(),
        });
    }
    INBOUND_PACKETS.lock().unwrap().clear();
}
