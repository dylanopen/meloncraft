use std::io::Read as _;

use bevy::ecs::entity::Entity;
use bevy::ecs::message::MessageWriter;
use bevy::ecs::system::Query;
use meloncraft_client::connection::ClientConnection;
use meloncraft_client::connection_state::ConnectionState;
use meloncraft_logger::{errorlog, tracelog};
use meloncraft_packets::network_messages::{
    ServerboundNetworkPacket, ServerboundNetworkPacketReceived,
};
use meloncraft_protocol_types::{ProtocolType as _, VarInt};

const VARINT_CONTINUE_BIT: u8 = 0b1000_0000;

pub fn read_streams(
    connection_q: Query<(Entity, &mut ClientConnection)>,
    mut serverbound_pw: MessageWriter<ServerboundNetworkPacketReceived>,
) {
    for (client, mut connection) in connection_q {
        let Ok(mut stream) = connection.tcp_stream.try_clone() else {
            errorlog!("Failed to clone TcpStream for a client.");
            // TODO: maybe we should remove the client's connection?
            continue;
        };

        'packets: loop {
            // First, check if the client's connection state is `Handshaking`. If so, we only
            // process the packet if it's the first packet the client has ever sent.
            // Otherwise, we ignore it, because we want the server to process the handshake before
            // we try to read the next packet (as otherwise we'll read the packet in the wrong
            // state, causing a crash).
            if connection.serverbound_packets_processed != 0
                && connection.state == ConnectionState::Handshaking
            {
                break;
            }

            // Then, we get the length of the total packet, which we'll set as the packet_bytes size
            // later. This is a VarInt, and as we don't have it as a byte-array yet (but a stream that
            // we don't know the size of), we'll have to check the continue bits here instead of using
            // the VarInt's net_deserialize() trait impl method.
            let mut length_bytes = Vec::new();
            loop {
                let mut single_byte_buf = vec![0_u8; 1];
                if stream.read_exact(&mut single_byte_buf).is_err() {
                    break 'packets; // no more packets to read
                }
                length_bytes.push(*single_byte_buf.first().unwrap());
                if single_byte_buf.first().unwrap() & VARINT_CONTINUE_BIT == 0 {
                    break; // no more data in varint to read
                }
            }

            // Now we need to actually deserialize the byte array, representing the length of the packet
            // as a VarInt. We can finally use our deserialize function:
            let Ok(length) = VarInt::net_deserialize(&mut length_bytes) else {
                continue; // no more packets left to read for this client's connection.
            };
            let length = length.0;
            let Ok(length_usize) = length.try_into() else {
                errorlog!(
                    "Client sent a packet length which doesn't convert from an `i32` into a `usize` (are you on a 32-bit system and is the client sending incorrect packets?) Packet length sent was {}, sent by client {}",
                    length,
                    connection.address
                );
                // TODO: maybe we should remove the client's connection?
                continue;
            };

            // Now, using that packet size we just decoded, we'll read the rest of the panic, printing
            // an error if the clientt doesn't send enough data:
            let mut packet_bytes = vec![0_u8; length_usize];
            if let Err(err) = stream.read_exact(&mut packet_bytes) {
                errorlog!(
                    "Couldn't read {length} bytes from client {}: maybe the client sent an incorrect packet? We'll ignore the packet for now. Error: {err}",
                    connection.address
                );
                // TODO: maybe we should remove the client's connection?
                continue;
            }

            // Finally, drain (read and remove) the packet ID from the start of the packet.
            // This is also a `VarInt`.
            let Ok(packet_id) = VarInt::net_deserialize(&mut packet_bytes) else {
                errorlog!(
                    "Couldn't read the packet ID sent by client {}. Packet bytes: {:x?}",
                    connection.address,
                    packet_bytes
                );
                // TODO: maybe we should remove the client's connection?
                continue;
            };
            let packet_id = packet_id.0;

            tracelog!(
                "Packet {} received from {}. Bytes: {:x?}",
                packet_id,
                connection.address,
                &packet_bytes
            );

            serverbound_pw.write(ServerboundNetworkPacketReceived {
                packet: ServerboundNetworkPacket {
                    client,
                    id: packet_id,
                    len: length,
                    data: packet_bytes,
                    state: connection.state,
                },
            });

            connection.serverbound_packets_processed += 1;
        }
    }
}
