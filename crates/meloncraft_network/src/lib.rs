use std::collections::HashMap;
use std::net::TcpListener;
use bevy::app::{App, Plugin, Update};
use crate::connection_listener::ConnectionListener;
use crate::connected_clients::ConnectedClients;
use crate::packet::IncomingNetworkPacketReceived;
use crate::tcp_reader::receive_packets;

pub mod packet;
pub mod connection_listener;
pub mod tcp_reader;
pub mod connected_clients;

pub struct MeloncraftNetworkPlugin;

impl Plugin for MeloncraftNetworkPlugin {
    fn build(&self, app: &mut App) {
        let connection_listener = ConnectionListener(TcpListener::bind("127.0.0.1:25565").unwrap());
        let connected_clients = ConnectedClients(HashMap::new());

        app.add_message::<IncomingNetworkPacketReceived>();
        app.insert_resource(connection_listener);
        app.insert_resource(connected_clients);
        app.add_systems(Update, receive_packets);
    }
}
