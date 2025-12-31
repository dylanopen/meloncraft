use std::io::{BufRead, BufReader};
use std::net::TcpStream;
use bevy::prelude::{Commands, MessageWriter, Query, Res, ResMut};
use meloncraft_client::connection::ClientConnection;
use meloncraft_client::connection_state::ConnectionState;
use meloncraft_protocol_types::deserialize;
use crate::connection_listener::ConnectionListener;
use crate::connected_clients::ConnectedClients;
use crate::packet::{IncomingNetworkPacket, IncomingNetworkPacketReceived};

pub fn receive_packets(
    mut commands: Commands,
    connection_listener: Res<ConnectionListener>,
    mut connected_clients: ResMut<ConnectedClients>,
    client_connections: Query<&ClientConnection>,
    mut incoming_packet_received_mw: MessageWriter<IncomingNetworkPacketReceived>,
) {
    if let Some(stream) = connection_listener.incoming().next() {
        let stream = stream.unwrap();
        println!(
            "Incoming connection from {:?}",
            &stream.peer_addr().unwrap()
        );

        let mut stream = stream.try_clone().unwrap();
        let mut buf_reader = BufReader::new(&mut stream);
        let mut raw_packet: Vec<u8> = buf_reader.fill_buf().unwrap().to_vec();

        let address = stream.peer_addr().unwrap();
        let client_entity = match connected_clients.0.get(&address) {
            Some(entity) => Some(*entity),
            None => None,
        };
        let new_connection_template = ClientConnection {
            address,
            state: ConnectionState::Handshaking,
        };
        let (client_entity, client_connection) = match client_entity {
            None => {
                (commands.spawn(new_connection_template.clone()).id(),
                 &new_connection_template)
                // cloning due to lifetimes: we can't use references here
            },
            Some(entity) => (entity, client_connections.get(entity).unwrap())
        };
        connected_clients.0.insert(address, client_entity);

        let length = deserialize::varint(&mut raw_packet).unwrap();
        let packet_id = deserialize::varint(&mut raw_packet).unwrap();
        let packet = IncomingNetworkPacket {
            client: client_entity,
            len: length,
            state: client_connection.state,
            id: packet_id,
            data: raw_packet,
        };
        dbg!(&packet);
        incoming_packet_received_mw.write(IncomingNetworkPacketReceived { packet });
    }
}

