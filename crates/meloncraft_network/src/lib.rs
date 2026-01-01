use std::collections::HashMap;
use std::net::{TcpListener, TcpStream};
use bevy::app::{App, Plugin, Update};
use crate::connection_listener::ConnectionListener;
use crate::connected_clients::ConnectedClients;
use crate::packet::{IncomingNetworkPacketReceived, OutgoingNetworkPacketReceived};
use crate::tcp_reader::receive_packets;
use crate::tcp_writer::send_packets;

pub mod packet;
pub mod connection_listener;
pub mod tcp_reader;
pub mod connected_clients;
mod tcp_writer;

pub struct MeloncraftNetworkPlugin;

impl Plugin for MeloncraftNetworkPlugin {
    fn build(&self, app: &mut App) {
        let tcp_stream = ConnectionListener(TcpListener::bind("127.0.0.1:25565").unwrap());
        let connected_clients = ConnectedClients(HashMap::new());

        app.add_message::<IncomingNetworkPacketReceived>();
        app.add_message::<OutgoingNetworkPacketReceived>();
        app.insert_resource(tcp_stream);
        app.insert_resource(connected_clients);
        app.add_systems(Update, receive_packets);
        app.add_systems(Update, send_packets);
    }
}
