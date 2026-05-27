use core::hint::spin_loop;
use std::io::{ErrorKind, Write as _};

use bevy::ecs::message::MessageReader;
use bevy::ecs::system::Query;
use meloncraft_client::connection::ClientConnection;
use meloncraft_logger::{errorlog, tracelog};
use meloncraft_packets::network_messages::ClientboundNetworkPacketReceived;
use meloncraft_protocol_types::{ProtocolType as _, VarInt};

pub fn write_streams(
    mut clientbound_pr: MessageReader<ClientboundNetworkPacketReceived>,
    mut connections: Query<&mut ClientConnection>,
) {
    for packet in clientbound_pr.read() {
        let packet = packet.packet.clone();
        let Ok(mut connection) = connections.get_mut(packet.client) else {
            errorlog!(
                "Tried to send packet with ID {} to entity {}, which does not have a client connection.",
                packet.id,
                packet.client
            );
            continue;
        };

        let mut packet_body_bytes = VarInt(packet.id).net_serialize();
        packet_body_bytes.extend(packet.data);

        let packet_body_len = packet_body_bytes.len();

        let mut packet_bytes = VarInt(packet_body_len.try_into().unwrap()).net_serialize();
        packet_bytes.extend(packet_body_bytes);

        // Write in 4096-byte chunks, retrying each chunk on WouldBlock
        let mut failed = false;
        for chunk in packet_bytes.chunks(4096) {
            let mut offset = 0;
            while offset < chunk.len() {
                match connection.tcp_stream.write(chunk.get(offset..).unwrap()) {
                    Ok(0) => {
                        errorlog!(
                            "Write returned 0 bytes for packet {} to client {}. Connection may be closed.",
                            packet.id,
                            connection.address
                        );
                        failed = true;
                        break;
                    }
                    Ok(n) => {
                        offset += n;
                    }
                    Err(e) if e.kind() == ErrorKind::WouldBlock => {
                        // Buffer full — spin until ready
                        spin_loop();
                    }
                    Err(e) if e.kind() == ErrorKind::Interrupted => {
                        // EINTR — retry immediately
                    }
                    Err(e) => {
                        errorlog!(
                            "Failed to write packet {} to TcpStream for client {}. Error: {}",
                            packet.id,
                            connection.address,
                            e
                        );
                        failed = true;
                        break;
                    }
                }
            }
            if failed {
                break;
            }
        }

        if let Err(err) = connection.tcp_stream.flush() {
            errorlog!(
                "Failed to flush TcpStream for client {} when processing packet with id {}. Error: {}",
                connection.address,
                packet.id,
                err
            );
        }
        tracelog!(
            "Sent packet with ID {} to client with IP {}. Size: {}",
            packet.id,
            connection.address,
            packet_body_len
        );
    }
}
