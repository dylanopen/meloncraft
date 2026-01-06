use crate::ServerboundPacket;
use bevy::prelude::{Message, MessageReader, MessageWriter};
use meloncraft_network::SERVERBOUND_PACKETS;
use meloncraft_network::packet::ServerboundNetworkPacketReceived;
use std::fmt::Debug;

pub fn fwd<T: Message + ServerboundPacket + Debug>(
    mut all_packets: MessageReader<ServerboundNetworkPacketReceived>,
    mut packet_writer: MessageWriter<T>,
) {
    for network_packet in all_packets.read() {
        if let Some(packet) = T::from_packet(&network_packet.packet) {
            packet_writer.write(packet);
        }
    }
}

