use crate::ServerboundPacket;
use bevy::prelude::{Message, MessageReader, MessageWriter, Query};
use meloncraft_client::connection::ClientConnection;
use meloncraft_network::SERVERBOUND_PACKETS;
use meloncraft_network::packet::ServerboundNetworkPacketReceived;
use std::fmt::Debug;

pub fn fwd<T: Message + ServerboundPacket + Debug>(
    mut all_packets: MessageReader<ServerboundNetworkPacketReceived>,
    mut packet_writer: MessageWriter<T>,
    client_connections: Query<&ClientConnection>,
) {
    for network_packet in all_packets.read() {
        if let Some(packet) = T::from_packet(&network_packet.packet, &client_connections) {
            packet_writer.write(packet);
        }
    }
}

pub fn read_new_packets(
    mut serverbound_network_packet_mw: MessageWriter<ServerboundNetworkPacketReceived>,
) {
    let mut serverbound_packets = SERVERBOUND_PACKETS.lock().unwrap();
    let mut inbound_packet_msgs = Vec::new();
    while let Some(packet) = serverbound_packets.pop() {
        inbound_packet_msgs.push(ServerboundNetworkPacketReceived { packet });
    }
    serverbound_packets.clear();
    serverbound_network_packet_mw.write_batch(inbound_packet_msgs);
}
