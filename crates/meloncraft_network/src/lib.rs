use crate::connection_listener::ConnectionListener;
use crate::connection_manager::connection_manager;
use crate::packet::{
    ClientboundNetworkPacket, ClientboundNetworkPacketReceived, IncomingNetworkPacket,
    IncomingNetworkPacketReceived,
};
use crate::tcp_reader::receive_new_clients;
use crate::tcp_writer::send_packets;
use bevy::app::{App, Plugin, Update};
use lazy_static::lazy_static;
use std::net::TcpListener;
use std::sync::Mutex;
use std::thread;

pub mod connection_listener;
mod connection_manager;
pub mod packet;
pub mod tcp_reader;
mod tcp_writer;

lazy_static! {
    pub static ref INBOUND_PACKETS: Mutex<Vec<IncomingNetworkPacket>> = {
        let packets = Vec::new();
        Mutex::new(packets)
    };
    pub static ref CLIENTBOUND_PACKETS: Mutex<Vec<ClientboundNetworkPacket>> = {
        let packets = Vec::new();
        Mutex::new(packets)
    };
}

pub struct MeloncraftNetworkPlugin;

impl Plugin for MeloncraftNetworkPlugin {
    fn build(&self, app: &mut App) {
        let connection_listener = ConnectionListener(TcpListener::bind("127.0.0.1:25565").unwrap());

        let tcp_listener = connection_listener.0.try_clone().unwrap();
        thread::spawn(move || receive_new_clients(tcp_listener.try_clone().unwrap()));

        app.add_message::<IncomingNetworkPacketReceived>();
        app.add_message::<ClientboundNetworkPacketReceived>();
        app.insert_resource(connection_listener);
        app.add_systems(Update, send_packets);
        app.add_systems(Update, connection_manager);
    }
}
