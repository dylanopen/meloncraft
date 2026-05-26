use crate::clientbound_packet::ClientboundPacket;
use crate::network_messages::ClientboundNetworkPacketReceived;
use bevy::prelude::{Message, MessageReader, MessageWriter};

pub fn fwd<T: Message + ClientboundPacket>(
    mut packet_reader: MessageReader<T>,
    mut packet_writer: MessageWriter<ClientboundNetworkPacketReceived>,
) {
    for packet in packet_reader.read() {
        packet_writer.write(ClientboundNetworkPacketReceived {
            packet: packet.serialize(),
        });
    }
}
