use std::collections::HashMap;
use std::io::Write;
use std::net::TcpStream;
use bevy::prelude::{Entity, MessageReader, Query, Res};
use meloncraft_client::connection::ClientConnection;
use meloncraft_protocol_types::serialize;
use crate::packet::{OutgoingNetworkPacket, OutgoingNetworkPacketReceived};

fn send_packet(stream: &mut TcpStream, packet_id: i32, mut data: Vec<u8>) {
    let mut response: Vec<u8> = serialize::varint(packet_id);
    response.append(&mut data);
    let mut length_prefixed_response: Vec<u8> = serialize::varint(response.len() as i32);
    length_prefixed_response.append(&mut response);

    stream.write(length_prefixed_response.as_slice()).unwrap();
}

pub fn send_packets(
    mut outgoing_packet_received_mr: MessageReader<OutgoingNetworkPacketReceived>,
    mut client_connections: Query<&mut ClientConnection>,
) {
    let mut client_packets: HashMap<Entity, Vec<OutgoingNetworkPacket>> = HashMap::new();
    for packet_msg in outgoing_packet_received_mr.read() {
        let packet = packet_msg.packet.clone();
        match client_packets.get_mut(&packet.client) {
            Some(packets) => {
                packets.push(packet);
            },
            None => {
                client_packets.insert(packet.client, vec![packet]);
            }
        }
    }
    for (client, packets) in client_packets {
        let mut stream = client_connections.get_mut(client).unwrap().tcp_stream.try_clone().unwrap();
        for mut packet in packets {
            send_packet(&mut stream, packet.id, packet.data);
        }
    }
}