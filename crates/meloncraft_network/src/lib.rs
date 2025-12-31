use std::collections::HashMap;
use std::net::TcpListener;
use bevy::app::{App, Plugin, Update};
use crate::connection_listener::ConnectionListener;
use crate::connection_state::ConnectionStates;
use crate::packet::IncomingPacketReceived;
use crate::tcp_reader::receive_packets;

pub mod connection_state;
pub mod packet;
pub mod connection_listener;
pub mod tcp_reader;

pub struct MeloncraftNetworkPlugin;

impl Plugin for MeloncraftNetworkPlugin {
    fn build(&self, app: &mut App) {
        let connection_listener = ConnectionListener(TcpListener::bind("127.0.0.1:25565").unwrap());
        let connection_states = ConnectionStates(HashMap::new());

        app.add_message::<IncomingPacketReceived>();
        app.insert_resource(connection_listener);
        app.insert_resource(connection_states);
        app.add_systems(Update, receive_packets);
    }
}
