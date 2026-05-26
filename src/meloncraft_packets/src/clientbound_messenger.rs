use crate::clientbound_packet::ClientboundPacket;
use bevy::prelude::{Message, MessageReader, MessageWriter};
use crate::network_messages::ClientboundNetworkPacketReceived;

pub fn fwd<T: Message + ClientboundPacket>(
    mut packet_reader: MessageReader<T>,
    mut packet_writer: MessageWriter<ClientboundNetworkPacketReceived>,
) {
    for packet in packet_reader.read() {
        if let Some(network_packet) = packet.serialize() {
            packet_writer.write(ClientboundNetworkPacketReceived {
                packet: network_packet,
            });
        }
    }
}
