use crate::INBOUND_PACKETS;
use crate::connection_listener::ConnectionListener;
use crate::packet::{IncomingNetworkPacket, IncomingNetworkPacketReceived};
use crate::tcp_reader::{handle_client, receive_new_clients};
use bevy::prelude::{Commands, Component, MessageWriter, Query, ResMut, Resource};
use lazy_static::lazy_static;
use meloncraft_client::connection::ClientConnection;
use std::sync::Mutex;
use std::thread;

lazy_static! {
    pub static ref CLIENT_CONNECTIONS: Mutex<Vec<ClientConnection>> = {
        let client_connections = Vec::new();
        Mutex::new(client_connections)
    };
}

#[derive(Component, Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct ClientConnectionId(pub usize);

impl ClientConnectionId {
    pub fn get(self) -> ClientConnection {
        CLIENT_CONNECTIONS.lock().unwrap()[self.0].clone()
    }
}

pub fn connection_manager(
    mut commands: Commands,
    client_connection_ecs: Query<&ClientConnectionId>,
    mut incoming_packet_mw: MessageWriter<IncomingNetworkPacketReceived>,
) {
    let client_connection_ecs_vec: Vec<&ClientConnectionId> =
        client_connection_ecs.iter().collect();

    if client_connection_ecs.iter().len() < CLIENT_CONNECTIONS.lock().unwrap().len() {
        let client_connections = CLIENT_CONNECTIONS.lock().unwrap().clone();
        for client_connection in client_connections.clone() {
            if let Some(pos) = client_connections
                .iter()
                .position(|c| c.address == client_connection.address)
            {
                if client_connection_ecs_vec.contains(&&ClientConnectionId(pos)) {
                    continue;
                }
                let entity = commands.spawn(ClientConnectionId(pos)).id();
                thread::spawn(move || {
                    handle_client(client_connection.tcp_stream.try_clone().unwrap(), entity)
                });
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
