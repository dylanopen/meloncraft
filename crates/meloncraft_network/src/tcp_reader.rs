use crate::INBOUND_PACKETS;
use crate::connected_clients::ConnectedClients;
use crate::connection_listener::ConnectionListener;
use crate::connection_manager::CLIENT_CONNECTIONS;
use crate::packet::{IncomingNetworkPacket, IncomingNetworkPacketReceived};
use bevy::prelude::{Commands, Entity, MessageWriter, Query, Res, ResMut};
use meloncraft_client::connection::ClientConnection;
use meloncraft_client::connection_state::ConnectionState;
use meloncraft_protocol_types::deserialize;
use std::io::{BufRead, BufReader, Read};
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc::{Receiver, Sender, channel};
use std::thread;
use std::time::Duration;

pub struct IncomingTcpPacket {
    pub client: Entity,
    pub len: i32,
    pub id: i32,
    pub data: Vec<u8>,
}

pub fn handle_client(mut stream: TcpStream, entity: Entity) {
    loop {
        let mut packet_len = [0; 3];
        let Ok(amount_read) = stream.peek(&mut packet_len) else {
            break;
        };
        let len = deserialize::varint(&mut packet_len.to_vec()).unwrap();
        let mut raw_packet = vec![0; len as usize];
        if stream.read_exact(&mut raw_packet).is_err() {
            break; // no more packets to read
        };
        if raw_packet.is_empty() {
            break;
        }
        dbg!(&raw_packet);

        let address = stream.peer_addr().unwrap();

        let length = deserialize::varint(&mut raw_packet).unwrap();
        let packet_id = deserialize::varint(&mut raw_packet).unwrap();
        let state = CLIENT_CONNECTIONS
            .lock()
            .unwrap()
            .iter()
            .filter(|conn| conn.address == address)
            .collect::<Vec<_>>()
            .first()
            .unwrap()
            .state;
        let packet = IncomingNetworkPacket {
            client: entity,
            len: length,
            id: packet_id,
            data: raw_packet,
            state,
        };
        INBOUND_PACKETS.lock().unwrap().push(packet);
    }
}

pub fn receive_new_clients(tcp_listener: TcpListener) {
    loop {
        for stream in tcp_listener.incoming() {
            match stream {
                Ok(stream) => {
                    CLIENT_CONNECTIONS.lock().unwrap().push(ClientConnection {
                        address: stream.peer_addr().unwrap(),
                        tcp_stream: stream,
                        state: ConnectionState::Handshaking,
                    });
                }
                Err(e) => {
                    eprintln!("Failed to establish TCP connection with new client: {e}")
                }
            }
        }
    }
}

pub fn receive_packets_old(
    mut commands: Commands,
    connection_listener: ResMut<ConnectionListener>,
    mut connected_clients: ResMut<ConnectedClients>,
    client_connections: Query<&ClientConnection>,
    mut incoming_packet_received_mw: MessageWriter<IncomingNetworkPacketReceived>,
) {
    for client in client_connections.iter() {
        loop {
            let mut stream = client.tcp_stream.try_clone().unwrap();
            stream.set_nonblocking(true).unwrap();
            let mut packet_len = [0; 3];
            let Ok(amount_read) = stream.peek(&mut packet_len) else {
                break;
            };
            let len = deserialize::varint(&mut packet_len.to_vec()).unwrap();
            let mut raw_packet = vec![0; len as usize];
            if let Err(_) = stream.read_exact(&mut raw_packet) {
                break; // no more packets to read
            };
            if raw_packet.is_empty() {
                break;
            }
            dbg!(&raw_packet);

            let address = stream.peer_addr().unwrap();
            let client_entity = match connected_clients.0.get(&address) {
                Some(entity) => Some(*entity),
                None => None,
            };
            let (client_entity, client_connection_state) = match client_entity {
                None => (
                    commands
                        .spawn(ClientConnection {
                            tcp_stream: stream,
                            state: ConnectionState::Handshaking,
                            address,
                        })
                        .id(),
                    &ConnectionState::Handshaking,
                ),
                Some(entity) => (entity, &client_connections.get(entity).unwrap().state),
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
    connection_listener.0.set_nonblocking(true).unwrap();
    for stream in connection_listener.incoming() {
        let Ok(stream) = stream else {
            break;
        };

        let mut stream = stream.try_clone().unwrap();
        let mut buf_reader = BufReader::new(&mut stream);
        let mut raw_packet: Vec<u8> = buf_reader.fill_buf().unwrap().to_vec();

        let address = stream.peer_addr().unwrap();
        let client_entity = match connected_clients.0.get(&address) {
            Some(entity) => Some(*entity),
            None => None,
        };
        let (client_entity, client_connection_state) = match client_entity {
            None => (
                commands
                    .spawn(ClientConnection {
                        tcp_stream: stream,
                        state: ConnectionState::Handshaking,
                        address,
                    })
                    .id(),
                &ConnectionState::Handshaking,
            ),
            Some(entity) => (entity, &client_connections.get(entity).unwrap().state),
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
