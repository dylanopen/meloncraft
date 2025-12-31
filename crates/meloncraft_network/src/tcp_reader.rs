use std::io::{BufRead, BufReader};
use std::net::TcpStream;
use bevy::prelude::{MessageWriter, Res, ResMut};
use meloncraft_protocol_types::deserialize;
use crate::connection_listener::ConnectionListener;
use crate::connection_state::{ConnectionState, ConnectionStates};
use crate::packet::{IncomingPacket, IncomingPacketReceived};

pub fn receive_packets(
    connection_listener: Res<ConnectionListener>,
    mut connection_states: ResMut<ConnectionStates>,
    mut incoming_packet_received_mw: MessageWriter<IncomingPacketReceived>,
) {
    while let Some(stream) = connection_listener.incoming().next() {
        let stream = stream.unwrap();
        println!(
            "Incoming connection from {:?}",
            &stream.peer_addr().unwrap()
        );

        loop {
            if read_packet(
                &stream,
                &mut connection_states,
                &mut incoming_packet_received_mw,
            ) {
                connection_states.remove(&stream.peer_addr().unwrap());
                break;
            }
        }
    }
}

fn read_packet(
    stream: &TcpStream,
    connection_states: &mut ConnectionStates,
    incoming_packet_received_mw: &mut MessageWriter<IncomingPacketReceived>,
) -> bool {
    let mut stream = stream.try_clone().unwrap();
    let mut buf_reader = BufReader::new(&mut stream);
    let mut raw_packet: Vec<u8> = buf_reader.fill_buf().unwrap().to_vec();
    println!("Received incoming packet: {:?}", raw_packet);
    let connection_state = connection_states
        .entry(stream.peer_addr().unwrap())
        .or_insert(ConnectionState::Handshaking);
    let length = deserialize::varint(&mut raw_packet).unwrap();
    let packet_id = deserialize::varint(&mut raw_packet).unwrap();
    let packet = IncomingPacket {
        len: length,
        state: *connection_state,
        id: packet_id,
        data: raw_packet,
    };
    incoming_packet_received_mw.write(IncomingPacketReceived { packet });
    true
}

