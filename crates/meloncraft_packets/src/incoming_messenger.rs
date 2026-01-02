use crate::IncomingPacket;
use bevy::prelude::{Message, MessageReader, MessageWriter, Query};
use meloncraft_client::connection::ClientConnection;
use meloncraft_network::INBOUND_PACKETS;
use meloncraft_network::packet::IncomingNetworkPacketReceived;

pub fn forward_incoming_packet<T: Message + IncomingPacket>(
    mut all_packets: MessageReader<IncomingNetworkPacketReceived>,
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
    mut incoming_network_packet_mw: MessageWriter<IncomingNetworkPacketReceived>,
) {
    let mut inbound_packets = INBOUND_PACKETS.lock().unwrap();
    let mut inbound_packet_msgs = Vec::new();
    while let Some(packet) = inbound_packets.pop() {
        inbound_packet_msgs.push(IncomingNetworkPacketReceived { packet });
    }
    inbound_packets.clear();
    incoming_network_packet_mw.write_batch(inbound_packet_msgs);
}
