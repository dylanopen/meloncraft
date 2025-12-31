use bevy::prelude::{Message, MessageReader, MessageWriter};
use meloncraft_network::packet::IncomingNetworkPacketReceived;
use meloncraft_packets::IncomingPacket;

pub fn forward_packet<T: Message+IncomingPacket>(
    mut all_packets: MessageReader<IncomingNetworkPacketReceived>,
    mut packet_writer: MessageWriter<T>
) {
    for network_packet in all_packets.read() {
        if let Some(packet) = T::from_packet(&network_packet.packet) {
            packet_writer.write(packet);
        }
    }
}
