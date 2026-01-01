use crate::INBOUND_PACKETS;
use crate::connected_clients::ConnectedClients;
use crate::connection_listener::ConnectionListener;
use crate::packet::{IncomingNetworkPacket, IncomingNetworkPacketReceived};
use bevy::prelude::{Commands, Entity, MessageWriter, Query, Res, ResMut};
use meloncraft_client::connection::{CLIENT_CONNECTIONS, ClientConnection};
use meloncraft_client::connection_state::ConnectionState;
use meloncraft_protocol_types::deserialize;
use std::io::{BufRead, BufReader, Read};
use std::net::{TcpListener, TcpStream};
use std::thread::sleep;
use std::time::Duration;

pub struct IncomingTcpPacket {
    pub client: Entity,
    pub len: i32,
    pub id: i32,
    pub data: Vec<u8>,
}

const VARINT_CONTINUE_BIT: u8 = 0b10000000;

pub fn handle_client(stream: TcpStream, entity: Entity) {
    let mut stream = stream.try_clone().unwrap();
    stream
        .set_read_timeout(Some(Duration::from_millis(1000)))
        .unwrap();
    let address = stream.peer_addr().unwrap();
    let mut buf_reader = BufReader::new(&mut stream);
    loop {
        let mut length_bytes = Vec::new();
        loop {
            let mut single_byte_buf = vec![0u8; 1];
            if buf_reader.read_exact(&mut single_byte_buf).is_err() {
                break; // no more packets to read
            }
            if single_byte_buf[0] & VARINT_CONTINUE_BIT == 0 {
                break; // no more data in varint to read
            }
            length_bytes.push(single_byte_buf[0]);
        }
        dbg!(&length_bytes);
        let length = deserialize::varint(&mut length_bytes).unwrap().0;
        let mut raw_packet: Vec<u8> = vec![0; length as usize];
        println!("receiving packet from client {entity}");
        buf_reader.read_to_end(&mut raw_packet).unwrap();
        dbg!(&raw_packet);
        // client sent incomplete packets, disconnect them

        if raw_packet.is_empty() {
            //break;
        }

        /*
        let mut packet_len = [0; 3];
        let Ok(_) = stream.peek(&mut packet_len) else {
            sleep(Duration::from_millis(1));
            dbg!("peeking");
            continue;
        };
        let len = deserialize::varint(&mut packet_len.to_vec()).unwrap();
        if len == 0 {
            continue; // client disconnected
        }
        let mut raw_packet = vec![0; (len + 3) as usize];
        if stream.read_exact(&mut raw_packet).is_err() {
            // no more packets to read for now
            sleep(Duration::from_millis(1));
            continue;
        };
        if raw_packet.is_empty() {
            sleep(Duration::from_millis(1));
            println!("Client {entity} disconnected");
            continue;
        }
        println!("packet sent from client {entity}");
         */

        let (length, length_bytes) = deserialize::varint(&mut raw_packet).unwrap();
        dbg!(length, length_bytes);
        buf_reader.consume((length + length_bytes) as usize);
        dbg!(&raw_packet);

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
            id: packet_id.0,
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
                        tcp_stream: stream.try_clone().unwrap(),
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
