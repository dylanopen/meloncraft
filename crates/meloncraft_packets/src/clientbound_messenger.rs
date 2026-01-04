use crate::clientbound_packet::ClientboundPacket;
use bevy::prelude::{Message, MessageReader, MessageWriter};
use meloncraft_network::packet::ClientboundNetworkPacketReceived;

pub fn forward_clientbound_packet<T: Message + ClientboundPacket>(
    mut packet_reader: MessageReader<T>,
    mut packet_writer: MessageWriter<ClientboundNetworkPacketReceived>,
) {
    for packet in packet_reader.read() {
        if let Some(network_packet) = packet.to_packet() {
            packet_writer.write(ClientboundNetworkPacketReceived {
                packet: network_packet,
            });
        }
    }
}
