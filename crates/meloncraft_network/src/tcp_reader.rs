use std::io::{BufRead, BufReader};
use bevy::prelude::{Commands, MessageWriter, Query, Res, ResMut};
use meloncraft_client::connection::ClientConnection;
use meloncraft_client::connection_state::ConnectionState;
use meloncraft_protocol_types::deserialize;
use crate::connection_listener::ConnectionListener;
use crate::connected_clients::ConnectedClients;
use crate::packet::{IncomingNetworkPacket, IncomingNetworkPacketReceived};

pub fn receive_packets(
    mut commands: Commands,
    connection_listener: ResMut<ConnectionListener>,
    mut connected_clients: ResMut<ConnectedClients>,
    client_connections: Query<&ClientConnection>,
    mut incoming_packet_received_mw: MessageWriter<IncomingNetworkPacketReceived>,
) {
    dbg!();
    for client in client_connections.iter() {
        loop {
            let mut stream = client.tcp_stream.try_clone().unwrap();
            stream.set_nonblocking(true).unwrap();
            let mut buf_reader = BufReader::new(&mut stream);
            let mut raw_packet: Vec<u8> = match buf_reader.fill_buf() {
                Ok(buf) => buf.to_vec(),
                Err(_) => break,
            };
            if raw_packet.is_empty() {
                break;
            }

            let address = stream.peer_addr().unwrap();
            let client_entity = match connected_clients.0.get(&address) {
                Some(entity) => Some(*entity),
                None => None,
            };
            let (client_entity, client_connection_state) = match client_entity {
                None => {
                    (commands.spawn(ClientConnection {
                        tcp_stream: stream,
                        state: ConnectionState::Handshaking,
                        address,
                    }).id(),
                     &ConnectionState::Handshaking)
                },
                Some(entity) => (entity, &client_connections.get(entity).unwrap().state)
            };
            connected_clients.0.insert(address, client_entity);

            let length = deserialize::varint(&mut raw_packet).unwrap();
            let packet_id = deserialize::varint(&mut raw_packet).unwrap();
            let packet = IncomingNetworkPacket {
                client: client_entity,
                len: length,
                state: *client_connection_state,
                id: packet_id,
                data: raw_packet,
            };
            incoming_packet_received_mw.write(IncomingNetworkPacketReceived { packet });
        }
    }
    dbg!();
    connection_listener.0.set_nonblocking(true).unwrap();
    for stream in connection_listener.incoming() {
        let Ok(stream) = stream else { break; };

        let mut stream = stream.try_clone().unwrap();
        let mut buf_reader = BufReader::new(&mut stream);
        let mut raw_packet: Vec<u8> = buf_reader.fill_buf().unwrap().to_vec();

        let address = stream.peer_addr().unwrap();
        let client_entity = match connected_clients.0.get(&address) {
            Some(entity) => Some(*entity),
            None => None,
        };
        let (client_entity, client_connection_state) = match client_entity {
            None => {
                (commands.spawn(ClientConnection {
                    tcp_stream: stream,
                    state: ConnectionState::Handshaking,
                    address,
                }).id(),
                 &ConnectionState::Handshaking)
            },
            Some(entity) => (entity, &client_connections.get(entity).unwrap().state)
        };
        connected_clients.0.insert(address, client_entity);

        let length = deserialize::varint(&mut raw_packet).unwrap();
        let packet_id = deserialize::varint(&mut raw_packet).unwrap();
        let packet = IncomingNetworkPacket {
            client: client_entity,
            len: length,
            state: *client_connection_state,
            id: packet_id,
            data: raw_packet,
        };
        incoming_packet_received_mw.write(IncomingNetworkPacketReceived { packet });
    }
    dbg!();
}
