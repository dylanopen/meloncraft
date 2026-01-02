use crate::outgoing_packet::OutgoingPacket;
use bevy::prelude::{Message, MessageReader, MessageWriter};
use meloncraft_network::packet::OutgoingNetworkPacketReceived;

pub fn forward_outgoing_packet<T: Message + OutgoingPacket>(
    mut packet_reader: MessageReader<T>,
    mut packet_writer: MessageWriter<OutgoingNetworkPacketReceived>,
) {
    for packet in packet_reader.read() {
        if let Some(network_packet) = packet.to_packet() {
            packet_writer.write(OutgoingNetworkPacketReceived {
                packet: network_packet,
            });
        }
    }
}
