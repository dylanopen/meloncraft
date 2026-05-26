use crate::ServerboundPacket;
use bevy::prelude::{Message, MessageReader, MessageWriter};
use crate::network_messages::ServerboundNetworkPacketReceived;
use core::fmt::Debug;

pub fn fwd<T: Message + ServerboundPacket + Debug>(
    mut all_packets: MessageReader<ServerboundNetworkPacketReceived>,
    mut packet_writer: MessageWriter<T>,
) {
    for network_packet in all_packets.read() {
        if let Some(packet) = T::deserialize(network_packet.packet.clone()) { // TODO: avoid cloning this
            packet_writer.write(packet);
        }
    }
}
