use crate::INBOUND_PACKETS;
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
    let mut iters = 0;
    let mut stream = stream.try_clone().unwrap();
    stream
        .set_read_timeout(Some(Duration::from_millis(15000)))
        .unwrap();
    let address = stream.peer_addr().unwrap();
    let mut buf_reader = BufReader::new(&mut stream);
    loop {
        if iters == 1 {
            sleep(Duration::from_millis(12)); // packets are messages - unfortunately necessary to avoid race condition
        }
        iters += 1;
        let mut length_bytes = Vec::new();
        loop {
            let mut single_byte_buf = vec![0u8; 1];
            if buf_reader.read_exact(&mut single_byte_buf).is_err() {
                break; // no more packets to read
            }
            length_bytes.push(single_byte_buf[0]);
            if single_byte_buf[0] & VARINT_CONTINUE_BIT == 0 {
                break; // no more data in varint to read
            }
        }
        let length = deserialize::varint(&mut length_bytes).unwrap();
        let mut raw_packet: Vec<u8> = vec![0; length as usize];
        if buf_reader.read_exact(&mut raw_packet).is_err() {
            println!("Client {entity} disconnected due to failing to stream packets");
            break;
        }
        if raw_packet.is_empty() {
            break;
        }

        let packet_id = deserialize::varint(&mut raw_packet).unwrap();
        let packet = IncomingNetworkPacket {
            client: entity,
            len: length,
            id: packet_id,
            data: raw_packet,
        };
        INBOUND_PACKETS.lock().unwrap().push(packet);
    }
}

pub fn receive_new_clients(tcp_listener: TcpListener) {
    loop {
        for stream in tcp_listener.incoming() {
            match stream {
                Ok(stream) => {
                    CLIENT_CONNECTIONS.lock().unwrap().push(stream);
                }
                Err(e) => {
                    eprintln!("Failed to establish TCP connection with new client: {e}")
                }
            }
        }
    }
}
